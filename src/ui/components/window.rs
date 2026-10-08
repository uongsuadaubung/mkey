//! Native Win32 Window Component & Helper Functions

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

#[repr(C)]
pub struct WNDCLASSEXW {
    pub cb_size: u32,
    pub style: u32,
    pub lpfn_wnd_proc: Option<unsafe extern "system" fn(isize, u32, usize, isize) -> isize>,
    pub cb_cls_extra: i32,
    pub cb_wnd_extra: i32,
    pub h_instance: isize,
    pub h_icon: isize,
    pub h_cursor: isize,
    pub h_br_background: isize,
    pub lpsz_menu_name: *const u16,
    pub lpsz_class_name: *const u16,
    pub h_icon_sm: isize,
}

#[repr(C)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[link(name = "user32")]
#[link(name = "gdi32")]
unsafe extern "system" {
    pub fn DefWindowProcW(h_wnd: isize, msg: u32, w_param: usize, l_param: isize) -> isize;
    pub fn RegisterClassExW(lp_wcx: *const WNDCLASSEXW) -> u16;
    pub fn CreateWindowExW(
        dw_ex_style: u32,
        lp_class_name: *const u16,
        lp_window_name: *const u16,
        dw_style: u32,
        x: i32,
        y: i32,
        n_width: i32,
        n_height: i32,
        h_wnd_parent: isize,
        h_menu: isize,
        h_instance: isize,
        lp_param: *mut std::ffi::c_void,
    ) -> isize;
    pub fn DestroyWindow(h_wnd: isize) -> i32;
    pub fn ShowWindow(h_wnd: isize, n_cmd_show: i32) -> i32;
    pub fn SetForegroundWindow(h_wnd: isize) -> i32;
    pub fn GetSystemMetrics(n_index: i32) -> i32;
    pub fn CreateFontW(
        c_height: i32,
        c_width: i32,
        c_escapement: i32,
        c_orientation: i32,
        c_weight: i32,
        b_italic: u32,
        b_underline: u32,
        b_strike_out: u32,
        i_char_set: u32,
        i_out_precision: u32,
        i_clip_precision: u32,
        i_quality: u32,
        i_pitch_and_family: u32,
        psz_face_name: *const u16,
    ) -> isize;
    pub fn DeleteObject(ho: isize) -> i32;
    pub fn AdjustWindowRect(lp_rect: *mut RECT, dw_style: u32, b_menu: i32) -> i32;
    pub fn SetWindowPos(
        h_wnd: isize,
        h_wnd_insert_after: isize,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        u_flags: u32,
    ) -> i32;
    pub fn SetBkMode(hdc: isize, mode: i32) -> i32;
    pub fn SetTextColor(hdc: isize, color: u32) -> u32;
    pub fn CreateSolidBrush(cr_color: u32) -> isize;
    pub fn SendMessageW(h_wnd: isize, msg: u32, w_param: usize, l_param: isize) -> isize;
    pub fn InvalidateRect(h_wnd: isize, lp_rect: *const RECT, b_erase: i32) -> i32;
    pub fn UpdateWindow(h_wnd: isize) -> i32;
}

#[link(name = "dwmapi")]
unsafe extern "system" {
    pub fn DwmSetWindowAttribute(
        hwnd: isize,
        dw_attribute: u32,
        pv_attribute: *const std::ffi::c_void,
        cb_attribute: u32,
    ) -> i32;
}

#[link(name = "comctl32")]
unsafe extern "system" {
    pub fn InitCommonControlsEx(picce: *const INITCOMMONCONTROLSEX) -> i32;
}

#[repr(C)]
pub struct INITCOMMONCONTROLSEX {
    pub dw_size: u32,
    pub dw_icc: u32,
}

pub const ICC_STANDARD_CLASSES: u32 = 0x00004000;
pub const ICC_TAB_CLASSES: u32 = 0x00000008;

