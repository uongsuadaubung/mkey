# MKey Rust Engine — Tài liệu Kỹ thuật & Kiến trúc Hệ thống

Tài liệu này mô tả chi tiết kiến trúc bên trong, luồng xử lý dữ liệu, máy trạng thái ngữ âm học và các thuật toán cốt lõi của **MKey Engine** được viết bằng Rust.

---

## 1. Kiến trúc Tổng thể (System Architecture)

Hệ thống được thiết kế theo mô hình **Pipeline phân tầng hướng sự kiện (Event-driven Layered Pipeline)**, tách biệt hoàn toàn giữa tầng tương tác phần cứng hệ điều hành và tầng logic xử lý tiếng Việt:

```mermaid
flowchart TD
    subgraph OS_Layer ["1. Tầng Hệ điều hành (Platform Layer)"]
        KBD_Hook["Low-Level Keyboard Hook (src/platform/win32/mod.rs)"]
        MOUSE_Hook["Low-Level Mouse Hook (src/platform/win32/mod.rs)"]
        SendInput["Win32 SendInput (src/platform/win32/injector.rs)"]
        AppDetect["App Detection (src/platform/win32/app_detect.rs)"]
        Registry["Windows Registry Run Key (src/platform/registry.rs)"]
    end

    subgraph Engine_Layer ["2. Tầng Điều phối (Engine Coordinator Layer)"]
        VE["VietnameseEngine (src/engine/mod.rs)"]
        ConfigStore["ConfigStore (src/engine/config_store.rs)"]
        MacroTbl["MacroTable (Bảng gõ tắt bảo toàn Casing)"]
        Hist["WordHistory (Ngăn xếp phục hồi qua phím cách)"]
        SpellingGuard["Spelling Checker & Restore on Wrong Spelling"]
    end

    subgraph Buffer_Layer ["3. Tầng Bộ đệm (Typing Buffer Layer)"]
        TB["TypingBuffer (src/engine/buffer.rs)"]
        RawKeys["raw_keys: Vec<RawKey> (Single Source of Truth)"]
        Eval["evaluate_keys() (Pure Projection)"]
    end

    subgraph Linguistic_Layer ["4. Tầng Ngữ âm học (Linguistic State Machine)"]
        SM["SyllableState (src/vietnamese/state.rs)"]
        OnsetNode["Onset (src/vietnamese/onset.rs)"]
        NucleusNode["Nucleus & Tone (src/vietnamese/nucleus.rs)"]
        CodaNode["Coda (src/vietnamese/coda.rs)"]
        PassNode["Passthrough (Ký tự thô / Tiếng Anh)"]
    end

    KBD_Hook -->|"on_key() / on_backspace()"| VE
    MOUSE_Hook -->|"reset()"| VE
    VE --> TB
    TB --> RawKeys
    RawKeys --> Eval
    Eval --> SM
    SM --> OnsetNode & NucleusNode & CodaNode & PassNode
    VE --> MacroTbl
    VE --> Hist
    VE --> SpellingGuard
    VE --> ConfigStore
    VE -->|"EngineAction (Replace / Passthrough)"| SendInput
```

---

## 2. Các Phân hệ Cốt lõi (Core Subsystems)

### 2.1. `TypingBuffer` — Single Source of Truth & Pure Projection
- **File:** `src/engine/buffer.rs`
- **Nguyên lý Single Source of Truth (SSOT):**
  - Mọi thao tác gõ trong phiên từ hiện tại đều được ghi nhận vào mảng `raw_keys: Vec<RawKey>`.
  - Trạng thái âm tiết `state: SyllableState` và độ dài ký tự hiển thị `emitted_len` được tính toán độc lập và nhất quán thông qua hàm chiếu thuần túy:
    $$\text{evaluate\_keys}(\text{raw\_keys}, \text{config}) \rightarrow (\text{state}, \text{rendered}, \text{emitted\_len}, \dots)$$
- **Thuật toán Backspace tất định (`handle_backspace`):**
  - Không dựa vào giả định số lượng ký tự xóa của hệ điều hành.
  - Khi nhận sự kiện Backspace, buffer tính `target_len = emitted_len - 1` và thực hiện `raw_keys.pop()` kèm tái chiếu `evaluate_keys` liên tục cho đến khi độ dài hiển thị giảm chính xác 1 đơn vị.
  - Đối với từ ở chế độ thô (`is_passthrough`), Backspace giảm trực tiếp từng ký tự một ($1:1$ với màn hình).
