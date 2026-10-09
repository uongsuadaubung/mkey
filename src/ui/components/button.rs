//! Native Win32 PushButton Component

use super::window::{InvalidateRect, to_wide};
use std::ptr::null_mut;

#[link(name = "user32")]
unsafe extern "system" {
    fn CreateWindowExW(
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
    fn SendMessageW(h_wnd: isize, msg: u32, w_param: usize, l_param: isize) -> isize;
    fn SetWindowTextW(h_wnd: isize, lp_string: *const u16) -> i32;
    fn EnableWindow(h_wnd: isize, b_enable: i32) -> i32;
    fn GetWindowLongW(h_wnd: isize, n_index: i32) -> i32;
}

const WS_CHILD: u32 = 0x40000000;
const WS_VISIBLE: u32 = 0x10000000;
const WS_TABSTOP: u32 = 0x00010000;
const WS_DISABLED: u32 = 0x08000000;
const BS_PUSHBUTTON: u32 = 0x00000000;
const WM_SETFONT: u32 = 0x0030;
const GWL_STYLE: i32 = -16;

pub struct PushButton {
    hwnd: isize,
    id: u32,
}

impl PushButton {
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        parent: isize,
        id: u32,
        text: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        hfont: isize,
    ) -> Option<Self> {
        let class_name = to_wide("BUTTON");
        let window_name = to_wide(text);

        let hwnd = unsafe {
            CreateWindowExW(
                0,
                class_name.as_ptr(),
                window_name.as_ptr(),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_PUSHBUTTON,
                x,
                y,
                width,
                height,
                parent,
                id as isize,
                0,
                null_mut(),
            )
        };

        if hwnd != 0 {
            if hfont != 0 {
                unsafe {
                    SendMessageW(hwnd, WM_SETFONT, hfont as usize, 1);
                }
            }
            Some(Self { hwnd, id })
        } else {
            None
        }
    }

    pub fn hwnd(&self) -> isize {
        self.hwnd
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn set_text(&self, text: &str) {
        let wide = to_wide(text);
        unsafe {
            SetWindowTextW(self.hwnd, wide.as_ptr());
        }
    }

    pub fn is_enabled(&self) -> bool {
        let style = unsafe { GetWindowLongW(self.hwnd, GWL_STYLE) } as u32;
        (style & WS_DISABLED) == 0
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            EnableWindow(self.hwnd, if enabled { 1 } else { 0 });
            InvalidateRect(self.hwnd, std::ptr::null(), 1);
        }
    }
}
