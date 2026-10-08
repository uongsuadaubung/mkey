//! UI Component Library for Pure Win32 API

pub mod button;
pub mod checkbox;
pub mod combobox;
pub mod group_box;
pub mod icon_loader;
pub mod listview;
pub mod menu;
pub mod tab_control;
pub mod textbox;
pub mod tray_icon;
pub mod window;

pub use button::PushButton;
pub use checkbox::CheckBox;
pub use combobox::ComboBox;
pub use group_box::{GroupBox, Label};
pub use icon_loader::{get_small_icon_size, is_windows_dark_taskbar, load_icon_from_memory, safe_destroy_icon};
pub use listview::ListView;
pub use menu::PopupMenu;
pub use tab_control::TabControl;
pub use textbox::TextBox;
pub use tray_icon::TrayIcon;
pub use window::{
    apply_modern_window_styling, center_window, create_app_font, init_common_controls, rgb, to_wide,
    CreateSolidBrush, InvalidateRect, SendMessageW, SetBkMode, UpdateWindow, FW_NORMAL, FW_SEMIBOLD,
    ICON_BIG, ICON_SMALL, SW_HIDE, SW_SHOW, TRANSPARENT, WM_SETICON, WS_EX_APPWINDOW, WS_CLIPCHILDREN,
    SetWindowPos, HWND_TOP, HWND_BOTTOM, SWP_NOSIZE, SWP_NOMOVE, SWP_NOACTIVATE, SWP_SHOWWINDOW,
};

