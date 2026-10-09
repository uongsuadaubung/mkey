//! Vietnamese (Tiếng Việt) localization for MKey

use super::LanguageStrings;

pub static STRINGS: LanguageStrings = LanguageStrings {
    // Window & General
    window_title: "MKey - Bảng điều khiển",
    helper_window_title: "MKeyHelper",

    // Header Card
    label_method: "Kiểu gõ:",
    method_telex: "Telex",
    method_vni: "VNI",
    method_simple_telex: "Simple Telex",
    label_mode: "Chế độ:",
    mode_vietnamese: "Tiếng Việt",
    mode_english: "Tiếng Anh",
    label_switch: "Phím chuyển:",
    check_ctrl_shift: "Ctrl + Shift",

    // Navigation TabBar
    tab_typing: "Bộ gõ",
    tab_macro: "Gõ tắt",
    tab_system: "Hệ thống",
    tab_about: "Thông tin",

    // Tab 0: Typing
    typing_title: "Tính năng hỗ trợ gõ & kiểm tra chính tả",
    check_spelling: "Bật kiểm tra chính tả tiếng Việt",
    check_restore_wrong: "Tự động khôi phục phím khi gõ sai từ",
    check_auto_upper: "Tự động viết hoa chữ cái đầu câu",

    // Tab 1: Macro
    check_use_macro: "Cho phép sử dụng bảng gõ tắt (Macro)",
    check_macro_in_english: "Cho phép gõ tắt cả khi ở chế độ tiếng Anh",
    edit_macro_key_placeholder: "Từ viết tắt",
    edit_macro_val_placeholder: "Cụm từ thay thế",
    btn_add_macro: "+ Thêm",
    btn_edit_macro: "Sửa",
    btn_cancel_macro: "Hủy",
    btn_del_macro: "Xóa",
    col_macro_key: "Từ viết tắt",
    col_macro_val: "Cụm từ thay thế",
    col_macro_type: "Phân loại",
    macro_type_normal: "Toàn từ",
    macro_type_start: "Phụ âm đầu",
    macro_type_end: "Phụ âm cuối",

    // Tab 2: System
    sys_title: "Khởi động & Nhật ký",
    check_autostart: "Khởi động cùng hệ điều hành Windows",
    check_show_dialog: "Bật hội thoại này khi khởi động",
    check_debug_log: "Bật ghi nhật ký chẩn đoán (Debug Log)",
    btn_open_log: "Xem log...",
    label_theme_title: "Giao diện & Ngôn ngữ",
    label_theme: "Giao diện:",
    theme_auto: "Theo hệ thống (Auto)",
    theme_light: "Sáng (Light)",
    theme_dark: "Tối (Dark)",
    label_language: "Ngôn ngữ hiển thị:",

    // Tab 3: About
    about_title: "MKey - Bộ gõ tiếng Việt",
    about_ver: "Phiên bản: 0.1.0",
    about_author: "Tác giả: Mạnh Kiên",
    about_email: "Email: manhkien13041997@gmail.com",
    about_github: "GitHub: https://github.com/uongsuadaubung/mkey",

    // Footer Buttons
    btn_exit: "Kết thúc",
    btn_defaults: "Mặc định",
    btn_close: "Đóng",

    // System Tray Menu & Tooltips
    tray_tooltip_vi: "MKey - Bộ gõ tiếng Việt (Tiếng Việt)",
    tray_tooltip_en: "MKey - Bộ gõ tiếng Việt (English)",
    tray_toggle_vi: "Bật tiếng Việt",
    tray_method_telex: "Kiểu gõ Telex",
    tray_method_vni: "Kiểu gõ VNI",
    tray_method_simple_telex: "Kiểu gõ Simple Telex",
    tray_control_panel: "Bảng điều khiển...",
    tray_exit: "Thoát",
};

