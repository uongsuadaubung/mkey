use crate::ui::components::window::*;
use std::ptr::null_mut;

struct EditSubclassData {
    cue: Vec<u16>,
    font: isize,
}

const EDIT_SUBCLASS_ID: usize = 0x407;

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

unsafe extern "system" fn edit_subclass_proc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
    uid_subclass: usize,
    ref_data: usize,
) -> isize {
    const WM_PAINT: u32 = 0x000F;
    const WM_SETFOCUS: u32 = 0x0007;
    const WM_KILLFOCUS: u32 = 0x0008;
    const WM_NCDESTROY: u32 = 0x0082;
    const WM_SET_CUE_BANNER: u32 = 0x8000 + 10;

    if msg == WM_NCDESTROY {
        if ref_data != 0 {
            unsafe {
                RemoveWindowSubclass(hwnd, edit_subclass_proc, uid_subclass);
                drop(Box::from_raw(ref_data as *mut EditSubclassData));
            }
        }
        return unsafe { DefSubclassProc(hwnd, msg, wparam, lparam) };
    }

    if msg == WM_SET_CUE_BANNER && ref_data != 0 {
        let text_ptr = lparam as *const u16;
        if !text_ptr.is_null() {
            let data = unsafe { &mut *(ref_data as *mut EditSubclassData) };
            let mut len = 0;
            while unsafe { *text_ptr.add(len) } != 0 {
                len += 1;
            }
            data.cue = unsafe { std::slice::from_raw_parts(text_ptr, len + 1) }.to_vec();
            unsafe {
                InvalidateRect(hwnd, null_mut(), 1);
            }
        }
        return 1;
    }

    let res = unsafe { DefSubclassProc(hwnd, msg, wparam, lparam) };

    if msg == WM_SETFOCUS || msg == WM_KILLFOCUS {
        unsafe {
            InvalidateRect(hwnd, null_mut(), 1);
        }
    } else if msg == WM_PAINT && ref_data != 0 {
        let data = unsafe { &*(ref_data as *const EditSubclassData) };
        if !data.cue.is_empty() {
            unsafe {
                if GetWindowTextLengthW(hwnd) == 0 && GetFocus() != hwnd {
                    let hdc = GetDC(hwnd);
                    if hdc != 0 {
                        let is_dark = crate::ui::is_current_dark();
                        let palette = crate::ui::colors::ThemePalette::get(is_dark);
                        let mut rc = RECT::default();
                        GetClientRect(hwnd, &mut rc);
                        rc.left += 6;
                        rc.top += 1;
                        SetBkMode(hdc, TRANSPARENT);
                        SetTextColor(hdc, palette.text_secondary);
                        if data.font != 0 {
                            SelectObject(hdc, data.font);
                        }
                        DrawTextW(
                            hdc,
                            data.cue.as_ptr(),
                            data.cue.len() as i32 - 1,
                            &mut rc,
                            DT_VCENTER | DT_SINGLELINE,
                        );
                        ReleaseDC(hwnd, hdc);
                    }
                }
            }
        }
    }

    res
}

pub struct TextBox {
    hwnd: isize,
    id: u32,
}

impl TextBox {
    #[allow(clippy::too_many_arguments)]
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
                0, // Flat modern border (no harsh 3D sunken WS_EX_CLIENTEDGE)
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

        let data = Box::into_raw(Box::new(EditSubclassData {
            cue: to_wide(cue_banner),
            font: hfont,
        }));
        unsafe {
            SetWindowSubclass(hwnd, edit_subclass_proc, EDIT_SUBCLASS_ID, data as usize);
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

    pub fn set_cue_banner(&self, text: &str) {
        const WM_SET_CUE_BANNER: u32 = 0x8000 + 10;
        let wide = to_wide(text);
        unsafe {
            SendMessageW(self.hwnd, WM_SET_CUE_BANNER, 0, wide.as_ptr() as isize);
        }
    }

    pub fn clear(&self) {
        self.set_text("");
    }
}
