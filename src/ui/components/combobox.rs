//! Native Win32 ComboBox Component

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
}

const WS_CHILD: u32 = 0x40000000;
const WS_VISIBLE: u32 = 0x10000000;
const WS_TABSTOP: u32 = 0x00010000;
const CBS_DROPDOWNLIST: u32 = 0x0003;
const CBS_HASSTRINGS: u32 = 0x0200;
const WS_VSCROLL: u32 = 0x00200000;

const CB_ADDSTRING: u32 = 0x0143;
const CB_SETCURSEL: u32 = 0x014E;
const CB_GETCURSEL: u32 = 0x0147;
const CB_ERR: isize = -1;
const WM_SETFONT: u32 = 0x0030;

pub struct ComboBox {
    hwnd: isize,
    id: u32,
}

impl ComboBox {
    pub fn create(
        parent: isize,
        id: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        hfont: isize,
    ) -> Option<Self> {
        let class_name = to_wide("COMBOBOX");

        let hwnd = unsafe {
            CreateWindowExW(
                0,
                class_name.as_ptr(),
                null_mut(),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_VSCROLL | CBS_DROPDOWNLIST | CBS_HASSTRINGS,
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

    pub fn add_item(&self, text: &str) {
        let wide = to_wide(text);
        unsafe {
            SendMessageW(self.hwnd, CB_ADDSTRING, 0, wide.as_ptr() as isize);
        }
    }

    pub fn get_selected(&self) -> Option<usize> {
        let sel = unsafe { SendMessageW(self.hwnd, CB_GETCURSEL, 0, 0) };
        if sel != CB_ERR && sel >= 0 {
            Some(sel as usize)
        } else {
            None
        }
    }

    pub fn set_selected(&self, index: usize) {
        unsafe {
            SendMessageW(self.hwnd, CB_SETCURSEL, index, 0);
        }
    }
}