- **Phát hiện Biên tự động (Automatic Boundary Splitting):**
  - **CamelCase Boundary:** Khi gặp một chữ cái viết HOA xen giữa các chữ cái viết thường (ví dụ: `userName`), buffer tự động chốt từ `user` và bắt đầu âm tiết mới với `Name`.
  - **Closed Syllable Boundary:** Khi âm tiết hiện tại đã đạt cấu trúc đóng (ví dụ: các nguyên âm không thể nhận phụ âm cuối như `ôi`, `ay`, `eo`), nếu người dùng gõ thêm một phụ âm không phải phím dấu Telex, buffer tự động ngắt từ cũ và mở từ mới.

---

### 2.2. `SyllableState` — Máy trạng thái Ngữ âm học Tiếng Việt (Micro-Modules)
- **Files:**
  - `src/vietnamese/state.rs`: Điều phối chu trình chuyển dịch trạng thái âm tiết.
  - `src/vietnamese/onset.rs`: Quản lý phụ âm đầu (`OnsetState`) và biến đổi chữ `đ`.
  - `src/vietnamese/nucleus.rs`: Quản lý nguyên âm (`NucleusState`), quy tắc đặt dấu thanh mới/cũ, dấu mũ (`^`), dấu móc (`w`).
  - `src/vietnamese/coda.rs`: Quản lý phụ âm cuối (`CodaState`), quy tắc gõ dấu xuyên phụ âm cuối và chặn lỗi chính tả (Stop Coda Guard).
  - `src/vietnamese/modifier.rs`: Ánh xạ biến âm thống nhất cho cả Telex và VNI.
- Máy trạng thái quản lý chu trình chuyển dịch của một âm tiết:
  $$\text{Empty} \longrightarrow \text{Onset} \longrightarrow \text{Nucleus} \longrightarrow \text{Coda}$$
  $$\downarrow \qquad\qquad\quad \downarrow \qquad\qquad\quad \downarrow \qquad\qquad\quad \downarrow$$
  $$\text{Passthrough} \quad \text{Passthrough} \quad \text{Passthrough} \quad \text{Passthrough}$$

#### Quy tắc chuyển dịch:
1. **Empty (Rỗng):**
   - Nhận phụ âm hợp lệ $\rightarrow$ `Onset`.
   - Nhận nguyên âm hợp lệ $\rightarrow$ `Nucleus` (không có phụ âm đầu).
   - Nhận ngoặc vuông `[` $\rightarrow$ `ư`, `]` $\rightarrow$ `ơ` (tính năng `bracket_w`).
2. **Onset (Phụ âm đầu - `onset.rs`):**
   - Hỗ trợ các tổ hợp phụ âm đơn và ghép: `b, c, d, đ, g, gh, h, k, l, m, n, ng, ngh, nh, p, ph, q, r, s, t, th, tr, v, x`.
   - Nhận nguyên âm $\rightarrow$ chuyển tiếp sang `Nucleus`.
   - Nhận phím tạo chữ `đ` (`d` thứ 2 trong Telex hoặc `9` trong VNI) $\rightarrow$ đổi `d` $\leftrightarrow$ `đ`.
3. **Nucleus (Nguyên âm & Dấu thanh - `nucleus.rs`):**
   - Quản lý các tổ hợp nguyên âm đơn, đôi, ba (`a, ă, â, e, ê, i, o, ô, ơ, u, ư, y`, `ia, iê, oa, oe, uô, ươ...`).
   - **Tự do đặt dấu thanh (Free Tone Placement):** Cho phép đặt dấu ở bất kỳ thời điểm nào:
     - Gõ dấu ngay sau nguyên âm đầu: `v-i-e-j-e-t` $\rightarrow$ `việt`.
     - Gõ dấu ở cuối từ: `v-i-e-e-t-j` $\rightarrow$ `việt`.
   - **Tổ hợp dấu mũ (`toggle_circumflex`):** Nhân đôi nguyên âm (`aa` $\rightarrow$ `â`, `ee` $\rightarrow$ `ê`, `oo` $\rightarrow$ `ô`). Gõ lần thứ 3 sẽ hoàn tác về dạng chữ đôi thô (`tee`, `loo`).
   - **Tổ hợp dấu móc (`apply_horn` / `revert_horn`):** Phím `w` gắn móc cho `u, o` $\rightarrow$ `ư, ơ` và trăng cho `a` $\rightarrow$ `ă`. Gõ `w` lần nữa sẽ hoàn tác.
