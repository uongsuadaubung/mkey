//! Control colors and cursor handlers (Themes, custom brushes, hyperlinks)

use crate::ui::colors::ThemePalette;
use crate::ui::components::window::*;
use crate::ui::theme::{get_input_bg_brush, get_tab_card_brush, is_current_dark};
use crate::ui::views::{IDC_LABEL_EMAIL, IDC_LABEL_GITHUB};

const IDC_HAND: usize = 32649;

#[link(name = "user32")]
unsafe extern "system" {
    fn LoadCursorW(h_instance: isize, lp_cursor_name: usize) -> isize;
    fn SetCursor(h_cursor: isize) -> isize;
    fn GetClassNameW(hWnd: isize, lpClassName: *mut u16, nMaxCount: i32) -> i32;
}

pub fn handle_ctlcolor_edit_listbox(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize {
    let hdc = wparam as isize;
    unsafe {
        let palette = ThemePalette::get(is_current_dark());
        SetTextColor(hdc, palette.text_input);
        SetBkColor(hdc, palette.bg_input);
        let input_br = get_input_bg_brush();
        if input_br != 0 {
            return input_br;
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

pub fn handle_ctlcolor_static(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize {
    let hdc = wparam as isize;
    let hwnd_ctrl = lparam;
    unsafe {
        let palette = ThemePalette::get(is_current_dark());
        let mut class_buf = [0u16; 32];
        let len = GetClassNameW(hwnd_ctrl, class_buf.as_mut_ptr(), 32);
        let class_slice = if len > 0 {
            &class_buf[..len as usize]
        } else {
            &[]
        };

        if utf16_str_eq_ignore_case(class_slice, "combobox")
            || utf16_str_eq_ignore_case(class_slice, "edit")
        {
            SetTextColor(hdc, palette.text_input);
            SetBkColor(hdc, palette.bg_input);
            let input_br = get_input_bg_brush();
            if input_br != 0 {
                return input_br;
            }
        } else {
            SetBkMode(hdc, TRANSPARENT);
            SetBkColor(hdc, palette.bg_card);

            let ctrl_id = GetDlgCtrlID(hwnd_ctrl) as u32;
            SetTextColor(hdc, palette.text_color_for_ctrl(ctrl_id));

            let card_br = get_tab_card_brush();
            if card_br != 0 {
                return card_br;
            }
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

pub fn handle_setcursor(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize {
    let child_hwnd = wparam as isize;
    let ctrl_id = unsafe { GetDlgCtrlID(child_hwnd) as u32 };
    let is_link = ctrl_id == IDC_LABEL_EMAIL || ctrl_id == IDC_LABEL_GITHUB;

    if is_link {
        unsafe {
            SetCursor(LoadCursorW(0, IDC_HAND));
        }
        1
    } else {
        let mut class_buf = [0u16; 16];
        let len = unsafe { GetClassNameW(child_hwnd, class_buf.as_mut_ptr(), 16) };
        let class_slice = if len > 0 {
            &class_buf[..len as usize]
        } else {
            &[]
        };
        if utf16_str_eq_ignore_case(class_slice, "Edit") {
            unsafe {
                SetCursor(LoadCursorW(0, IDC_IBEAM));
            }
            1
        } else {
            unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
        }
    }
}

fn utf16_str_eq_ignore_case(slice: &[u16], ascii_str: &str) -> bool {
    if slice.len() != ascii_str.len() {
        return false;
    }
    slice
        .iter()
        .zip(ascii_str.bytes())
        .all(|(&u, b)| (u as u8).eq_ignore_ascii_case(&b))
}
