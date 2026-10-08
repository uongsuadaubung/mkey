//! Native Win32 SysListView32 Report Component

use crate::ui::components::window::*;
use std::ptr::null_mut;

const LISTVIEW_SUBCLASS_ID: usize = 0x410;

#[repr(C)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
struct NMHDR {
    hwnd_from: isize,
    id_from: usize,
    code: u32,
}

#[repr(C)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
struct NMCUSTOMDRAW {
    hdr: NMHDR,
    dw_draw_stage: u32,
    hdc: isize,
    rc: RECT,
    dw_item_spec: usize,
    u_item_state: u32,
    l_item_lparam: isize,
}

unsafe extern "system" fn listview_subclass_proc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
    uid_subclass: usize,
    ref_data: usize,
) -> isize {
    const WM_NOTIFY: u32 = 0x004E;
    const WM_NCDESTROY: u32 = 0x0082;
    const NM_CUSTOMDRAW: u32 = 0xFFFFFFF4;
    const CDDS_PREPAINT: u32 = 0x00000001;
    const CDDS_ITEMPREPAINT: u32 = 0x00010001;
    const CDRF_NOTIFYITEMDRAW: isize = 0x00000020;
    const CDRF_SKIPDEFAULT: isize = 0x00000004;

    if msg == WM_NCDESTROY {
        unsafe {
            RemoveWindowSubclass(hwnd, listview_subclass_proc, uid_subclass);
            return DefSubclassProc(hwnd, msg, wparam, lparam);
        }
    }

    if msg == WM_NOTIFY {
        let nmhdr = unsafe { &*(lparam as *const NMHDR) };
        if nmhdr.code == NM_CUSTOMDRAW {
            let nmcd = unsafe { &mut *(lparam as *mut NMCUSTOMDRAW) };
            if nmcd.dw_draw_stage == CDDS_PREPAINT {
                return CDRF_NOTIFYITEMDRAW;
            } else if nmcd.dw_draw_stage == CDDS_ITEMPREPAINT {
                let is_dark = crate::ui::is_current_dark();
                let palette = crate::ui::colors::ThemePalette::get(is_dark);

                unsafe {
                    // 1. Fill background of header item
                    let bg_brush = CreateSolidBrush(palette.bg_card_header);
                    FillRect(nmcd.hdc, &nmcd.rc, bg_brush);
                    DeleteObject(bg_brush);

                    // 2. Draw subtle vertical column separator line on the right edge
                    let sep_rc = RECT {
                        left: nmcd.rc.right - 1,
                        top: nmcd.rc.top + 3,
                        right: nmcd.rc.right,
                        bottom: nmcd.rc.bottom - 3,
                    };
                    let sep_brush = CreateSolidBrush(palette.border_separator);
                    FillRect(nmcd.hdc, &sep_rc, sep_brush);
                    DeleteObject(sep_brush);

                    // 3. Draw column header title with crisp primary text color
                    let strings = crate::language::current();
                    let title = match nmcd.dw_item_spec {
                        0 => strings.col_macro_key,
                        1 => strings.col_macro_val,
                        _ => "",
                    };
                    let title_w = to_wide(title);
                    let mut text_rc = RECT {
                        left: nmcd.rc.left + 8,
                        top: nmcd.rc.top,
                        right: nmcd.rc.right - 8,
                        bottom: nmcd.rc.bottom,
                    };
                    SetBkMode(nmcd.hdc, TRANSPARENT);
                    SetTextColor(nmcd.hdc, palette.text_primary);
                    let font = ref_data as isize;
                    if font != 0 {
                        SelectObject(nmcd.hdc, font);
                    }
                    DrawTextW(
                        nmcd.hdc,
                        title_w.as_ptr(),
                        title_w.len() as i32 - 1,
                        &mut text_rc,
                        DT_VCENTER | DT_SINGLELINE,
                    );
                }

                return CDRF_SKIPDEFAULT;
            }
        }
    }

    unsafe { DefSubclassProc(hwnd, msg, wparam, lparam) }
}

#[repr(C)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
struct LVCOLUMNW {
    mask: u32,
    fmt: i32,
    cx: i32,
    psz_text: *mut u16,
    cch_text_max: i32,
    i_sub_item: i32,
    i_image: i32,
    i_order: i32,
    cx_min: i32,
    cx_default: i32,
    cx_ideal: i32,
}