4. **Coda (Phụ âm cuối - `coda.rs`):**
   - Tiếp nhận các phụ âm cuối: `c, ch, m, n, ng, nh, p, t`.
   - **Free Mark dấu mũ xuyên phụ âm cuối (`a, e, o`):** Cho phép đặt dấu mũ sau khi đã gõ phụ âm cuối (ví dụ: `v-a-n-a-s` $\rightarrow$ `vấn`, `c-o-n-g-o` $\rightarrow$ `công`, `d-d-e-n-e` $\rightarrow$ `đên`).
   - **Stop Coda Guard (Chặn tạo từ sai ngữ âm):** Các phụ âm cuối tắc vô thanh (`c, ch, p, t`) khi chưa có dấu thanh thì không thể nhận thêm dấu mũ (nhờ đó các từ tiếng Anh như `data` không bao giờ bị biến thành `dât`).
   - Áp dụng quy tắc thanh điệu ngữ âm học: Phụ âm cuối tắc vô thanh (`c, ch, p, t`) chỉ chấp nhận thanh **Sắc** hoặc **Nặng**; nếu gõ thanh khác, hệ thống tự động hủy dấu hoặc đẩy ra ký tự thô.
5. **Passthrough (Ký tự thô):**
   - Được kích hoạt khi tổ hợp phím không tuân thủ ngữ âm tiếng Việt hoặc từ đã bị hủy dấu. Toàn bộ ký tự sau đó sẽ được xuất nguyên bản không can thiệp.

---

### 2.3. `WordHistory` — Ngăn xếp Phục hồi qua Phím cách
- **File:** `src/engine/history.rs`
- Cung cấp tính năng **"Nhớ từ đã gõ qua phím cách"**: Khi người dùng đã gõ xong từ và bấm dấu cách, nếu bấm Backspace xóa dấu cách đó thì từ cũ được nạp lại vào buffer để tiếp tục sửa.
- **Cấu trúc dữ liệu:**
  ```rust
  pub struct CommittedWord {
      pub raw_keys: Vec<RawKey>,
      pub emitted_text: String,
      pub is_raw_restored: bool,
      pub spaces_after: usize,
  }
  ```
- **Quy tắc an toàn tuyệt đối:**
  1. Chỉ phục hồi từ khi `spaces_after == 1` (đúng một dấu cách ngăn giữa con trỏ và từ trước).
  2. Khi gặp phím xuống dòng (`Enter`: `\r`, `\n`), dấu câu (`.`, `,`, `!`, `?`...), click chuột hoặc phím điều hướng $\rightarrow$ gọi `WordHistory::clear()`. Điều này triệt tiêu hoàn toàn lỗi "lội ngược dòng" gây biến dạng từ ngữ.

---

### 2.4. `Spelling Guard` — Tự động Khôi phục Từ Tiếng Anh (`restore_on_wrong_spelling`)
- **File:** `src/engine/mod.rs` & `src/vietnamese/spelling.rs`
- Khắc phục nhược điểm kinh điển của bộ gõ tiếng Việt khi người dùng gõ từ tiếng Anh chứa các phím dấu Telex (`s, f, r, x, j, w`):
  - `fix` $\rightarrow$ tạm thời hiển thị `fĩ` trên màn hình.
  - Khi người dùng nhấn phím ngắt từ (`Space`): Engine phân tích âm tiết. Vì tiếng Việt không tồn tại âm tiết `fĩ` (phụ âm đầu `f` không kết hợp dấu ngã `ĩ`), hệ thống tự động phát ra lệnh `Replace`: lùi lại 2 ký tự xóa `fĩ` và xuất trả `fix `.
  - Hỗ trợ toàn diện các từ phổ biến: `fix`, `fox`, `fax`, `first`, `server`, `user`, `word`, `win`...
  - Không làm ảnh hưởng đến các từ tiếng Việt hợp lệ có kết thúc bằng phím `x` như: `mã` (`max`), `sĩ` (`six`), `bõ` (`box`), `sẽ` (`sex`), `tã` (`tax`).

