//! Modern Segmented Pill TabBar Control
//! Native Win32 Fluent Design Segmented Control with smooth pill animations and hover states.

use crate::ui::colors::ThemePalette;
use crate::ui::components::window::*;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};

const WS_CHILD: u32 = 0x40000000;
const WS_VISIBLE: u32 = 0x10000000;
const WM_PAINT: u32 = 0x000F;
const WM_ERASEBKGND: u32 = 0x0014;
const WM_LBUTTONDOWN: u32 = 0x0201;
const WM_MOUSEMOVE: u32 = 0x0200;
const WM_MOUSELEAVE: u32 = 0x02A3;
const WM_SETCURSOR: u32 = 0x0020;
const WM_COMMAND: u32 = 0x0111;

static TAB_CUR_SEL: AtomicUsize = AtomicUsize::new(0);
static TAB_HOVER: AtomicIsize = AtomicIsize::new(-1);
static TAB_FONT_NORMAL: AtomicIsize = AtomicIsize::new(0);
static TAB_FONT_BOLD: AtomicIsize = AtomicIsize::new(0);
static TAB_PARENT: AtomicIsize = AtomicIsize::new(0);
static TAB_CONTROL_ID: AtomicUsize = AtomicUsize::new(0);
static TAB_TITLES: [&str; 4] = ["Bộ gõ", "Gõ tắt", "Hệ thống", "Thông tin"];

#[link(name = "user32")]
unsafe extern "system" {
    fn TrackMouseEvent(lp_event_track: *mut TRACKMOUSEEVENT) -> i32;
}

#[repr(C)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
struct TRACKMOUSEEVENT {
    cb_size: u32,
    dw_flags: u32,
    hwnd_track: isize,
    dw_hover_time: u32,
}
const TME_LEAVE: u32 = 0x00000002;