#[repr(C)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
struct LVITEMW {
    mask: u32,
    i_item: i32,
    i_sub_item: i32,
    state: u32,
    state_mask: u32,
    psz_text: *mut u16,
    cch_text_max: i32,
    i_image: i32,
    l_param: isize,
    i_indent: i32,
    i_group_id: i32,
    c_columns: u32,
    pu_columns: *mut u32,
    pi_col_fmt: *mut i32,
    i_group: i32,
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

const ICC_LISTVIEW_CLASSES: u32 = 0x00000001;

const WS_CHILD: u32 = 0x40000000;
const WS_VISIBLE: u32 = 0x10000000;
const WS_BORDER: u32 = 0x00800000;
const WS_CLIPSIBLINGS: u32 = 0x04000000;
const WS_TABSTOP: u32 = 0x00010000;

const LVS_REPORT: u32 = 0x0001;
const LVS_SINGLESEL: u32 = 0x0004;
const LVS_SHOWSELALWAYS: u32 = 0x0008;

const LVM_FIRST: u32 = 0x1000;
const LVM_SETBKCOLOR: u32 = LVM_FIRST + 1;
const LVM_SETTEXTCOLOR: u32 = LVM_FIRST + 36;
const LVM_SETTEXTBKCOLOR: u32 = LVM_FIRST + 38;
const LVM_INSERTCOLUMNW: u32 = LVM_FIRST + 97;
const LVM_INSERTITEMW: u32 = LVM_FIRST + 77;
const LVM_SETITEMTEXTW: u32 = LVM_FIRST + 116;
const LVM_DELETEALLITEMS: u32 = LVM_FIRST + 9;
const LVM_DELETEITEM: u32 = LVM_FIRST + 8;
const LVM_GETNEXTITEM: u32 = LVM_FIRST + 12;
const LVM_SETEXTENDEDLISTVIEWSTYLE: u32 = LVM_FIRST + 54;

const LVS_EX_FULLROWSELECT: isize = 0x00000020;
const LVS_EX_DOUBLEBUFFER: isize = 0x00010000;

const LVCF_FMT: u32 = 0x0001;
const LVCF_WIDTH: u32 = 0x0002;
const LVCF_TEXT: u32 = 0x0004;
const LVCF_SUBITEM: u32 = 0x0008;
const LVCFMT_LEFT: i32 = 0;

const LVIF_TEXT: u32 = 0x0001;
const LVNI_SELECTED: isize = 2;

const WM_SETFONT: u32 = 0x0030;

pub struct ListView {
    hwnd: isize,
}

impl ListView {
    pub fn create(
        parent: isize,
        id: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        font: isize,
    ) -> Option<Self> {
        let icex = INITCOMMONCONTROLSEX {
            dw_size: std::mem::size_of::<INITCOMMONCONTROLSEX>() as u32,
            dw_icc: ICC_LISTVIEW_CLASSES,
        };
        unsafe {
            InitCommonControlsEx(&icex);
        }

        let class_name = to_wide("SysListView32");
        let style = WS_CHILD
            | WS_VISIBLE
            | WS_BORDER
            | WS_CLIPSIBLINGS
            | WS_TABSTOP
            | LVS_REPORT
            | LVS_SINGLESEL
            | LVS_SHOWSELALWAYS;

        let hwnd = unsafe {
            CreateWindowExW(
                0, // Flat modern border (no ugly 3D sunken WS_EX_CLIENTEDGE)
                class_name.as_ptr(),
                null_mut(),
                style,
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

        const LVM_GETHEADER: u32 = LVM_FIRST + 31;
        if font != 0 {
            unsafe {
                SendMessageW(hwnd, WM_SETFONT, font as usize, 1);
                let h_header = SendMessageW(hwnd, LVM_GETHEADER, 0, 0);
                if h_header != 0 {
                    SendMessageW(h_header, WM_SETFONT, font as usize, 1);
                }
            }
        }

        // Enable full row select and double buffering (remove harsh spreadsheet LVS_EX_GRIDLINES)
        unsafe {
            SendMessageW(
                hwnd,
                LVM_SETEXTENDEDLISTVIEWSTYLE,
                0,
                LVS_EX_FULLROWSELECT | LVS_EX_DOUBLEBUFFER,
            );

            SetWindowSubclass(
                hwnd,
                listview_subclass_proc,
                LISTVIEW_SUBCLASS_ID,
                font as usize,
            );
        }

        Some(Self { hwnd })
    }

    pub fn hwnd(&self) -> isize {
        self.hwnd
    }

    pub fn add_column(&self, index: i32, title: &str, width: i32) {
        let mut text = to_wide(title);
        let mut col = LVCOLUMNW {
            mask: LVCF_FMT | LVCF_WIDTH | LVCF_TEXT | LVCF_SUBITEM,
            fmt: LVCFMT_LEFT,
            cx: width,
            psz_text: text.as_mut_ptr(),
            cch_text_max: text.len() as i32,
            i_sub_item: index,
            i_image: 0,
            i_order: index,
            cx_min: 0,
            cx_default: 0,
            cx_ideal: 0,
        };
        unsafe {
            SendMessageW(
                self.hwnd,
                LVM_INSERTCOLUMNW,
                index as usize,
                &mut col as *mut _ as isize,
            );
        }
    }

    pub fn refresh_header(&self) {
        const LVM_GETHEADER: u32 = LVM_FIRST + 31;
        unsafe {
            let h_header = SendMessageW(self.hwnd, LVM_GETHEADER, 0, 0);
            if h_header != 0 {
                use crate::ui::components::window::{InvalidateRect, UpdateWindow};
                InvalidateRect(h_header, null_mut(), 1);
                UpdateWindow(h_header);
            }
        }
    }

    pub fn add_item(&self, index: i32, shortcut: &str, expansion: &str) {
        let mut text0 = to_wide(shortcut);
        let mut item = LVITEMW {
            mask: LVIF_TEXT,
            i_item: index,
            i_sub_item: 0,
            state: 0,
            state_mask: 0,
            psz_text: text0.as_mut_ptr(),
            cch_text_max: text0.len() as i32,
            i_image: 0,
            l_param: 0,
            i_indent: 0,
            i_group_id: 0,
            c_columns: 0,
            pu_columns: null_mut(),
            pi_col_fmt: null_mut(),
            i_group: 0,
        };
        unsafe {
            SendMessageW(self.hwnd, LVM_INSERTITEMW, 0, &mut item as *mut _ as isize);
        }

        let mut text1 = to_wide(expansion);
        let mut sub_item = LVITEMW {
            mask: LVIF_TEXT,
            i_item: index,
            i_sub_item: 1,
            state: 0,
            state_mask: 0,
            psz_text: text1.as_mut_ptr(),
            cch_text_max: text1.len() as i32,
            i_image: 0,
            l_param: 0,
            i_indent: 0,
            i_group_id: 0,
            c_columns: 0,
            pu_columns: null_mut(),
            pi_col_fmt: null_mut(),
            i_group: 0,
        };
        unsafe {
            SendMessageW(
                self.hwnd,
                LVM_SETITEMTEXTW,
                index as usize,
                &mut sub_item as *mut _ as isize,
            );
        }
    }

    pub fn clear(&self) {
        unsafe {
            SendMessageW(self.hwnd, LVM_DELETEALLITEMS, 0, 0);
        }
    }

    pub fn delete_item(&self, index: i32) {
        unsafe {
            SendMessageW(self.hwnd, LVM_DELETEITEM, index as usize, 0);
        }
    }

    pub fn clear_selection(&self) {
        const LVM_SETITEMSTATE: u32 = LVM_FIRST + 43;
        const LVIS_SELECTED: u32 = 0x0002;
        const LVIS_FOCUSED: u32 = 0x0001;

        let mut item = LVITEMW {
            mask: 0x0008, // LVIF_STATE
            i_item: -1,
            i_sub_item: 0,
            state: 0,
            state_mask: LVIS_SELECTED | LVIS_FOCUSED,
            psz_text: null_mut(),
            cch_text_max: 0,
            i_image: 0,
            l_param: 0,
            i_indent: 0,
            i_group_id: 0,
            c_columns: 0,
            pu_columns: null_mut(),
            pi_col_fmt: null_mut(),
            i_group: 0,
        };
        unsafe {
            SendMessageW(
                self.hwnd,
                LVM_SETITEMSTATE,
                usize::MAX,
                &mut item as *mut _ as isize,
            );
        }
    }

    pub fn get_selected_index(&self) -> Option<i32> {
        let idx = unsafe { SendMessageW(self.hwnd, LVM_GETNEXTITEM, usize::MAX, LVNI_SELECTED) };
        if idx >= 0 { Some(idx as i32) } else { None }
    }

    pub fn get_item_text(&self, index: i32, sub_item: i32) -> String {
        let mut buf = vec![0u16; 512];
        let mut item = LVITEMW {
            mask: 0,
            i_item: index,
            i_sub_item: sub_item,
            state: 0,
            state_mask: 0,
            psz_text: buf.as_mut_ptr(),
            cch_text_max: 512,
            i_image: 0,
            l_param: 0,
            i_indent: 0,
            i_group_id: 0,
            c_columns: 0,
            pu_columns: null_mut(),
            pi_col_fmt: null_mut(),
            i_group: 0,
        };
        const LVM_GETITEMTEXTW: u32 = LVM_FIRST + 115;
        let len = unsafe {
            SendMessageW(
                self.hwnd,
                LVM_GETITEMTEXTW,
                index as usize,
                &mut item as *mut _ as isize,
            )
        };
        if len > 0 {
            String::from_utf16_lossy(&buf[..len as usize])
        } else {
            String::new()
        }
    }

    pub fn apply_theme(&self, is_dark: bool) {
        let palette = crate::ui::colors::ThemePalette::get(is_dark);
        let lv_theme_name = if is_dark {
            "DarkMode_Explorer"
        } else {
            "Explorer"
        };
        let header_theme_name = if is_dark {
            "DarkMode_ItemsView"
        } else {
            "ItemsView"
        };
        const LVM_GETHEADER: u32 = LVM_FIRST + 31;
        const CLR_NONE: u32 = 0xFFFFFFFF;

        unsafe {
            crate::ui::components::allow_window_dark_mode(self.hwnd, is_dark);
            SendMessageW(self.hwnd, LVM_SETBKCOLOR, 0, palette.bg_input as isize);
            SendMessageW(self.hwnd, LVM_SETTEXTCOLOR, 0, palette.text_input as isize);
            SendMessageW(self.hwnd, LVM_SETTEXTBKCOLOR, 0, CLR_NONE as isize);
            crate::ui::components::SetWindowTheme(
                self.hwnd,
                to_wide(lv_theme_name).as_ptr(),
                null_mut(),
            );
            crate::ui::components::SetWindowPos(
                self.hwnd,
                0,
                0,
                0,
                0,
                0,
                crate::ui::components::SWP_NOMOVE
                    | crate::ui::components::SWP_NOSIZE
                    | crate::ui::components::SWP_NOZORDER
                    | crate::ui::components::SWP_FRAMECHANGED,
            );

            // Theme the Header Control (SysHeader32)
            let h_header = SendMessageW(self.hwnd, LVM_GETHEADER, 0, 0);
            if h_header != 0 {
                crate::ui::components::allow_window_dark_mode(h_header, is_dark);
                let res = crate::ui::components::SetWindowTheme(
                    h_header,
                    to_wide(header_theme_name).as_ptr(),
                    null_mut(),
                );
                if is_dark && res != 0 {
                    crate::ui::components::SetWindowTheme(
                        h_header,
                        to_wide("DarkMode_Explorer").as_ptr(),
                        null_mut(),
                    );
                }
                crate::ui::components::SetWindowPos(
                    h_header,
                    0,
                    0,
                    0,
                    0,
                    0,
                    crate::ui::components::SWP_NOMOVE
                        | crate::ui::components::SWP_NOSIZE
                        | crate::ui::components::SWP_NOZORDER
                        | crate::ui::components::SWP_FRAMECHANGED,
                );
                crate::ui::components::InvalidateRect(h_header, null_mut(), 1);
                crate::ui::components::UpdateWindow(h_header);
            }

            crate::ui::components::InvalidateRect(self.hwnd, null_mut(), 1);
            crate::ui::components::UpdateWindow(self.hwnd);
        }
    }
}
