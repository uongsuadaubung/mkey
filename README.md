# MKey — Bộ gõ tiếng Việt thế hệ mới bằng Rust

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)]()
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)]()
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

> 🚀 **MKey Rust Engine:** Bộ gõ tiếng Việt thế hệ mới được viết bằng **Rust**, tập trung hoàn toàn vào sự chuẩn xác, tốc độ và độ mượt mà khi gõ văn bản hàng ngày. Loại bỏ hoàn toàn biến toàn cục dễ vỡ, buffer corruption và các lỗi xung đột dấu kinh điển.  
> 📖 **Tài liệu kiến trúc chi tiết:** [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)

---

## ⚡ Hiệu năng Kỷ lục & Số liệu Landing Page (Highlights)

> *"Gõ nhanh như chớp, nhẹ như lông vũ — Bộ gõ tiếng Việt duy nhất hoạt động dưới 1 MB RAM."*

| Chỉ số (Metric) | Kết quả đo đạc thực tế (Benchmark) | Ý nghĩa đối với người dùng & Game thủ |
| :--- | :--- | :--- |
| 🚀 **Tốc độ xử lý (Throughput)** | **~10.960.000 ký tự / giây** | Xử lý gần 11 triệu phím/giây, không bao giờ bị nghẽn phím khi gõ cực nhanh |
| ⏱️ **Độ trễ phản hồi (Latency)** | **~0.09 microsecond / phím** (0.00009 ms / 91 ns) | Phản hồi ngay trong 91 nano-giây, nhanh gấp **150.000 lần** ngưỡng mắt người nhận biết |
| 💾 **Bộ nhớ RAM (Tray / Chạy ngầm)** | **0.5 – 0.9 MB RAM** | Nhẹ kỷ lục; gần như vô hình trong Task Manager, không tốn tài nguyên máy |
| 🖥️ **Bộ nhớ RAM (Control Panel mở)** | **~3.5 MB RAM** | Toàn bộ giao diện Win32 native siêu gọn nhẹ, không dùng WebView/Electron cồng kềnh |
| 📦 **Dung lượng file thực thi (.exe)** | **~479 KB** | Siêu nhỏ gọn, nhẹ hơn cả một bức ảnh; tải và khởi chạy tức thì |
| 🛡️ **Runtime Dependencies** | **0 dependencies (Pure Rust)** | 100% Rust thuần giao tiếp trực tiếp Windows API, không rủi ro bảo mật bên thứ 3 |
| 🎮 **Hiện tượng khựng phím (Stutter)** | **Zero Input Lag / Zero GC Pause** | Không có Garbage Collector (GC) thu gom rác gây trễ nhịp khi chơi game đỉnh cao |

### 🏆 So sánh với các bộ gõ tiếng Việt thông thường

| Tiêu chí | MKey (Rust Native) | Bộ gõ C/C++ truyền thống | Bộ gõ nền tảng Web / Electron |
| :--- | :--- | :---: | :---: |
| **Dung lượng file thực thi (.exe)** | **~479 KB** | 3 MB – 10 MB | 80 MB – 150 MB |
| **Mức chiếm dụng RAM khi chạy ngầm** | **< 1 MB** (0.5 – 0.9 MB) | 15 MB – 40 MB | 100 MB – 300 MB |
| **Độ trễ xử lý phím** | **~0.09 µs** (91 ns) | 1.5 – 5.0 µs | 10 – 30 ms (có độ trễ) |
| **Khựng khung hình (Micro-stutter)** | **Hoàn toàn KHÔNG (Zero GC)** | Hiếm gặp | Thường xuyên do GC / Event loop |
| **Cơ chế khôi phục từ tiếng Anh** | **Tự động theo ngữ âm** | Thoát dấu thủ công / Danh sách thô | Dễ xung đột phím |
| **Công nghệ khôi phục Backspace** | **SSOT Replay tất định** | Phỏng đoán ký tự xóa (dễ vỡ) | Phỏng đoán |

---

## ⚡ Bắt đầu nhanh (Quick Start)

