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
    check_switch_key: "Switch key:",

    // Navigation TabBar
    tab_typing: "Typing",
    tab_macro: "Macro",
    tab_system: "System",
    tab_about: "About",

    // Tab 0: Typing & Sound
    typing_title: "Typing Assistance & Spell Checking",
    check_restore_wrong: "Auto-restore keys on wrong spelling",
    check_auto_upper: "Auto-capitalize first letter of sentence",
    sound_title: "Mechanical Keyboard Sound Simulation",
    check_sound_enabled: "Enable mechanical keyboard sound",
    label_switch_type: "Switch type:",
    label_volume: "Volume:",
    btn_test_sound: "Test",

    // Tab 1: Macro
    check_use_macro: "Enable macro shortcut table",
    check_macro_in_english: "Allow shorthand in English mode",
    edit_macro_key_placeholder: "Shortcut",
    edit_macro_val_placeholder: "Replacement",
    btn_add_macro: "+ Add",
    btn_edit_macro: "Edit",
    btn_cancel_macro: "Cancel",
    btn_del_macro: "Delete",
    col_macro_key: "Shortcut",
    col_macro_val: "Replacement",
    col_macro_type: "Type",
    macro_type_normal: "Whole word",
    macro_type_start: "Start consonant",
    macro_type_end: "End consonant",

    // Tab 2: System
    sys_title: "Startup & Logging",
    check_autostart: "Start with Windows",
    check_show_dialog: "Show this dialog on startup",
    check_debug_log: "Enable diagnostic logging (Debug Log)",
    btn_open_log: "View log...",
    btn_check_update: "Check for Updates...",
    btn_checking_update: "Checking...",
    label_theme_title: "Appearance & Language",
    label_theme: "Theme:",
    theme_auto: "System (Auto)",
    theme_light: "Light",
    theme_dark: "Dark",
    label_language: "Display language:",

    // Tab 3: About
    about_title: "MKey - Vietnamese Input Engine",
    about_ver: concat!("Version: ", env!("CARGO_PKG_VERSION")),
    about_author: "Author: Manh Kien",
    about_email: "Email: manhkien13041997@gmail.com",
    about_github: "GitHub: https://github.com/uongsuadaubung/mkey",

    // Footer Buttons
    btn_exit: "Exit",
    btn_defaults: "Defaults",
    btn_close: "Close",

    // Hotkey Capture Dialog
    hotkey_dialog_title: "Change Switch Mode Shortcut",
    hotkey_dialog_prompt: "Press key combination to switch typing mode:",
    hotkey_press_prompt: "Press any key combination...",
    btn_save: "Save",
    btn_clear: "Clear",
    btn_cancel: "Cancel",

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
