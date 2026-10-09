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
        Injector["SendInput & Omnibox Guard (src/platform/win32/injector.rs)"]
        AppDetect["App Detection (src/platform/win32/app_detect.rs)"]
        Registry["Windows Registry Run Key (src/platform/registry.rs)"]
    end

    subgraph UI_Layer ["2. Tầng Giao diện Native Win32 (UI Layer)"]
        WndProc["Window Procedure Router (src/ui/wnd_proc.rs)"]
        CtrlPanel["Control Panel Views (src/ui/views/)"]
        ThemeEngine["Theme & GDI Renderer (src/ui/theme.rs, paint.rs, colors.rs)"]
        Components["Native Components & Subclassing (src/ui/components/)"]
        TrayMenu["System Tray Context Menu"]
    end

    subgraph Engine_Layer ["3. Tầng Điều phối (Engine Coordinator Layer)"]
        VE["VietnameseEngine (src/engine/mod.rs)"]
        ConfigStore["ConfigStore (src/engine/config_store.rs)"]
        MacroTbl["MacroTable (Bảng gõ tắt 3 tầng & Casing)"]
        Hist["WordHistory (Ring Buffer VecDeque phục hồi từ)"]
        SpellingGuard["Spelling Checker & English Word Restore"]
    end

    subgraph Language_Layer ["4. Phân hệ Đa ngôn ngữ (Localization)"]
        Lang["Language & LanguageStrings (src/language/)"]
    end

    subgraph Buffer_Layer ["5. Tầng Bộ đệm (Typing Buffer Layer)"]
        TB["TypingBuffer (src/engine/buffer.rs)"]
        RawKeys["raw_keys: Vec<RawKey> (Single Source of Truth)"]
        Eval["evaluate_keys() & Common Prefix Diff"]
    end

    subgraph Linguistic_Layer ["6. Tầng Ngữ âm học (Linguistic State Machine)"]
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
    VE -->|"EngineAction (Replace / Passthrough)"| Injector
    AppDetect --> Injector
    WndProc --> CtrlPanel & TrayMenu
    CtrlPanel --> Components & ThemeEngine
    Lang --> CtrlPanel & TrayMenu
    ConfigStore --> VE & WndProc
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
- **Tối ưu hóa Tiền tố chung (Common Prefix Differential Optimization):**
  - Khi người dùng gõ phím biến đổi dấu hoặc vần (ví dụ: `go` $\to$ `gõ`), `TypingBuffer` so khớp chuỗi ký tự hiển thị cũ và mới thông qua iterator streaming `chars().zip()`.
  - Thay vì phát lệnh xóa toàn bộ từ cũ và gõ lại cả từ (`Backspace 2` + `gõ`), engine chỉ phát lệnh lùi phần hậu tố bị thay đổi (`Backspace 1` + `õ`), giữ nguyên tiền tố chung `g`.
  - Thuật toán này triệt tiêu hoàn toàn lỗi kinh điển "nhân đôi ký tự" (`ggõ`, `ttoán`) trên các thanh địa chỉ trình duyệt (Chrome, Edge, Firefox).
- **Phát hiện Biên từ CamelCase (`is_boundary`):**
  - Khi gặp một chữ cái viết HOA xen giữa các chữ cái viết thường (ví dụ: `userName`), buffer tự động chốt từ `user` và bắt đầu phiên âm tiết mới với `Name`.
  - Việc xử lý âm tiết đã đóng cấu trúc (ví dụ: `ôi`, `ay` không nhận thêm phụ âm) được điều phối tự nhiên thông qua chu trình chuyển dịch của máy trạng thái ngữ âm `SyllableState`.

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

### 2.3. `WordHistory` — Ring Buffer Phục hồi qua Phím cách
- **File:** `src/engine/history.rs`
- Cung cấp tính năng **"Nhớ từ đã gõ qua phím cách"**: Khi người dùng đã gõ xong từ và bấm dấu cách, nếu bấm Backspace xóa dấu cách đó thì từ cũ được nạp lại vào buffer để tiếp tục sửa.
- **Cấu trúc dữ liệu:**
  ```rust
  pub struct CommittedWord {
      pub raw_keys: Vec<RawKey>,
      pub is_raw_restored: bool,
      pub spaces_after: usize,
  }
  ```
- **Kiến trúc Ring Buffer $O(1)$ (`VecDeque`):**
  - Hệ thống duy trì tối đa `MAX_HISTORY_WORDS = 50` từ đã gõ trong hàng đợi hai đầu `VecDeque<CommittedWord>`.
  - Khi bộ nhớ đệm đạt giới hạn, thao tác thu hồi phần tử cũ nhất thực hiện qua `pop_front()` với độ phức tạp $O(1)$ thay vì dồn mảng $O(N)$, loại bỏ nguy cơ giật lag và rò rỉ bộ nhớ khi gõ văn bản dài.