unsafe extern "system" fn tab_bar_wnd_proc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    match msg {
        WM_ERASEBKGND => 1,
        WM_MOUSEMOVE => {
            let mut tme = TRACKMOUSEEVENT {
                cb_size: std::mem::size_of::<TRACKMOUSEEVENT>() as u32,
                dw_flags: TME_LEAVE,
                hwnd_track: hwnd,
                dw_hover_time: 0,
            };
            unsafe {
                TrackMouseEvent(&mut tme);
                let mut rc = RECT::default();
                GetClientRect(hwnd, &mut rc);
                let w = rc.right - rc.left;
                if w > 0 {
                    let x = (lparam & 0xFFFF) as i32;
                    let tab_w = (w - 6) / 4;
                    let idx = ((x - 3) / tab_w.max(1)).clamp(0, 3) as isize;
                    if TAB_HOVER.swap(idx, Ordering::Relaxed) != idx {
                        InvalidateRect(hwnd, null_mut(), 0);
                    }
                }
            }
            0
        }
        WM_MOUSELEAVE => {
            if TAB_HOVER.swap(-1, Ordering::Relaxed) != -1 {
                unsafe {
                    InvalidateRect(hwnd, null_mut(), 0);
                }
            }
            0
        }
        WM_SETCURSOR => {
            unsafe {
                SetCursor(LoadCursorW(0, IDC_ARROW));
            }
            1
        }
        WM_LBUTTONDOWN => {
            unsafe {
                let mut rc = RECT::default();
                GetClientRect(hwnd, &mut rc);
                let w = rc.right - rc.left;
                if w > 0 {
                    let x = (lparam & 0xFFFF) as i32;
                    let tab_w = (w - 6) / 4;
                    let idx = ((x - 3) / tab_w.max(1)).clamp(0, 3) as usize;
                    if TAB_CUR_SEL.swap(idx, Ordering::Relaxed) != idx {
                        InvalidateRect(hwnd, null_mut(), 0);
                        let parent = TAB_PARENT.load(Ordering::Relaxed);
                        let id = TAB_CONTROL_ID.load(Ordering::Relaxed);
                        if parent != 0 {
                            SendMessageW(parent, WM_COMMAND, id & 0xFFFF, hwnd);
                        }
                    }
                }
            }
            0
        }
        WM_PAINT => {
            unsafe {
                let mut ps: PAINTSTRUCT = std::mem::zeroed();
                let hdc = BeginPaint(hwnd, &mut ps);
                if hdc != 0 {
                    let mut rc = RECT::default();
                    GetClientRect(hwnd, &mut rc);

                    let is_dark = crate::ui::is_current_dark();
                    let palette = ThemePalette::get(is_dark);
                    let cur_sel = TAB_CUR_SEL.load(Ordering::Relaxed);
                    let hover_idx = TAB_HOVER.load(Ordering::Relaxed);
                    let font_normal = TAB_FONT_NORMAL.load(Ordering::Relaxed);
                    let font_bold = TAB_FONT_BOLD.load(Ordering::Relaxed);

                    // 0. Fill the entire control rect with the window background so corners outside the pill blend invisibly
                    let brush_bg = CreateSolidBrush(palette.bg_window);
                    FillRect(hdc, &rc, brush_bg);
                    DeleteObject(brush_bg);

                    // 1. Paint Outer Segmented Track Pill
                    let brush_track = CreateSolidBrush(palette.tab_track_bg);
                    let pen_track = CreatePen(PS_SOLID, 1, palette.tab_track_border);
                    let old_brush = SelectObject(hdc, brush_track);
                    let old_pen = SelectObject(hdc, pen_track);

                    RoundRect(hdc, rc.left, rc.top, rc.right, rc.bottom, 12, 12);

                    SelectObject(hdc, old_brush);
                    SelectObject(hdc, old_pen);
                    DeleteObject(brush_track);
                    DeleteObject(pen_track);

                    // 2. Draw Tabs
                    let num_tabs = TAB_TITLES.len() as i32;
                    let track_padding = 3;
                    let available_w = (rc.right - rc.left) - (track_padding * 2);
                    let tab_w = available_w / num_tabs;

                    SetBkMode(hdc, TRANSPARENT);

                    for (i, title) in TAB_TITLES.iter().enumerate() {
                        let i_i32 = i as i32;
                        let item_left = track_padding + i_i32 * tab_w;
                        let item_right = if i == TAB_TITLES.len() - 1 {
                            rc.right - track_padding
                        } else {
                            item_left + tab_w
                        };
                        let item_top = track_padding;
                        let item_bottom = rc.bottom - track_padding;

                        let mut item_rc = RECT {
                            left: item_left,
                            top: item_top,
                            right: item_right,
                            bottom: item_bottom,
                        };

                        if i == cur_sel {
                            // Active Tab Pill
                            let h_active_br = CreateSolidBrush(palette.tab_active_bg);
                            let h_active_pen = CreatePen(PS_SOLID, 1, palette.tab_active_border);
                            let ob = SelectObject(hdc, h_active_br);
                            let op = SelectObject(hdc, h_active_pen);

                            RoundRect(
                                hdc,
                                item_rc.left,
                                item_rc.top,
                                item_rc.right,
                                item_rc.bottom,
                                8,
                                8,
                            );

                            SelectObject(hdc, ob);
                            SelectObject(hdc, op);
                            DeleteObject(h_active_br);
                            DeleteObject(h_active_pen);

                            // Text
                            if font_bold != 0 {
                                SelectObject(hdc, font_bold);
                            }
                            SetTextColor(hdc, palette.text_primary);
                        } else {
                            // Inactive Tab (with subtle hover)
                            if hover_idx == i as isize {
                                let h_hvr_br = CreateSolidBrush(palette.tab_hover_bg);
                                let h_null_pen = CreatePen(PS_NULL, 0, 0);
                                let ob = SelectObject(hdc, h_hvr_br);
                                let op = SelectObject(hdc, h_null_pen);

                                RoundRect(
                                    hdc,
                                    item_rc.left,
                                    item_rc.top,
                                    item_rc.right,
                                    item_rc.bottom,
                                    8,
                                    8,
                                );

                                SelectObject(hdc, ob);
                                SelectObject(hdc, op);
                                DeleteObject(h_hvr_br);
                                DeleteObject(h_null_pen);
                            }

                            if font_normal != 0 {
                                SelectObject(hdc, font_normal);
                            }
                            SetTextColor(hdc, palette.text_secondary);
                        }

                        let title_w = to_wide(title);
                        DrawTextW(
                            hdc,
                            title_w.as_ptr(),
                            title.encode_utf16().count() as i32,
                            &mut item_rc,
                            DT_CENTER | DT_VCENTER | DT_SINGLELINE,
                        );
                    }

                    EndPaint(hwnd, &ps);
                }
            }
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

pub struct TabBar {
    hwnd: isize,
}

impl TabBar {
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        parent: isize,
        id: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        hfont_normal: isize,
        hfont_bold: isize,
    ) -> Option<Self> {
        let class_name = to_wide("MKeyTabBarClass");

        unsafe {
            TAB_CUR_SEL.store(0, Ordering::Relaxed);
            TAB_HOVER.store(-1, Ordering::Relaxed);
            TAB_FONT_NORMAL.store(hfont_normal, Ordering::Relaxed);
            TAB_FONT_BOLD.store(hfont_bold, Ordering::Relaxed);
            TAB_PARENT.store(parent, Ordering::Relaxed);
            TAB_CONTROL_ID.store(id as usize, Ordering::Relaxed);

            let mut wc: WNDCLASSEXW = std::mem::zeroed();
            wc.cb_size = std::mem::size_of::<WNDCLASSEXW>() as u32;
            wc.lpfn_wnd_proc = Some(tab_bar_wnd_proc);
            wc.lpsz_class_name = class_name.as_ptr();
            wc.h_cursor = LoadCursorW(0, IDC_ARROW);
            RegisterClassExW(&wc);

            let hwnd = CreateWindowExW(
                0,
                class_name.as_ptr(),
                to_wide("TabBar").as_ptr(),
                WS_CHILD | WS_VISIBLE,
                x,
                y,
                width,
                height,
                parent,
                id as isize,
                0,
                null_mut(),
            );

            if hwnd != 0 { Some(Self { hwnd }) } else { None }
        }
    }

    pub fn hwnd(&self) -> isize {
        self.hwnd
    }

    pub fn get_cur_sel(&self) -> usize {
        TAB_CUR_SEL.load(Ordering::Relaxed)
    }

    pub fn set_cur_sel(&self, index: usize) {
        if index < TAB_TITLES.len() {
            TAB_CUR_SEL.store(index, Ordering::Relaxed);
            unsafe {
                InvalidateRect(self.hwnd, null_mut(), 0);
            }
        }
    }
}
