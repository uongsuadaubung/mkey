//! English localization for MKey

use super::LanguageStrings;

pub static STRINGS: LanguageStrings = LanguageStrings {
    // Window & General
    window_title: "MKey - Control Panel",
    helper_window_title: "MKeyHelper",

    // Header Card
    label_method: "Input Method:",
    method_telex: "Telex",
    method_vni: "VNI",
    method_simple_telex: "Simple Telex",
    label_mode: "Mode:",
    mode_vietnamese: "Vietnamese",
    mode_english: "English",
    label_switch: "Switch Key:",
    check_ctrl_shift: "Ctrl + Shift",

    // Navigation TabBar
    tab_typing: "Typing",
    tab_macro: "Macro",
    tab_system: "System",
    tab_about: "About",

    // Tab 0: Typing
    typing_title: "Typing Assistance & Spell Checking",
    check_spelling: "Enable Vietnamese spell checking",
    check_restore_wrong: "Auto-restore keys on wrong spelling",
    check_auto_upper: "Auto-capitalize first letter of sentence",

    // Tab 1: Macro
    check_use_macro: "Enable macro shortcut table",
    edit_macro_key_placeholder: "Shortcut",
    edit_macro_val_placeholder: "Replacement",
    btn_add_macro: "+ Add",
    btn_edit_macro: "Edit",
    btn_cancel_macro: "Cancel",
    btn_del_macro: "Delete",
    col_macro_key: "Shortcut",
    col_macro_val: "Replacement",

    // Tab 2: System
    sys_title: "Startup & Logging",
    check_autostart: "Start with Windows",
    check_show_dialog: "Show this dialog on startup",
    check_debug_log: "Enable diagnostic logging (Debug Log)",
    btn_open_log: "View log...",
    label_theme_title: "Appearance & Language",
    label_theme: "Theme:",
    theme_auto: "System (Auto)",
    theme_light: "Light",
    theme_dark: "Dark",
    label_language: "Display language:",

    // Tab 3: About
    about_title: "MKey - Vietnamese Input Engine",
    about_ver: "Version: 0.1.0",
    about_author: "Author: Manh Kien",
    about_email: "Email: manhkien13041997@gmail.com",
    about_github: "GitHub: https://github.com/uongsuadaubung/mkey",

    // Footer Buttons
    btn_exit: "Exit",
    btn_defaults: "Defaults",
    btn_close: "Close",

    // System Tray Menu & Tooltips
    tray_tooltip_vi: "MKey - Vietnamese IME (Vietnamese)",
    tray_tooltip_en: "MKey - Vietnamese IME (English)",
    tray_toggle_vi: "Vietnamese Mode",
    tray_method_telex: "Telex Input",
    tray_method_vni: "VNI Input",
    tray_method_simple_telex: "Simple Telex Input",
    tray_control_panel: "Control Panel...",
    tray_exit: "Exit",
};

