//! Native Win32 TabControl Component

use super::window::to_wide;
use std::ptr::null_mut;

#[repr(C)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
struct TCITEMW {
    mask: u32,
    dw_state: u32,
    dw_state_mask: u32,
    psz_text: *mut u16,
    cch_text_max: i32,
    i_image: i32,
    l_param: isize,
}

#[repr(C)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
struct INITCOMMONCONTROLSEX {
    dw_size: u32,
    dw_icc: u32,
}

#[link(name = "comctl32")]
#[link(name = "user32")]
unsafe extern "system" {
    fn InitCommonControlsEx(lp_init_ctrls: *const INITCOMMONCONTROLSEX) -> i32;
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

const ICC_TAB_CLASSES: u32 = 0x00000008;
const WS_CHILD: u32 = 0x40000000;
const WS_VISIBLE: u32 = 0x10000000;
const WS_CLIPSIBLINGS: u32 = 0x04000000;
const WS_TABSTOP: u32 = 0x00010000;

const TCM_FIRST: u32 = 0x1300;
const TCM_INSERTITEMW: u32 = TCM_FIRST + 62;
const TCM_GETCURSEL: u32 = TCM_FIRST + 11;
const TCM_SETCURSEL: u32 = TCM_FIRST + 12;
const TCIF_TEXT: u32 = 0x0001;
const WM_SETFONT: u32 = 0x0030;

pub struct TabControl {
    hwnd: isize,
    id: u32,
}

impl TabControl {
    pub fn create(
        parent: isize,
        id: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        hfont: isize,
    ) -> Option<Self> {
        let icex = INITCOMMONCONTROLSEX {
            dw_size: std::mem::size_of::<INITCOMMONCONTROLSEX>() as u32,
            dw_icc: ICC_TAB_CLASSES,
        };
        unsafe {
            InitCommonControlsEx(&icex);
        }

        let class_name = to_wide("SysTabControl32");
        let hwnd = unsafe {
            CreateWindowExW(
                0,
                class_name.as_ptr(),
                null_mut(),
                WS_CHILD | WS_VISIBLE | WS_CLIPSIBLINGS | WS_TABSTOP,
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

    pub fn insert_tab(&self, index: usize, text: &str) {
        let mut wide = to_wide(text);
        let mut item = TCITEMW {
            mask: TCIF_TEXT,
            dw_state: 0,
            dw_state_mask: 0,
            psz_text: wide.as_mut_ptr(),
            cch_text_max: wide.len() as i32,
            i_image: -1,
            l_param: 0,
        };
        unsafe {
            SendMessageW(
                self.hwnd,
                TCM_INSERTITEMW,
                index,
                &mut item as *mut _ as isize,
            );
        }
    }

    pub fn get_cur_sel(&self) -> usize {
        let sel = unsafe { SendMessageW(self.hwnd, TCM_GETCURSEL, 0, 0) };
        if sel < 0 { 0 } else { sel as usize }
    }

    pub fn set_cur_sel(&self, index: usize) {
        unsafe {
            SendMessageW(self.hwnd, TCM_SETCURSEL, index, 0);
        }
    }
}
