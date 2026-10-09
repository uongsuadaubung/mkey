//! Native Win32 Trackbar / Slider Component
//! Uses msctls_trackbar32 from comctl32.dll for smooth native Windows slider controls.

use super::window::to_wide;
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
    fn EnableWindow(h_wnd: isize, b_enable: i32) -> i32;
    fn IsWindowEnabled(h_wnd: isize) -> i32;
}

const WS_CHILD: u32 = 0x40000000;
const WS_VISIBLE: u32 = 0x10000000;
const WS_TABSTOP: u32 = 0x00010000;
const TBS_HORZ: u32 = 0x0000;
const TBS_AUTOTICKS: u32 = 0x0001;

const TBM_GETPOS: u32 = 0x0400;
const TBM_SETRANGE: u32 = 0x0406;
const TBM_SETPOS: u32 = 0x0405;
const TBM_SETTICFREQ: u32 = 0x0414;

pub struct Slider {
    hwnd: isize,
    id: u32,
}

impl Slider {
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        parent: isize,
        id: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        min: u32,
        max: u32,
    ) -> Option<Self> {
        let class_name = to_wide("msctls_trackbar32");
        let hwnd = unsafe {
            CreateWindowExW(
                0,
                class_name.as_ptr(),
                null_mut(),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | TBS_HORZ | TBS_AUTOTICKS,
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

        if hwnd == 0 {
            return None;
        }

        unsafe {
            let range = ((max as isize) << 16) | (min as isize & 0xFFFF);
            SendMessageW(hwnd, TBM_SETRANGE, 1, range);
            SendMessageW(hwnd, TBM_SETTICFREQ, 10, 0);
        }

        Some(Self { hwnd, id })
    }

    #[inline]
    pub fn hwnd(&self) -> isize {
        self.hwnd
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn get_pos(&self) -> u32 {
        unsafe { (SendMessageW(self.hwnd, TBM_GETPOS, 0, 0) as u32).min(100) }
    }

    pub fn set_pos(&self, pos: u32) {
        unsafe {
            SendMessageW(self.hwnd, TBM_SETPOS, 1, pos as isize);
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            EnableWindow(self.hwnd, if enabled { 1 } else { 0 });
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { IsWindowEnabled(self.hwnd) != 0 }
    }
}

