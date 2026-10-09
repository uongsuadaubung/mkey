//! Native Win32 CheckBox Component
//! Custom-subclassed for perfect Dark and Light theme integration,
//! crisp primary text color, custom Fluent 18x18 checkbox, and exact vertical centering.

use super::window::*;
use std::ptr::null_mut;

const CHECKBOX_SUBCLASS_ID: usize = 0x408;

const WS_CHILD: u32 = 0x40000000;
const WS_VISIBLE: u32 = 0x10000000;
const WS_TABSTOP: u32 = 0x00010000;
const BS_AUTOCHECKBOX: u32 = 0x00000003;
const BM_GETCHECK: u32 = 0x00F0;
const BM_SETCHECK: u32 = 0x00F1;
const BST_CHECKED: usize = 0x0001;
const BST_UNCHECKED: usize = 0x0000;
const WM_SETFONT: u32 = 0x0030;

#[link(name = "user32")]
unsafe extern "system" {
    fn TrackMouseEvent(lp_event_track: *mut TRACKMOUSEEVENT) -> i32;
}

#[repr(C)]
struct TRACKMOUSEEVENT {
    cb_size: u32,
    dw_flags: u32,
    hwnd_track: isize,
    dw_hover_time: u32,
}
const TME_LEAVE: u32 = 0x00000002;

struct CheckBoxData {
    is_hover: bool,
}

