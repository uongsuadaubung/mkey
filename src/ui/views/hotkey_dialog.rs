//! Modal Dialog for capturing and customizing typing mode switch hotkey

use crate::engine::config_store;
use crate::platform::win32::{
    ENGINE_INSTANCE, WM_HOTKEY_CAPTURED, clear_captured_hotkey, get_captured_display_text,
    get_captured_hotkey, start_hotkey_capture, stop_hotkey_capture,
};
use crate::ui::UI_MANAGER;
use crate::ui::colors::ThemePalette;
use crate::ui::components::window::*;
use crate::ui::components::*;
use crate::ui::theme::{get_input_bg_brush, get_tab_card_brush, is_current_dark};
use std::sync::atomic::{AtomicIsize, Ordering};

pub const IDC_BTN_HOTKEY_SAVE: u32 = 601;
pub const IDC_BTN_HOTKEY_CLEAR: u32 = 602;
pub const IDC_BTN_HOTKEY_CANCEL: u32 = 603;
pub const IDC_LABEL_HOTKEY_DISPLAY: u32 = 604;

static DIALOG_PARENT_HWND: AtomicIsize = AtomicIsize::new(0);
static LABEL_DISPLAY_HWND: AtomicIsize = AtomicIsize::new(0);

/// Displays the modal Hotkey Capture Dialog centered over the parent control panel
pub fn show_hotkey_dialog(parent: isize) {
    let class_name = to_wide("MKeyHotkeyDialogClass");

    unsafe {
        let mut wc: WNDCLASSEXW = std::mem::zeroed();
        wc.cb_size = std::mem::size_of::<WNDCLASSEXW>() as u32;
        wc.lpfn_wnd_proc = Some(hotkey_dialog_wnd_proc);
        wc.h_instance = 0;
        wc.h_cursor = LoadCursorW(0, 32512); // IDC_ARROW
        wc.lpsz_class_name = class_name.as_ptr();
        RegisterClassExW(&wc);

        DIALOG_PARENT_HWND.store(parent, Ordering::SeqCst);

        let width = 380;
        let height = 210;

        let mut rc: RECT = std::mem::zeroed();
        GetWindowRect(parent, &mut rc);
        let x = rc.left + ((rc.right - rc.left) - width) / 2;
        let y = rc.top + ((rc.bottom - rc.top) - height) / 2;

        let strings = crate::language::current();
        let title = to_wide(strings.hotkey_dialog_title);

        // Disable parent to make dialog modal
        EnableWindow(parent, 0);

        let hwnd = CreateWindowExW(
            0x00010000 | 0x00000001, // WS_EX_CONTROLPARENT | WS_EX_DLGMODALFRAME
            class_name.as_ptr(),
            title.as_ptr(),
            0x00080000 | 0x00020000 | 0x10000000 | 0x00C00000, // WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE | WS_CAPTION
            x,
            y,
            width,
            height,
            parent,
            0,
            0,
            std::ptr::null_mut(),
        );

        if hwnd != 0 {
            SetForegroundWindow(hwnd);
        } else {
            EnableWindow(parent, 1);
        }
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn hotkey_dialog_wnd_proc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    const WM_CREATE: u32 = 0x0001;
    const WM_DESTROY: u32 = 0x0002;
    const WM_COMMAND: u32 = 0x0111;
    const WM_CTLCOLORSTATIC: u32 = 0x0138;
    const WM_PAINT: u32 = 0x000F;
    const WM_ERASEBKGND: u32 = 0x0014;
    const WM_CLOSE: u32 = 0x0010;

    unsafe {
        match msg {
            WM_CREATE => {
                let strings = crate::language::current();

                // Instruction prompt label
                Label::create(hwnd, strings.hotkey_dialog_prompt, 25, 18, 330, 20, 0);

                // Large hotkey display box
                let cur_hotkey_text = {
                    let guard = ENGINE_INSTANCE.lock().unwrap();
                    guard
                        .as_ref()
                        .map(|e| e.config().switch_key.display_text())
                        .unwrap_or_else(|| "Ctrl + Shift".to_string())
                };

                let label_disp = Label::create_with_id(
                    hwnd,
                    IDC_LABEL_HOTKEY_DISPLAY,
                    &format!("  [  {}  ]", cur_hotkey_text),
                    25,
                    48,
                    315,
                    36,
                    0,
                );
                if let Some(lbl) = label_disp {
                    LABEL_DISPLAY_HWND.store(lbl.hwnd(), Ordering::SeqCst);
                }

                // Buttons: [ Lưu ] [ Xóa ] [ Hủy ]
                PushButton::create(
                    hwnd,
                    IDC_BTN_HOTKEY_SAVE,
                    strings.btn_save,
                    35,
                    108,
                    85,
                    30,
                    0,
                );

                PushButton::create(
                    hwnd,
                    IDC_BTN_HOTKEY_CLEAR,
                    strings.btn_clear,
                    140,
                    108,
                    85,
                    30,
                    0,
                );

                PushButton::create(
                    hwnd,
                    IDC_BTN_HOTKEY_CANCEL,
                    strings.btn_cancel,
                    245,
                    108,
                    85,
                    30,
                    0,
                );

                start_hotkey_capture(hwnd);
                0
            }
            WM_HOTKEY_CAPTURED => {
                let text = get_captured_display_text();
                let disp_hwnd = LABEL_DISPLAY_HWND.load(Ordering::SeqCst);
                if disp_hwnd != 0 && !text.is_empty() {
                    let wide = to_wide(&format!("  [  {}  ]", text));
                    SetWindowTextW(disp_hwnd, wide.as_ptr());
                }
                0
            }
            WM_COMMAND => {
                let control_id = (wparam & 0xFFFF) as u32;
                match control_id {
                    IDC_BTN_HOTKEY_CLEAR => {
                        clear_captured_hotkey();
                        let strings = crate::language::current();
                        let text = to_wide(&format!("  [ {} ]", strings.hotkey_press_prompt));
                        let disp_hwnd = LABEL_DISPLAY_HWND.load(Ordering::SeqCst);
                        if disp_hwnd != 0 {
                            SetWindowTextW(disp_hwnd, text.as_ptr());
                        }
                    }
                    IDC_BTN_HOTKEY_SAVE => {
                        // If no key was captured (or cleared), revert to default Ctrl + Shift
                        let final_hotkey = get_captured_hotkey().unwrap_or_default();

                        if let Ok(mut guard) = ENGINE_INSTANCE.lock()
                            && let Some(ref mut engine) = *guard
                        {
                            engine.config_mut().switch_key = final_hotkey;
                            engine.config_mut().switch_with_ctrl_shift =
                                final_hotkey.is_ctrl_shift();
                            config_store::save_config_and_macros_debounced(
                                engine.config(),
                                &engine.macro_table,
                            );
                            println!(
                                "[MKey] >> Đã cập nhật phím chuyển chế độ: {}",
                                final_hotkey.display_text()
                            );
                        }

                        if let Ok(ui_guard) = UI_MANAGER.try_lock()
                            && let Some(ref ui) = *ui_guard
                        {
                            ui.controls
                                .header
                                .btn_switch_key
                                .set_text(&final_hotkey.display_text());
                        }

                        stop_hotkey_capture();
                        DestroyWindow(hwnd);
                    }
                    IDC_BTN_HOTKEY_CANCEL => {
                        stop_hotkey_capture();
                        DestroyWindow(hwnd);
                    }
                    _ => {}
                }
                0
            }
            WM_CLOSE => {
                stop_hotkey_capture();
                DestroyWindow(hwnd);
                0
            }
            WM_DESTROY => {
                stop_hotkey_capture();
                let parent = DIALOG_PARENT_HWND.swap(0, Ordering::SeqCst);
                if parent != 0 {
                    EnableWindow(parent, 1);
                    SetForegroundWindow(parent);
                }
                LABEL_DISPLAY_HWND.store(0, Ordering::SeqCst);
                0
            }
            WM_ERASEBKGND => 1,
            WM_PAINT => {
                let mut ps: PAINTSTRUCT = std::mem::zeroed();
                let hdc = BeginPaint(hwnd, &mut ps);
                if hdc != 0 {
                    let is_dark = is_current_dark();
                    let palette = ThemePalette::get(is_dark);
                    let brush = CreateSolidBrush(palette.bg_window);
                    let mut rc: RECT = std::mem::zeroed();
                    GetClientRect(hwnd, &mut rc);
                    FillRect(hdc, &rc, brush);
                    DeleteObject(brush);
                    EndPaint(hwnd, &ps);
                }
                0
            }
            WM_CTLCOLORSTATIC => {
                let hdc = wparam as isize;
                let hwnd_ctrl = lparam;
                let is_dark = is_current_dark();
                let palette = ThemePalette::get(is_dark);

                if hwnd_ctrl == LABEL_DISPLAY_HWND.load(Ordering::SeqCst) {
                    SetBkMode(hdc, 2); // OPAQUE
                    SetBkColor(hdc, palette.bg_input);
                    SetTextColor(hdc, palette.text_link);
                    let input_br = get_input_bg_brush();
                    if input_br != 0 {
                        return input_br;
                    }
                } else {
                    SetBkMode(hdc, TRANSPARENT);
                    SetBkColor(hdc, palette.bg_window);
                    SetTextColor(hdc, palette.text_primary);
                    let card_br = get_tab_card_brush();
                    if card_br != 0 {
                        return card_br;
                    }
                }
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

#[link(name = "user32")]
unsafe extern "system" {
    fn GetWindowRect(hWnd: isize, lpRect: *mut RECT) -> i32;
    fn GetClientRect(hWnd: isize, lpRect: *mut RECT) -> i32;
    fn FillRect(hDC: isize, lprc: *const RECT, hbr: isize) -> i32;
}
