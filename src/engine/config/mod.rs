//! Engine Configuration options and models

pub mod hotkey;
pub mod input_method;
pub mod theme;

pub use hotkey::*;
pub use input_method::*;
pub use theme::*;

/// Configuration options for the Vietnamese engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineConfig {
    pub method: InputMethod,
    pub theme: UiTheme,
    pub language: crate::language::Language,
    /// Bật/tắt chế độ gõ tiếng Việt (true: bật gõ tiếng Việt, false: tiếng Anh)
    pub enabled: bool,
    /// Tự động khôi phục phím nếu gõ sai chính tả
    pub restore_on_wrong_spelling: bool,
    /// Tự động viết hoa chữ cái đầu câu sau dấu chấm / Enter
    pub auto_uppercase_first_char: bool,
    /// Phím gõ tắt ngoặc vuông: [ -> ư, ] -> ơ
    pub bracket_w: bool,
    /// Bật tính năng gõ tắt (Macro) - Mặc định TẮT
    pub use_macro: bool,
    /// Cho phép gõ tắt cả khi ở chế độ tiếng Anh
    pub use_macro_in_english_mode: bool,
    /// Khôi phục từ khi bấm Backspace xoá dấu cách
    pub remember_history_across_space: bool,
    /// Bật/tắt sử dụng phím tắt chuyển chế độ
    pub switch_key_enabled: bool,
    /// Phím chuyển chế độ (mặc định Ctrl + Shift, hoặc bất kỳ phím nào người dùng cài đặt)
    pub switch_key: Hotkey,
    /// Tương thích ngược: true nếu switch_key là Ctrl + Shift
    pub switch_with_ctrl_shift: bool,
    /// Bật/tắt âm thanh phím cơ
    pub sound_enabled: bool,
    /// Tên profile switch phím cơ (tương ứng với tên thư mục trong ~/.config/mkey/switches/)
    pub sound_profile: String,
    /// Âm lượng phím cơ (0% - 100%)
    pub sound_volume: u8,
    /// Bật hội thoại này khi khởi động cùng Windows
    pub show_dialog_on_startup: bool,
    /// Bật chế độ debug log (ghi nhận vết từng phím, trạng thái state machine và output để chẩn đoán lỗi)
    pub debug: bool,
    /// Đường dẫn file lưu nhật ký debug log trên ổ đĩa
    pub debug_file_path: Option<String>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            method: InputMethod::Telex,
            theme: UiTheme::Auto,
            language: crate::language::Language::default(),
            enabled: true,
            restore_on_wrong_spelling: true,
            auto_uppercase_first_char: false,
            bracket_w: true,
            use_macro: false,
            use_macro_in_english_mode: true,
            remember_history_across_space: true,
            switch_key_enabled: true,
            switch_key: Hotkey::default(),
            switch_with_ctrl_shift: true,
            sound_enabled: false,
            sound_profile: "Holy Panda".to_string(),
            sound_volume: 50,
            show_dialog_on_startup: true,
            debug: false,
            debug_file_path: Some("mkey_debug.log".to_string()),
        }
    }
}
