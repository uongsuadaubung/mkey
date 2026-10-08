# MKey — Bộ gõ tiếng Việt thế hệ mới bằng Rust

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)]()
[![Tests](https://img.shields.io/badge/tests-54%20passed-brightgreen.svg)]()
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

> 🚀 **MKey Rust Engine:** Bộ gõ tiếng Việt thế hệ mới được viết bằng **Rust**, tập trung hoàn toàn vào sự chuẩn xác, tốc độ và độ mượt mà khi gõ văn bản hàng ngày. Loại bỏ hoàn toàn biến toàn cục dễ vỡ, buffer corruption và các lỗi xung đột dấu kinh điển.  
> 📖 **Tài liệu kiến trúc chi tiết:** [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)

---

## ⚡ Bắt đầu nhanh (Quick Start)

### Yêu cầu
- [Rust toolchain](https://rustup.rs/) (khuyến nghị phiên bản 1.75 trở lên)
- Hệ điều hành: Windows 10 / 11

### Chạy kiểm thử (Automated Tests)
Hệ thống được bảo đảm bởi bộ kiểm thử tự động gồm **54 test cases** (Unit, Integration, Spelling, Macro, Undo Toggle, Casing, Mode Switch, Glides):
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
│   ├── main.rs                  # Entry point ứng dụng
│   ├── lib.rs                   # Thư viện MKey public API
│   ├── platform/
│   │   ├── mod.rs
│   │   └── win32.rs             # Windows Low-Level Keyboard/Mouse Hook & SendInput
│   ├── engine/
│   │   ├── mod.rs               # VietnameseEngine điều phối trung tâm
│   │   ├── buffer.rs            # TypingBuffer quản lý chuỗi phím thô
│   │   ├── history.rs           # WordHistory theo dõi từ qua dấu cách
│   │   ├── macro_table.rs       # Bảng gõ tắt và xử lý casing
│   │   ├── action.rs            # EngineAction (Passthrough, Replace, Consume)
│   │   └── config.rs            # EngineConfig cấu hình bộ gõ
│   ├── ui/
│   │   ├── mod.rs               # Hệ thống Win32 UI & message pump
│   │   ├── components/          # Controls Win32 thuần (Button, Checkbox, Tab, Combo...)
│   │   └── views/               # Control Panel & System Tray
│   └── vietnamese/
│       ├── mod.rs
│       ├── state.rs             # Máy trạng thái âm tiết: Onset -> Nucleus -> Coda
│       ├── syllable.rs          # Cấu trúc âm tiết tiếng Việt
│       ├── spelling.rs          # Quy tắc ngữ âm học & ghép vần tiếng Việt
│       ├── charset.rs           # Bảng ký tự, nguyên âm, dấu thanh, dấu phụ
│       ├── telex.rs             # Bảng quy tắc biến âm Telex
│       └── vni.rs               # Bảng quy tắc biến âm VNI
├── tests/
│   ├── comprehensive_test.rs    # Bộ kiểm thử tổng hợp 7 nhóm tính năng
│   ├── engine_test.rs           # 26 bài test chuyên sâu bộ đệm & khôi phục từ
│   └── spelling_test.rs         # 5 bài test ngữ âm học và cấu trúc vần
└── docs/
    └── ARCHITECTURE.md          # Tài liệu đặc tả kiến trúc kỹ thuật toàn diện
```

---

## 👨‍💻 Tác giả phát triển
- **Mạnh Kiên** (`manhkien13041997@gmail.com`)
- Phiên bản **MKey Rust Native Edition** (2026).

## 📜 Giấy phép
- Phân phối theo giấy phép mã nguồn mở **GPL-3.0**.