unsafe extern "system" fn checkbox_subclass_proc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
    uid_subclass: usize,
    ref_data: usize,
) -> isize {
    const WM_PAINT: u32 = 0x000F;
    const WM_ERASEBKGND: u32 = 0x0014;
    const WM_NCDESTROY: u32 = 0x0082;
    const WM_LBUTTONUP: u32 = 0x0202;
    const WM_KEYUP: u32 = 0x0101;
    const WM_MOUSEMOVE: u32 = 0x0200;
    const WM_MOUSELEAVE: u32 = 0x02A3;
    const DT_LEFT: u32 = 0x00000000;
    const DT_VCENTER: u32 = 0x00000004;
    const DT_SINGLELINE: u32 = 0x00000020;
    const PS_SOLID: i32 = 0;
    const TRANSPARENT: i32 = 1;

    if msg == WM_NCDESTROY {
        if ref_data != 0 {
            unsafe {
                RemoveWindowSubclass(hwnd, checkbox_subclass_proc, uid_subclass);
                drop(Box::from_raw(ref_data as *mut CheckBoxData));
            }
        }
        return unsafe { DefSubclassProc(hwnd, msg, wparam, lparam) };
    }

    if msg == WM_ERASEBKGND {
        return 1;
    }

    let data_ptr = ref_data as *mut CheckBoxData;

    if msg == WM_MOUSEMOVE && !data_ptr.is_null() {
        let data = unsafe { &mut *data_ptr };
        if !data.is_hover {
            data.is_hover = true;
            let mut tme = TRACKMOUSEEVENT {
                cb_size: std::mem::size_of::<TRACKMOUSEEVENT>() as u32,
                dw_flags: TME_LEAVE,
                hwnd_track: hwnd,
                dw_hover_time: 0,
            };
            unsafe {
                TrackMouseEvent(&mut tme);
                InvalidateRect(hwnd, null_mut(), 0);
            }
        }
    } else if msg == WM_MOUSELEAVE && !data_ptr.is_null() {
        let data = unsafe { &mut *data_ptr };
        if data.is_hover {
            data.is_hover = false;
            unsafe {
                InvalidateRect(hwnd, null_mut(), 0);
            }
        }
    } else if msg == WM_LBUTTONUP || msg == WM_KEYUP || msg == BM_SETCHECK {
        let res = unsafe { DefSubclassProc(hwnd, msg, wparam, lparam) };
        unsafe {
            InvalidateRect(hwnd, null_mut(), 0);
        }
        return res;
    }

    if msg == WM_PAINT {
        unsafe {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            if hdc != 0 {
                let mut rc = RECT::default();
                GetClientRect(hwnd, &mut rc);

                let is_dark = crate::ui::is_current_dark();
                let palette = crate::ui::colors::ThemePalette::get(is_dark);
                let is_hover = if !data_ptr.is_null() {
                    (*data_ptr).is_hover
                } else {
                    false
                };

                // 1. Fill entire background seamlessly with the Card background
                let bg_brush = CreateSolidBrush(palette.bg_card);
                FillRect(hdc, &rc, bg_brush);
                DeleteObject(bg_brush);

                // 2. Check current checked / disabled state
                let is_checked = SendMessageW(hwnd, BM_GETCHECK, 0, 0) == (BST_CHECKED as isize);
                let style = GetWindowLongW(hwnd, GWL_STYLE) as u32;
                let is_disabled = (style & WS_DISABLED) != 0;

                // 3. Draw Checkbox Glyph - compact Fluent 13x13, precisely centered vertically
                let box_size = 13;
                let h = rc.bottom - rc.top;
                let box_top = (h - box_size) / 2;
                let box_rc = RECT {
                    left: 2,
                    top: box_top,
                    right: 2 + box_size,
                    bottom: box_top + box_size,
                };

                // Determine box fill & border colors based on state & theme
                let (glyph_bg, glyph_border) = if is_checked {
                    if is_hover && !is_disabled {
                        (0x00E68718, 0x00E68718) // Brighter Accent Blue on hover
                    } else if is_disabled {
                        (0x00888888, 0x00888888) // Muted gray when disabled
                    } else {
                        (0x00D77800, 0x00D77800) // Fluent Accent Blue (rgb 0, 120, 215)
                    }
                } else if is_disabled {
                    (palette.bg_input, palette.border)
                } else if is_dark {
                    if is_hover {
                        (0x00403A3A, 0x00C0B8B8) // Subtle lighter dark fill, brighter gray border
                    } else {
                        (0x00343030, 0x00888282) // Dark input fill, refined visible gray border
                    }
                } else {
                    // Light mode
                    if is_hover {
                        (0x00FAF5F5, 0x00605A5A) // Crisp dark border on hover
                    } else {
                        (0x00FFFFFF, 0x008E8888) // Pure white fill, balanced neutral border
                    }
                };

                let br = CreateSolidBrush(glyph_bg);
                let pen = CreatePen(PS_SOLID, 1, glyph_border);
                let ob = SelectObject(hdc, br);
                let op = SelectObject(hdc, pen);

                // Rounded checkbox rectangle (2px smooth radius for 13x13 box)
                RoundRect(hdc, box_rc.left, box_rc.top, box_rc.right, box_rc.bottom, 2, 2);

                SelectObject(hdc, ob);
                SelectObject(hdc, op);
                DeleteObject(br);
                DeleteObject(pen);

                if is_checked {
                    // Draw crisp white checkmark inside 13x13 box
                    let pen_check = CreatePen(PS_SOLID, 2, 0x00FFFFFF);
                    let op_c = SelectObject(hdc, pen_check);
                    MoveToEx(hdc, box_rc.left + 2, box_rc.top + 6, null_mut());
                    LineTo(hdc, box_rc.left + 5, box_rc.top + 9);
                    LineTo(hdc, box_rc.left + 10, box_rc.top + 3);
                    SelectObject(hdc, op_c);
                    DeleteObject(pen_check);
                }

                // 4. Draw Label Text in theme's primary text color (aligned right next to check box)
                let mut text_buf = [0u16; 256];
                let text_len = GetWindowTextW(hwnd, text_buf.as_mut_ptr(), 256);
                if text_len > 0 {
                    let gap = 5; // Compact, clean spacing to label
                    let mut text_rc = RECT {
                        left: box_rc.right + gap,
                        top: 0,
                        right: rc.right,
                        bottom: rc.bottom,
                    };
                    SetBkMode(hdc, TRANSPARENT);
                    let text_color = if is_disabled {
                        palette.text_secondary
                    } else {
                        palette.text_primary
                    };
                    SetTextColor(hdc, text_color);

                    let hfont = SendMessageW(hwnd, WM_GETFONT, 0, 0);
                    let old_font = if hfont != 0 {
                        SelectObject(hdc, hfont)
                    } else {
                        0
                    };

                    DrawTextW(
                        hdc,
                        text_buf.as_ptr(),
                        text_len,
                        &mut text_rc,
                        DT_LEFT | DT_VCENTER | DT_SINGLELINE,
                    );

                    if old_font != 0 {
                        SelectObject(hdc, old_font);
                    }
                }

                EndPaint(hwnd, &ps);
            }
        }
        return 0;
    }

    unsafe { DefSubclassProc(hwnd, msg, wparam, lparam) }
}

pub struct CheckBox {
    hwnd: isize,
    id: u32,
}

impl CheckBox {
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
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_AUTOCHECKBOX,
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
            let data = Box::into_raw(Box::new(CheckBoxData { is_hover: false }));
            unsafe {
                SetWindowSubclass(hwnd, checkbox_subclass_proc, CHECKBOX_SUBCLASS_ID, data as usize);
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

    pub fn is_checked(&self) -> bool {
        let res = unsafe { SendMessageW(self.hwnd, BM_GETCHECK, 0, 0) };
        res == BST_CHECKED as isize
    }

    pub fn set_checked(&self, checked: bool) {
        let state = if checked { BST_CHECKED } else { BST_UNCHECKED };
        unsafe {
            SendMessageW(self.hwnd, BM_SETCHECK, state, 0);
            InvalidateRect(self.hwnd, null_mut(), 0);
        }
    }

    pub fn set_text(&self, text: &str) {
        let wide = to_wide(text);
        unsafe {
            SetWindowTextW(self.hwnd, wide.as_ptr());
            InvalidateRect(self.hwnd, null_mut(), 0);
        }
    }
}
