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
    /// Whether this input method belongs to the Telex syntax family (using letter keys as accents)
    #[inline]
    pub fn is_telex_family(&self) -> bool {
        matches!(self, InputMethod::Telex | InputMethod::SimpleTelex1 | InputMethod::SimpleTelex2)
    }

    /// Whether this input method is VNI (using number keys 1-9 as accents)
    #[inline]
    pub fn is_vni(&self) -> bool {
        matches!(self, InputMethod::Vni)
    }

    /// Whether standalone 'w' at word start or empty state turns into 'ư'
    /// - Standard Telex: Yes ('w' -> 'ư')
    /// - Simple Telex 1: No ('w' stays literal 'w' for English words like "win", "web")
    #[inline]
    pub fn has_standalone_w(&self) -> bool {
        matches!(self, InputMethod::Telex | InputMethod::SimpleTelex2)
    }

    /// Whether square bracket keys '[' and ']' act as shortcuts for 'ư' and 'ơ'
    /// - Standard Telex: Yes ('[' -> 'ư', ']' -> 'ơ')
    /// - Simple Telex: No ('[' and ']' remain brackets for programming)
    #[inline]
    pub fn has_bracket_shortcuts(&self) -> bool {
        matches!(self, InputMethod::Telex)
    }
}

/// Configuration options for the Vietnamese engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineConfig {
    pub method: InputMethod,
    /// Bật/tắt chế độ gõ tiếng Việt (true: bật gõ tiếng Việt, false: tiếng Anh)
    pub enabled: bool,
    /// Kiểm tra chính tả tiếng Việt
    pub check_spelling: bool,
    /// Tự động khôi phục phím nếu gõ sai chính tả
    pub restore_on_wrong_spelling: bool,
    /// Tự động viết hoa chữ cái đầu câu sau dấu chấm / Enter
    pub auto_uppercase_first_char: bool,
    /// Gõ phụ âm nhanh đầu từ: f->ph, j->gi, w->qu
    pub quick_start_consonant: bool,
    /// Gõ phụ âm nhanh cuối từ: g->ng, h->nh, k->ch
    pub quick_end_consonant: bool,
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
    /// Tự động khởi động cùng Windows
    pub autostart: bool,
    /// Bật chế độ debug log (ghi nhận vết từng phím, trạng thái state machine và output để chẩn đoán lỗi)
    pub debug: bool,
    /// Đường dẫn file lưu nhật ký debug log trên ổ đĩa
    pub debug_file_path: Option<String>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            method: InputMethod::Telex,
            enabled: true,
            check_spelling: true,
            restore_on_wrong_spelling: true,
            auto_uppercase_first_char: false,
            quick_start_consonant: false,
            quick_end_consonant: false,
            bracket_w: true,
            use_macro: false,
            use_macro_in_english_mode: true,
            remember_history_across_space: true,
            switch_with_ctrl_shift: true,
            autostart: false,
            debug: false,
            debug_file_path: Some("mkey_debug.log".to_string()),
        }
    }
}
