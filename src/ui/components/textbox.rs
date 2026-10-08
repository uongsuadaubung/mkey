//! Native Win32 Edit (TextBox) Component

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
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
    fn GetWindowTextW(h_wnd: isize, lp_string: *mut u16, n_max_count: i32) -> i32;
    fn SetWindowTextW(h_wnd: isize, lp_string: *const u16) -> i32;
    fn GetWindowTextLengthW(h_wnd: isize) -> i32;
}

const WS_CHILD: u32 = 0x40000000;
const WS_VISIBLE: u32 = 0x10000000;
const WS_TABSTOP: u32 = 0x00010000;
const WS_BORDER: u32 = 0x00800000;
const WS_CLIPSIBLINGS: u32 = 0x04000000;
const ES_AUTOHSCROLL: u32 = 0x0080;
const WM_SETFONT: u32 = 0x0030;
const EM_SETCUEBANNER: u32 = 0x1501;

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

pub struct TextBox {
    hwnd: isize,
    id: u32,
}

impl TextBox {
    pub fn create(
        parent: isize,
        id: u32,
        initial_text: &str,
        cue_banner: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        hfont: isize,
    ) -> Option<Self> {
        let class_name = to_wide("EDIT");
        let window_name = to_wide(initial_text);

        let hwnd = unsafe {
            CreateWindowExW(
                0x00000200, // WS_EX_CLIENTEDGE
                class_name.as_ptr(),
                window_name.as_ptr(),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_BORDER | WS_CLIPSIBLINGS | ES_AUTOHSCROLL,
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

        if hfont != 0 {
            unsafe {
                SendMessageW(hwnd, WM_SETFONT, hfont as usize, 1);
            }
        }

        if !cue_banner.is_empty() {
            let cue_w = to_wide(cue_banner);
            unsafe {
                SendMessageW(hwnd, EM_SETCUEBANNER, 1, cue_w.as_ptr() as isize);
            }
        }

        Some(Self { hwnd, id })
    }

    pub fn hwnd(&self) -> isize {
        self.hwnd
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn get_text(&self) -> String {
        let len = unsafe { GetWindowTextLengthW(self.hwnd) };
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0u16; (len + 1) as usize];
        let copied = unsafe { GetWindowTextW(self.hwnd, buf.as_mut_ptr(), len + 1) };
        if copied > 0 {
            String::from_utf16_lossy(&buf[..copied as usize])
        } else {
            String::new()
        }
    }

    pub fn set_text(&self, text: &str) {
        let wide = to_wide(text);
        unsafe {
            SetWindowTextW(self.hwnd, wide.as_ptr());
        }
    }

    pub fn clear(&self) {
        self.set_text("");
    }
}