- **Quy tắc an toàn tuyệt đối:**
  1. Chỉ phục hồi từ khi `spaces_after == 1` (đúng một dấu cách ngăn giữa con trỏ và từ trước).
  2. Khi gặp phím xuống dòng (`Enter`: `\r`, `\n`), dấu câu (`.`, `,`, `!`, `?`...), click chuột hoặc phím điều hướng $\rightarrow$ gọi `WordHistory::clear()`. Điều này triệt tiêu hoàn toàn lỗi "lội ngược dòng" gây biến dạng từ ngữ.
  3. **Tự động ngắt từ phục hồi (Graceful Word Boundary Detachment):** Khi một từ được nạp lại qua phím cách, nếu người dùng gõ một phím mới không thể ghép vần (làm từ rơi vào `Passthrough`), buffer tự động ngắt bỏ từ phục hồi để gõ từ mới độc lập, không bẫy người dùng vào từ cũ.

---

### 2.4. `Spelling Guard` — Tự động Khôi phục Từ Tiếng Anh (`restore_on_wrong_spelling`)
- **File:** `src/engine/mod.rs` & `src/vietnamese/spelling.rs`
- Khắc phục nhược điểm kinh điển của bộ gõ tiếng Việt khi người dùng gõ từ tiếng Anh chứa các phím dấu Telex (`s, f, r, x, j, w`):
  - `fix` $\rightarrow$ tạm thời hiển thị `fĩ` trên màn hình.
  - Khi người dùng nhấn phím ngắt từ (`Space`): Engine phân tích âm tiết. Vì tiếng Việt không tồn tại âm tiết `fĩ` (phụ âm đầu `f` không kết hợp dấu ngã `ĩ`), hệ thống tự động phát ra lệnh `Replace`: lùi lại 2 ký tự xóa `fĩ` và xuất trả `fix `.
  - Hỗ trợ toàn diện các từ phổ biến: `fix`, `fox`, `fax`, `first`, `server`, `user`, `word`, `win`...
  - Không làm ảnh hưởng đến các từ tiếng Việt hợp lệ có kết thúc bằng phím `x` như: `mã` (`max`), `sĩ` (`six`), `bõ` (`box`), `sẽ` (`sex`), `tã` (`tax`).

---

### 2.5. `MacroTable` — Bảng Gõ tắt 3 Tầng & Bảo toàn Dạng Chữ
- **File:** `src/engine/macro_table.rs`
- Tự động thay thế từ viết tắt khi nhấn phím cách hoặc gõ vần.
- **Hệ thống phân loại Gõ tắt 3 tầng (`MacroType`):**
  1. `Normal`: Gõ tắt từ nguyên khối (`vn` $\to$ `việt nam`).
  2. `StartConsonant`: Thay thế phụ âm đầu siêu tốc (`f` $\to$ `ph`, `j` $\to$ `gi`, `w` $\to$ `qu`).
  3. `EndConsonant`: Thay thế phụ âm cuối siêu tốc (`g` $\to$ `ng`, `h` $\to$ `nh`, `k` $\to$ `ch`).
- **Bảo vệ Phụ âm Đơn (Single Character Guard):**
  - Khi người dùng gõ một phụ âm đơn đứng một mình rồi nhấn Space (ví dụ: `f `, `g `, `j `), engine bảo toàn ký tự gốc không kích hoạt gõ tắt (nhằm giữ an toàn cho biến trong lập trình, công thức toán `f(x)` hoặc lệnh console).
  - Phụ âm đầu/cuối chỉ tự động biến đổi khi kết hợp với cấu trúc âm tiết hợp lệ theo sau (`fa` $\to$ `pha`, `dag` $\to$ `dang`).
- **Cơ chế Pre-sorted Cache & Batch Loading:**
  - Hai danh mục phụ âm đầu (`start_consonants`) và phụ âm cuối (`end_consonants`) được duy trì trong bộ đệm sắp xếp sẵn theo độ dài khóa giảm dần (`len DESC`), bảo đảm thuật toán Greedy Matching luôn ưu tiên khớp cụm ký tự dài nhất trước.
  - Khi nạp danh mục cấu hình lớn từ file đĩa, engine sử dụng `insert_typed_no_cache()` và chỉ gọi `rebuild_cache()` một lần duy nhất sau khi nạp xong, giảm độ phức tạp từ $O(N \cdot K \log K)$ xuống $O(K \log K)$.
- **Bảo toàn Casing thông minh:**
  - Viết thường: `vn` $\to$ `việt nam`.
  - Viết hoa đầu (Title Case): `Vn` $\to$ `Việt Nam`.
  - Viết hoa toàn bộ (All Caps): `VN` $\to$ `VIỆT NAM`.

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
  - Chuyển đổi ngôn ngữ an toàn đa luồng thông qua cờ nguyên tử `AtomicU8` (`CURRENT_LANG`) và lưu trữ tùy chọn vào `config.ini` (`[system] language = vi / en`).

---

