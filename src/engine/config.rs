use std::fmt;
use std::str::FromStr;

/// Vietnamese Input Method type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMethod {
    #[default]
    Telex,
    Vni,
    SimpleTelex1,
    SimpleTelex2,
}

impl InputMethod {
    /// String representation used for configuration storage
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Telex => "telex",
            Self::Vni => "vni",
            Self::SimpleTelex1 => "simple_telex1",
            Self::SimpleTelex2 => "simple_telex2",
        }
    }

    /// Whether this input method belongs to the Telex syntax family (using letter keys as accents)
    pub const fn is_telex_family(&self) -> bool {
        matches!(
            self,
            InputMethod::Telex | InputMethod::SimpleTelex1 | InputMethod::SimpleTelex2
        )
    }

    /// Whether this input method is VNI (using number keys 1-9 as accents)
    pub const fn is_vni(&self) -> bool {
        matches!(self, InputMethod::Vni)
    }

    /// Whether standalone 'w' at word start or empty state turns into 'ư'
    /// - Standard Telex: Yes ('w' -> 'ư')
    /// - Simple Telex 1: No ('w' stays literal 'w' for English words like "win", "web")
    pub const fn has_standalone_w(&self) -> bool {
        matches!(self, InputMethod::Telex | InputMethod::SimpleTelex2)
    }

    /// Whether square bracket keys '[' and ']' act as shortcuts for 'ư' and 'ơ'
    /// - Standard Telex: Yes ('[' -> 'ư', ']' -> 'ơ')
    /// - Simple Telex: No ('[' and ']' remain brackets for programming)
    pub const fn has_bracket_shortcuts(&self) -> bool {
        matches!(self, InputMethod::Telex)
    }
}

impl fmt::Display for InputMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for InputMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for InputMethod {
    fn from(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "vni" => Self::Vni,
            "simple_telex1" | "simpletelex1" | "simpletelex" => Self::SimpleTelex1,
            "simple_telex2" | "simpletelex2" => Self::SimpleTelex2,
            _ => Self::Telex,
        }
    }
}

impl FromStr for InputMethod {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl From<InputMethod> for usize {
    fn from(m: InputMethod) -> Self {
        match m {
            InputMethod::Telex => 0,
            InputMethod::Vni => 1,
            InputMethod::SimpleTelex1 | InputMethod::SimpleTelex2 => 2,
        }
    }
}

impl From<usize> for InputMethod {
    fn from(idx: usize) -> Self {
        match idx {
            1 => Self::Vni,
            2 => Self::SimpleTelex1,
            _ => Self::Telex,
        }
    }
}

/// UI Theme mode (Auto, Light, Dark)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UiTheme {
    #[default]
    Auto,
    Light,
    Dark,
}

impl UiTheme {
    /// String representation used for configuration storage
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

impl fmt::Display for UiTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for UiTheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for UiTheme {
    fn from(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "light" => Self::Light,
            "dark" => Self::Dark,
            _ => Self::Auto,
        }
    }
}

impl FromStr for UiTheme {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl From<UiTheme> for usize {
    fn from(t: UiTheme) -> Self {
        match t {
            UiTheme::Auto => 0,
            UiTheme::Light => 1,
            UiTheme::Dark => 2,
        }
    }
}

impl From<usize> for UiTheme {
    fn from(idx: usize) -> Self {
        match idx {
            1 => Self::Light,
            2 => Self::Dark,
            _ => Self::Auto,
        }
    }
}


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
    /// Phím chuyển chế độ (true: Ctrl + Shift, false: Alt + Z)
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