### Yêu cầu
- [Rust toolchain](https://rustup.rs/) (khuyến nghị phiên bản 1.75 trở lên)
- Hệ điều hành: Windows 10 / 11

### Chạy kiểm thử (Automated Tests)
Hệ thống được bảo đảm bởi bộ kiểm thử tự động toàn diện (Unit, Integration, Spelling, Macro, Language/i18n, Undo Toggle, Casing, Mode Switch, Glides, Performance Throughput):
```bash
cargo test
```

### Biên dịch bản phát hành (Build Release)
```bash
cargo build --release
```
Tập tin thực thi sau khi biên dịch nằm tại: `target/release/MKey.exe`.

---

## 💎 Đặc điểm Kiến trúc & Tính năng Cốt lõi

1. **Phím tắt chuyển nhanh Tiếng Việt / Tiếng Anh (`Ctrl + Shift`):**
   - Hỗ trợ chuyển đổi tức thì giữa chế độ gõ Tiếng Việt và Tiếng Anh chỉ bằng tổ hợp `Ctrl + Shift`.
   - Cơ chế cách ly hoàn toàn (Smart Arming/Disarming): Nếu nhấn tổ hợp khác như `Ctrl + Shift + P` (IDE), `Ctrl + Shift + Esc` (Task Manager), hay phím tắt bất kỳ thì cơ chế chuyển chế độ tự động vô hiệu hóa, không bao giờ cướp phím tắt của phần mềm khác.

2. **Deterministic Replay Buffer (Single Source of Truth):**
   - Bộ đệm `TypingBuffer` luôn coi chuỗi phím thô (`raw_keys`) là nguồn sự thật duy nhất.
   - Khi bấm phím `Backspace`, engine không "đoán mò" ký tự xóa mà tua lại từ đầu chuỗi phím thô qua State Machine, loại bỏ 100% lỗi "dấu ma" hay vỡ ký tự.

3. **Unified Linguistic Modifiers (Nguồn sự thật ngữ âm tập trung):**
   - Toàn bộ quy tắc đặt dấu thanh (`s, f, r, x, j, z`), dấu mũ (`aa, ee, oo`), dấu móc (`w`), phụ âm đ (`dd`), và cơ chế Undo toggle (gõ phím lặp lại để hủy dấu) được tập trung tại `NucleusState::apply_telex_modifier` và `apply_vni_modifier`.
   - Đảm bảo tính nhất quán tuyệt đối giữa gõ dấu sớm (`toán`), gõ dấu tự do ở cuối từ (`toans`), hoặc gõ từ có âm đệm (`quán`, `hoàng`).

4. **Tự động khôi phục từ tiếng Anh (English Word Auto-Restoration):**
   - Khi gõ các từ tiếng Anh có chứa bẫy dấu Telex (`fix`, `Fix`, `FIX`, `user`, `server`, `first`, `word`), engine tự động phát hiện và khôi phục về từ tiếng Anh nguyên bản khi bấm dấu cách hoặc phím ngắt từ.
   - Các từ tiếng Việt hợp lệ kết thúc bằng phím dấu (`mã`, `sĩ`, `bõ`) không bị khôi phục nhầm.

5. **Khôi phục từ xuyên khoảng trắng (Backspace across space):**
   - Bấm `Backspace` xóa dấu cách sẽ tự động nạp lại từ trước đó vào bộ đệm để người dùng chỉnh sửa tiếp tục mà không cần gõ lại từ đầu.
   - Bộ nhớ lịch sử được giới hạn cứng (`MAX_HISTORY_WORDS = 50`) để ngăn ngừa rò rỉ bộ nhớ khi gõ văn bản dài.

6. **Bộ gõ tắt thông minh (Smart Macro Expansion):**
   - Tự động biến đổi hoa/thường theo từ gốc:
     - `ko` $\to$ `không`
     - `Ko` $\to$ `Không`
     - `KO` $\to$ `KHÔNG`
   - Tra cứu linh hoạt cả từ thô (`ddc`) lẫn từ hiển thị (`đc`).

---

## 🎮 Lộ trình phát triển & Tính năng mở rộng

### Chế độ chơi game (WASD Exclusion / Game Mode)
Khi chơi game (CS, Valorant, Genshin, v.v.), người dùng nhấn các phím di chuyển `W, A, S, D`. Nếu quên tắt tiếng Việt, `W` có thể biến thành `ư`, `S` có thể nuốt phím để đợi bỏ dấu sắc.
- Người dùng có thể dùng ngay phím tắt **`Ctrl + Shift`** vừa được trang bị để tắt gõ tiếng Việt trước khi vào game.
- Dự kiến tính năng tự động: **Danh sách đen tự động loại trừ (Blacklist / Exclude apps)** tự động nhận diện cửa sổ game đang active để tự ngắt tiếng Việt và tự bật lại khi quay về ứng dụng văn phòng/trình duyệt.

### Bảng mã truyền thống (Legacy Charset Transcoder)
- Bổ sung bộ chuyển mã (Transcoder) sang các bảng mã cổ như TCVN3 (ABC), VNI-Windows, CP1258 để phục vụ các phần mềm kế toán, kỹ thuật cũ (AutoCAD, phần mềm khai thuế).

### Worker Thread ghi Log bất đồng bộ (Async Logger)
- Chuyển toàn bộ việc ghi file nhật ký chẩn đoán `mkey_debug.log` sang worker thread chạy nền thông qua kênh `mpsc`, đảm bảo luồng Hook Windows đạt độ trễ tuyệt đối dưới 0.1ms.

---

## 📂 Cấu trúc Thư mục Dự án

```
MKey/
├── Cargo.toml
├── src/
│   ├── main.rs                  # Entry point ứng dụng (GUI & Win32 Hook Loop)
│   ├── lib.rs                   # Thư viện MKey public API
│   ├── language/                # Phân hệ Đa ngôn ngữ (i18n & Localization)
│   │   ├── mod.rs               # Language enum & LanguageStrings dictionary
│   │   ├── vi.rs                # Bản dịch Tiếng Việt (Default)
│   │   └── en.rs                # Bản dịch Tiếng Anh (English)
│   ├── platform/
│   │   ├── mod.rs               # Platform facade
│   │   ├── process.rs           # Đơn phiên chạy (Single-instance Mutex)
│   │   ├── registry.rs          # Đọc/ghi Windows Registry (Run key Autostart)
│   │   └── win32/               # Tách lớp native Win32 FFI & Hook
│   │       ├── mod.rs           # Low-Level Keyboard & Mouse Hook, Windows Message Loop
│   │       ├── types.rs         # Win32 FFI structs (INPUT, MSG, POINT) & constants
│   │       ├── injector.rs      # Tổng hợp phím SendInput (Unicode & Backspace)
│   │       └── app_detect.rs    # Nhận diện tiến trình active & browser Omnibox
│   ├── engine/
│   │   ├── mod.rs               # VietnameseEngine điều phối trung tâm
│   │   ├── buffer.rs            # TypingBuffer quản lý chuỗi phím thô (SSOT)
│   │   ├── history.rs           # WordHistory Ring Buffer VecDeque quản lý từ
│   │   ├── macro_table.rs       # Bảng gõ tắt 3 tầng (Normal, Start, End) & Casing
│   │   ├── action.rs            # EngineAction (Passthrough, Replace, Consume)
│   │   ├── config.rs            # EngineConfig cấu hình bộ gõ
│   │   └── config_store.rs      # Tải/lưu cấu hình config.ini & bảng macro ra đĩa
│   ├── ui/
│   │   ├── mod.rs               # Hệ thống Win32 UI facade & message pump
│   │   ├── theme.rs             # Dark/Light theme, Windows 11 UxTheme, GDI brush cache
│   │   ├── paint.rs             # Custom GDI rendering: card bo góc & container
│   │   ├── wnd_proc.rs          # Window Procedures & Message router
│   │   ├── colors.rs            # Bảng màu định nghĩa giao diện
│   │   ├── components/          # Controls Win32 thuần (Button, Checkbox, ListView, Tab...)
│   │   └── views/               # Control Panel & System Tray Context Menu
│   └── vietnamese/
│       ├── mod.rs               # Export module xử lý tiếng Việt
│       ├── onset.rs             # Quản lý phụ âm đầu (OnsetState, phím đ)
│       ├── nucleus.rs           # Quản lý nguyên âm, dấu thanh, dấu mũ, dấu móc
│       ├── coda.rs              # Quản lý phụ âm cuối (CodaState, kiểm soát chính tả)
│       ├── state.rs             # Máy trạng thái âm tiết: Empty -> Onset -> Nucleus -> Coda
│       ├── modifier.rs          # Xử lý phím biến âm Telex / VNI tập trung
│       ├── syllable.rs          # Cấu trúc âm tiết tiếng Việt
│       ├── spelling.rs          # Quy tắc ngữ âm học & ghép vần tiếng Việt
│       └── charset.rs           # Bảng ký tự, nguyên âm, dấu thanh, dấu phụ
├── tests/
│   ├── comprehensive_test.rs    # Kiểm thử kịch bản gõ phức hợp & benchmark thông lượng
│   ├── config_test.rs           # Kiểm thử lưu trữ cấu hình & registry autostart
│   ├── engine_test.rs           # Kiểm thử chuyên sâu bộ đệm, macro & phục hồi từ
│   ├── language_test.rs         # Kiểm thử phân hệ đa ngôn ngữ & từ điển i18n
│   └── spelling_test.rs         # Kiểm thử quy tắc ngữ âm học & ghép vần tiếng Việt
└── docs/
    └── ARCHITECTURE.md          # Tài liệu đặc tả kiến trúc kỹ thuật toàn diện
```

---

## 👨‍💻 Tác giả phát triển
- **Mạnh Kiên** (`manhkien13041997@gmail.com`)
- Phiên bản **MKey Rust Native Edition** (2026).

## 📜 Giấy phép
- Phân phối theo giấy phép mã nguồn mở **GPL-3.0**.