### 2.8. `Platform & Win32 Injector` — Tầng Giao tiếp Hệ điều hành Cấp thấp
- **Files:** `src/platform/win32/mod.rs`, `src/platform/win32/injector.rs`, `src/platform/win32/app_detect.rs`
- **Kỹ thuật Tổng hợp Phím Nguyên tử (`SendInput`):**
  - Chuỗi phím xóa lùi (`VK_BACK`) và ký tự Unicode mới được gom vào một mảng `Vec<INPUT>` duy nhất và gửi qua một lời gọi `SendInput` nguyên tử. Ngăn chặn hiện tượng con trỏ bị trôi hoặc xung đột luồng gõ phím.
- **Cơ chế Autocomplete Guard bằng ký tự vô hình `U+202F` (`injector.rs` & `app_detect.rs`):**
  - Khi người dùng gõ trên thanh địa chỉ Chromium Omnibox (Chrome, Edge, Brave), Firefox hoặc ô Excel đang có văn bản gợi ý tự động (inline autocomplete selection):
    1. Gửi ký tự vô hình `U+202F` (Narrow No-Break Space) để xóa đè vùng chọn gợi ý mà không làm con trỏ nhảy về cuối dòng.
    2. Gửi 1 phím Backspace xóa ký tự `U+202F`.
    3. Thực hiện lùi `backspaces` và gõ ký tự tiếng Việt bình thường.
- **Trạng thái Cách ly Phím tắt Chuyển Chế độ (`CTRL_SHIFT_ARMED`):**
  - Cờ nguyên tử `CTRL_SHIFT_ARMED` kiểm soát vòng đời chuyển đổi Việt/Anh (`Ctrl + Shift`). Nếu người dùng nhấn thêm bất kỳ phím thứ 3 nào (ví dụ: `Ctrl + Shift + Esc`), cờ tự động hủy kích hoạt, ngăn cản cướp phím tắt của các phần mềm khác.
- **Thu gọn Bộ nhớ Làm việc (`trim_working_set`):**
  - Tích hợp `SetProcessWorkingSetSize` khi người dùng ẩn/đóng Bảng điều khiển xuống System Tray, chủ động trả lại các trang bộ nhớ vật lý không sử dụng về cho Windows kernel.

---

### 2.9. `Native Win32 UI & Theming` — Phân hệ Giao diện Thuần Native
- **Files:** `src/ui/mod.rs`, `src/ui/paint.rs`, `src/ui/theme.rs`, `src/ui/colors.rs`, `src/ui/wnd_proc.rs`
- **Kiến trúc GDI Thuần (Pure Win32 GDI):**
  - Không sử dụng WebView, Electron hay framework giao diện cồng kềnh. Toàn bộ các thành phần hiển thị (card bo góc, tab bar, nút bấm, bảng danh sách ListView) được kết xuất trực tiếp qua Windows GDI và UxTheme.
- **Khử chớp nháy (Double Buffering):**
  - Mọi thao tác vẽ container và thẻ card được thực hiện trên một Memory Device Context (`CreateCompatibleDC`, `CreateCompatibleBitmap`) trước khi đẩy ra màn hình bằng `BitBlt` trong thông điệp `WM_PAINT`, mang lại trải nghiệm thị giác mượt mà 100%.
- **Cơ chế Subclassing Điều khiển Win32 (`SetWindowSubclass`):**
  - Can thiệp thông điệp `WM_PAINT`, `WM_MOUSEMOVE`, `WM_MOUSELEAVE` của các điều khiển Win32 chuẩn (`BUTTON`, `SysListView32`) để áp dụng phong cách thiết kế hiện đại mà vẫn bảo toàn đầy đủ tính năng trợ năng (accessibility) và điều hướng bàn phím của hệ điều hành.
- **Hệ thống Giao diện Sáng/Tối (Light/Dark Theme Engine):**
  - Tự động nhận diện theme hệ thống Windows qua Registry (`AppsUseLightTheme`), hỗ trợ chuyển đổi giao diện thời gian thực với bảng màu ngữ nghĩa `ThemePalette` và bộ nhớ đệm GDI brush/pen tái sử dụng.

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
│   │   ├── history.rs          # WordHistory Ring Buffer VecDeque quản lý từ
│   │   └── macro_table.rs      # Bảng gõ tắt 3 tầng (Normal, Start, End) & Casing
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
│   ├── comprehensive_test.rs   # Kiểm thử kịch bản gõ phức hợp & benchmark thông lượng
│   ├── config_test.rs          # Kiểm thử lưu trữ cấu hình & registry autostart
│   ├── engine_test.rs          # Kiểm thử chuyên sâu toàn bộ hành vi Engine
│   ├── language_test.rs        # Kiểm thử từ điển đa ngôn ngữ & i18n
│   └── spelling_test.rs        # Kiểm thử quy tắc ngữ âm học & ghép vần tiếng Việt
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
# Chạy tất cả unit & integration tests
cargo test

# Chạy riêng từng test suite
cargo test --test comprehensive_test
cargo test --test config_test
cargo test --test engine_test
cargo test --test language_test
cargo test --test spelling_test
```

### 4.3. Biên dịch bản Release
```bash
cargo build --release
```
File thực thi độc lập sẽ được tạo tại:
`target/release/MKey.exe`
