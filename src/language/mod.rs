//! Multi-language & Localization (i18n) subsystem for MKey
//! Eliminates hardcoded UI strings and provides type-safe, zero-allocation localized texts.

pub mod en;
pub mod vi;

use std::fmt;
use std::str::FromStr;
use std::sync::atomic::{AtomicU8, Ordering};

/// Supported user interface languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    Vietnamese,
    English,
}

impl Language {
    /// Two-letter ISO code for configuration storage
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vietnamese => "vi",
            Self::English => "en",
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for Language {
    fn from(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "en" | "english" => Self::English,
            _ => Self::Vietnamese,
        }
    }
}

impl FromStr for Language {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl From<Language> for usize {
    fn from(l: Language) -> Self {
        match l {
            Language::Vietnamese => 0,
            Language::English => 1,
        }
    }
}

impl From<usize> for Language {
    fn from(idx: usize) -> Self {
        match idx {
            1 => Self::English,
            _ => Self::Vietnamese,
        }
    }
}

/// Comprehensive compile-time checked localized string dictionary for MKey UI
pub struct LanguageStrings {
    // Window & General
    pub window_title: &'static str,
    pub helper_window_title: &'static str,

    // Header Card
    pub label_method: &'static str,
    pub method_telex: &'static str,
    pub method_vni: &'static str,
    pub method_simple_telex: &'static str,
    pub label_mode: &'static str,
    pub mode_vietnamese: &'static str,
    pub mode_english: &'static str,
    pub label_switch: &'static str,
    pub check_ctrl_shift: &'static str,

    // Navigation TabBar
    pub tab_typing: &'static str,
    pub tab_macro: &'static str,
    pub tab_system: &'static str,
    pub tab_about: &'static str,

    // Tab 0: Typing
    pub typing_title: &'static str,
    pub check_restore_wrong: &'static str,
    pub check_auto_upper: &'static str,

    // Tab 1: Macro
    pub check_use_macro: &'static str,
    pub check_macro_in_english: &'static str,
    pub edit_macro_key_placeholder: &'static str,
    pub edit_macro_val_placeholder: &'static str,
    pub btn_add_macro: &'static str,
    pub btn_edit_macro: &'static str,
    pub btn_cancel_macro: &'static str,
    pub btn_del_macro: &'static str,
    pub col_macro_key: &'static str,
    pub col_macro_val: &'static str,
    pub col_macro_type: &'static str,
    pub macro_type_normal: &'static str,
    pub macro_type_start: &'static str,
    pub macro_type_end: &'static str,

    // Tab 2: System
    pub sys_title: &'static str,
    pub check_autostart: &'static str,
    pub check_show_dialog: &'static str,
    pub check_debug_log: &'static str,
    pub btn_open_log: &'static str,
    pub label_theme_title: &'static str,
    pub label_theme: &'static str,
    pub theme_auto: &'static str,
    pub theme_light: &'static str,
    pub theme_dark: &'static str,
    pub label_language: &'static str,

    // Tab 3: About
    pub about_title: &'static str,
    pub about_ver: &'static str,
    pub about_author: &'static str,
    pub about_email: &'static str,
    pub about_github: &'static str,

    // Footer Buttons
    pub btn_exit: &'static str,
    pub btn_defaults: &'static str,
    pub btn_close: &'static str,

    // System Tray Menu & Tooltips
    pub tray_tooltip_vi: &'static str,
    pub tray_tooltip_en: &'static str,
    pub tray_toggle_vi: &'static str,
    pub tray_method_telex: &'static str,
    pub tray_method_vni: &'static str,
    pub tray_method_simple_telex: &'static str,
    pub tray_control_panel: &'static str,
    pub tray_exit: &'static str,
}

impl LanguageStrings {
    pub const fn tab_titles(&self) -> [&'static str; 4] {
        [self.tab_typing, self.tab_macro, self.tab_system, self.tab_about]
    }
}

static CURRENT_LANG: AtomicU8 = AtomicU8::new(0);

/// Returns the static string dictionary for the given language
pub fn get_strings(lang: Language) -> &'static LanguageStrings {
    match lang {
        Language::Vietnamese => &vi::STRINGS,
        Language::English => &en::STRINGS,
    }
}

/// Gets the currently active language
pub fn current_language() -> Language {
    match CURRENT_LANG.load(Ordering::Relaxed) {
        1 => Language::English,
        _ => Language::Vietnamese,
    }
}

/// Sets the currently active language
pub fn set_current_language(lang: Language) {
    let val = match lang {
        Language::Vietnamese => 0,
        Language::English => 1,
    };
    CURRENT_LANG.store(val, Ordering::Relaxed);
}

/// Convenience helper to fetch the active localized strings
pub fn current() -> &'static LanguageStrings {
    get_strings(current_language())
}