pub const DWMWA_USE_IMMERSIVE_DARK_MODE: u32 = 20;
pub const DWMWA_WINDOW_CORNER_PREFERENCE: u32 = 33;
pub const DWMWCP_ROUND: u32 = 2; // Windows 11 rounded corners

pub const TRANSPARENT: i32 = 1;
pub const OPAQUE: i32 = 2;

/// Applies Windows 11 native rounded corners and dark/light title bar
pub fn apply_modern_window_styling(hwnd: isize, is_dark: bool) {
    unsafe {
        let corner_pref: u32 = DWMWCP_ROUND;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            &corner_pref as *const u32 as *const _,
            std::mem::size_of::<u32>() as u32,
        );

        let dark_mode: u32 = if is_dark { 1 } else { 0 };
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE,
            &dark_mode as *const u32 as *const _,
            std::mem::size_of::<u32>() as u32,
        );
    }
}

pub const ICC_LISTVIEW_CLASSES: u32 = 0x00000001;

/// Ensures ComCtl32 v6 visual styles engine is fully initialized
pub fn init_common_controls() {
    let icce = INITCOMMONCONTROLSEX {
        dw_size: std::mem::size_of::<INITCOMMONCONTROLSEX>() as u32,
        dw_icc: ICC_STANDARD_CLASSES | ICC_TAB_CLASSES | ICC_LISTVIEW_CLASSES,
    };
    unsafe {
        InitCommonControlsEx(&icce);
    }
}

pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) | ((g as u32) << 8) | ((b as u32) << 16)
}

pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_CLIPCHILDREN: u32 = 0x02000000;
pub const WS_EX_DLGMODALFRAME: u32 = 0x00000001;
pub const WS_EX_CONTROLPARENT: u32 = 0x00010000;
pub const WS_EX_APPWINDOW: u32 = 0x00040000;

pub const WM_SETICON: u32 = 0x0080;
pub const ICON_SMALL: usize = 0;
pub const ICON_BIG: usize = 1;

pub const SW_HIDE: i32 = 0;
pub const SW_SHOW: i32 = 5;

pub const HWND_TOP: isize = 0;
pub const HWND_BOTTOM: isize = 1;
pub const SWP_NOSIZE: u32 = 0x0001;
pub const SWP_NOMOVE: u32 = 0x0002;
pub const SWP_NOACTIVATE: u32 = 0x0010;
pub const SWP_SHOWWINDOW: u32 = 0x0040;

pub const SM_CXSCREEN: i32 = 0;
pub const SM_CYSCREEN: i32 = 1;

pub const COLOR_WINDOW: isize = 5;
pub const COLOR_BTNFACE: isize = 15;

pub const CLEARTYPE_QUALITY: u32 = 5;
pub const DEFAULT_CHARSET: u32 = 1;
pub const FW_NORMAL: i32 = 400;
pub const FW_SEMIBOLD: i32 = 600;

pub fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

/// Creates modern Segoe UI font matching Windows 10/11 standards
pub fn create_app_font(point_size: i32, weight: i32) -> isize {
    let font_name = to_wide("Segoe UI");
    // Standard 96 DPI: 1 pt ≈ 1.33 px -> height = - MulDiv(point_size, 96, 72)
    let height = -((point_size * 96 + 36) / 72);
    unsafe {
        CreateFontW(
            height,
            0,
            0,
            0,
            weight,
            0,
            0,
            0,
            DEFAULT_CHARSET,
            0,
            0,
            CLEARTYPE_QUALITY,
            0,
            font_name.as_ptr(),
        )
    }
}

pub fn center_window(hwnd: isize, width: i32, height: i32) {
    unsafe {
        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);
        let x = (screen_w - width) / 2;
        let y = (screen_h - height) / 2;
        SetWindowPos(hwnd, 0, x, y, width, height, 0x0004 /* SWP_NOZORDER */ | 0x0040 /* SWP_SHOWWINDOW */);
    }
}