---

### 2.5. `MacroTable` — Bảng Gõ tắt Bảo toàn Dạng Chữ
- **File:** `src/engine/macro_table.rs`
- Tự động thay thế từ viết tắt khi nhấn phím cách.
- **Bảo toàn Casing thông minh:**
  - Viết thường: `vn` $\rightarrow$ `việt nam`.
  - Viết hoa đầu (Title Case): `Vn` $\rightarrow$ `Việt Nam`.
  - Viết hoa toàn bộ (All Caps): `VN` $\rightarrow$ `VIỆT NAM`.

---

### 2.6. `ConfigStore` & Khởi động cùng Hệ điều hành (Windows Autostart)
- **Files:** `src/engine/config_store.rs`, `src/platform/registry.rs`
- **Tách biệt Trách nhiệm Lưu trữ (Separation of Concerns):**
  - Cấu hình gõ phím (`EngineConfig`) và danh mục Macro được lưu trữ vào file `config.ini` qua `ConfigStore`.
  - Thuộc tính `show_dialog_on_startup: bool` ("Bật hội thoại này khi khởi động") được lưu trong mục `[system]` của file cấu hình.
  - Trạng thái **Khởi động cùng Windows (Autostart)** được tách rời hoàn toàn khỏi `EngineConfig`, được truy vấn và cấu hình trực tiếp với Windows Registry (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run\MKey`) qua module `src/platform/registry.rs`. Khi chạy tự khởi động với flag `--autostart`, ứng dụng chỉ hiển thị cửa sổ nếu `show_dialog_on_startup == true`.

---

### 2.7. `Language` — Phân hệ Đa ngôn ngữ & Bản địa hóa (i18n / Localization)
- **Files:** `src/language/mod.rs`, `src/language/vi.rs`, `src/language/en.rs`
- **Triệt tiêu Hardcoded Text & Đảm bảo Type-Safety:**
  - Toàn bộ chuỗi văn bản giao diện (tiêu đề cửa sổ, nhãn, nút bấm, placeholder gõ tắt, tray context menu, tooltip) được tập trung tại struct `LanguageStrings` với kiểu `&'static str` (Zero Runtime Allocation).
  - Trình biên dịch Rust bảo đảm tính đầy đủ (Compile-time Completeness): Bất kỳ ngôn ngữ mới nào được bổ sung (Pháp, Nhật, v.v.) bắt buộc phải cung cấp đủ toàn bộ trường chuỗi tương ứng, loại bỏ 100% rủi ro thiếu key hoặc vỡ giao diện.
  - Hỗ trợ chuyển đổi ngôn ngữ linh hoạt tại runtime và lưu trữ tùy chọn ngôn ngữ trong `config.ini` (`[system] language = vi / en`).

---

## 3. Bản đồ File Mã nguồn (Codebase Directory Map)

```
MKey/
├── Cargo.toml                  # Khai báo cấu hình dự án Rust & dependencies
├── src/
│   ├── lib.rs                  # Export library API
│   ├── main.rs                 # Điểm khởi chạy ứng dụng Windows GUI & Hook loop
│   ├── language/               # Phân hệ Đa ngôn ngữ (i18n & Localization)
│   │   ├── mod.rs              # Language enum, LanguageStrings dictionary & switcher
│   │   ├── vi.rs               # Bản dịch Tiếng Việt (Default)
│   │   └── en.rs               # Bản dịch Tiếng Anh (English)
│   ├── engine/                 # Phân hệ điều phối gõ phím
│   │   ├── mod.rs              # VietnameseEngine coordinator
│   │   ├── action.rs           # Định nghĩa EngineAction (Passthrough, Replace, Consume)
│   │   ├── buffer.rs           # TypingBuffer & Thuật toán SSOT evaluate_keys
│   │   ├── config.rs           # EngineConfig (Telex, VNI, language, show_dialog_on_startup...)
│   │   ├── config_store.rs     # Tải/lưu cấu hình config.ini & bảng macro ra đĩa
│   │   ├── history.rs          # WordHistory & Stack quản lý dấu cách
│   │   └── macro_table.rs      # Bảng gõ tắt & Casing preservation
│   ├── ui/                     # Phân hệ Giao diện Native Win32
│   │   ├── mod.rs              # UI Manager facade, Show/Hide control panel
│   │   ├── theme.rs            # Dark/Light theme, Windows 11 UxTheme, GDI brush cache
│   │   ├── paint.rs            # Custom GDI rendering: card bo góc & container
│   │   ├── wnd_proc.rs         # Window Procedures & Message router (WM_PAINT, WM_COMMAND...)
│   │   ├── colors.rs           # Bảng màu định nghĩa UI (Dark/Light palette)
│   │   ├── components/         # Button, CheckBox, ComboBox, ListView, TabBar, TextBox...
│   │   └── views/              # Control Panel layout & System Tray Context Menu
│   ├── vietnamese/             # Phân hệ Ngữ âm học tiếng Việt
│   │   ├── mod.rs              # Vietnamese module export
│   │   ├── charset.rs          # Bảng mã ký tự, nguyên âm cơ bản, dấu thanh, dấu mũ
│   │   ├── onset.rs            # Máy trạng thái phụ âm đầu (OnsetState, dấu đ)
│   │   ├── nucleus.rs          # Máy trạng thái nguyên âm, dấu thanh, dấu mũ, dấu móc
│   │   ├── coda.rs             # Máy trạng thái phụ âm cuối (CodaState, chặn sai âm)
│   │   ├── modifier.rs         # Xử lý phím biến âm Telex / VNI
│   │   ├── spelling.rs         # Ma trận luật ngữ âm & Phonotactics validation
│   │   ├── state.rs            # SyllableState machine điều phối chuyển dịch trạng thái
│   │   └── syllable.rs         # VietnameseSyllable struct & Renderer
│   └── platform/               # Tầng giao tiếp hệ điều hành
│       ├── mod.rs              # Platform facade
│       ├── process.rs          # Đơn phiên chạy (Single-instance Mutex)
│       ├── registry.rs         # Đọc/ghi Windows Registry (Run key Autostart)
│       └── win32/              # Tách lớp native Win32 FFI & Hook
│           ├── mod.rs          # Low-Level Keyboard & Mouse Hook, Windows Message Loop
│           ├── types.rs        # Win32 FFI structs (INPUT, MSG, POINT) & constants
│           ├── injector.rs     # Tổng hợp phím SendInput (Unicode & Backspace)
│           └── app_detect.rs   # Nhận diện tiến trình active & browser Omnibox
├── tests/                      # Bộ kiểm thử tự động (Unit & Integration Tests)
│   ├── comprehensive_test.rs   # 7 tests kiểm thử kịch bản gõ nâng cao
│   ├── config_test.rs          # 4 tests kiểm thử lưu trữ config & registry autostart
│   ├── engine_test.rs          # 34 tests kiểm thử toàn bộ hành vi Engine
│   ├── language_test.rs        # 5 tests kiểm thử từ điển đa ngôn ngữ & i18n
│   ├── listview_test.rs        # 4 tests kiểm thử giao diện danh sách gõ tắt
│   └── spelling_test.rs        # 5 tests kiểm thử luật chính tả và ngữ âm
└── docs/
    └── ARCHITECTURE.md         # Tài liệu kiến trúc này
```

---

## 4. Hướng dẫn Biên dịch & Chạy Kiểm thử (Build & Test Guide)

### 4.1. Yêu cầu hệ thống
- Rust toolchain (phiên bản `1.75+` khuyến nghị): `rustc`, `cargo`.
- Hệ điều hành: Windows 10 / 11 (hỗ trợ Windows API `user32.dll`).

### 4.2. Chạy toàn bộ Test Suite
```bash
# Chạy tất cả 59 unit & integration tests
cargo test

# Chạy riêng từng test suite
cargo test --test comprehensive_test
cargo test --test config_test
cargo test --test engine_test
cargo test --test language_test
cargo test --test listview_test
cargo test --test spelling_test
```

### 4.3. Biên dịch bản Release tối ưu
```bash
cargo build --release
```
File thực thi độc lập sẽ được tạo tại:
`target/release/MKey.exe`

