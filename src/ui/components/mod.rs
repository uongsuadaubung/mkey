//! UI Component Library for Pure Win32 API

pub mod button;
pub mod checkbox;
pub mod combobox;
pub mod group_box;
pub mod icon_loader;
pub mod listview;
pub mod menu;
pub mod slider;
pub mod tab_bar;
pub mod tab_control;
pub mod textbox;
pub mod tray_icon;
pub mod window;

pub use button::PushButton;
pub use checkbox::CheckBox;
pub use combobox::ComboBox;
pub use group_box::{GroupBox, Label};
pub use icon_loader::{
    get_small_icon_size, is_windows_dark_taskbar, load_icon_from_memory, safe_destroy_icon,
};
pub use listview::ListView;
pub use menu::PopupMenu;
pub use slider::Slider;
pub use tab_bar::TabBar;
pub use tab_control::TabControl;
pub use textbox::TextBox;
pub use tray_icon::TrayIcon;
pub use window::{
    CallWindowProcW, CreateSolidBrush, DefSubclassProc, DeleteObject, EnumChildWindows, FW_NORMAL,
    FW_SEMIBOLD, FillRect, FrameRect, GWLP_WNDPROC, GWL_STYLE, GetClassNameW, GetClientRect, GetDC,
    GetDlgCtrlID, GetFocus, GetWindowLongW, GetWindowRect, HBRUSH, HCURSOR, HDC, HFONT, HHOOK,
    HICON, HINSTANCE, HMENU, HMODULE, HWND, HWND_BOTTOM, HWND_TOP, ICON_BIG, ICON_SMALL,
    InvalidateRect, POINT, RDW_ALLCHILDREN, RDW_ERASE, RDW_FRAME, RDW_INVALIDATE, RDW_UPDATENOW,
    RECT, RedrawWindow, ReleaseDC, RemoveWindowSubclass, SW_HIDE, SW_SHOW, SWP_FRAMECHANGED,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW, ScreenToClient,
    SendMessageW, SetBkColor, SetBkMode, SetTextColor, SetWindowLongPtrW, SetWindowPos,
    SetWindowSubclass, SetWindowTextW, SetWindowTheme, TRANSPARENT, UpdateWindow, WM_SETICON,
    WS_CLIPCHILDREN, WS_EX_APPWINDOW, allow_window_dark_mode, apply_modern_window_styling,
    center_window, create_app_font, init_common_controls, set_preferred_app_mode, to_wide,

};
