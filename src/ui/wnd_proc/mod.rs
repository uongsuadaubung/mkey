//! MKey Win32 Window Procedures & Message Handlers

pub mod colors;
pub mod commands;
pub mod macro_events;
pub mod tray;

pub use tray::{handle_menu_command, tray_helper_wnd_proc};

use crate::platform::win32::ENGINE_INSTANCE;
use crate::ui::components::window::*;
use crate::ui::paint::paint_control_panel;
use crate::ui::theme::{apply_ui_theme, is_current_dark, resolve_is_dark};
use crate::ui::views::IDC_LIST_MACRO;
use crate::ui::UI_MANAGER;

const WM_PAINT: u32 = 0x000F;
const WM_CLOSE: u32 = 0x0010;
const WM_ERASEBKGND: u32 = 0x0014;
const WM_SHOWWINDOW: u32 = 0x0018;
const WM_SETTINGCHANGE: u32 = 0x001A;
const WM_SETCURSOR: u32 = 0x0020;
const WM_NOTIFY: u32 = 0x004E;
const WM_COMMAND: u32 = 0x0111;
const WM_HSCROLL: u32 = 0x0114;
const WM_CTLCOLOREDIT: u32 = 0x0133;
const WM_CTLCOLORLISTBOX: u32 = 0x0134;
const WM_CTLCOLORSTATIC: u32 = 0x0138;

#[repr(C)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
struct NMHDR {
    hwnd_from: isize,
    id_from: usize,
    code: u32,
}

/// Control Panel Dialog window procedure
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn control_panel_wnd_proc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    match msg {
        WM_ERASEBKGND => 1,
        WM_SHOWWINDOW => {
            if wparam != 0 {
                // Window is being shown -> refresh with latest engine config & ensure tab controls are visible
                if let Ok(ui_guard) = UI_MANAGER.try_lock()
                    && let Some(ref ui) = *ui_guard
                {
                    if let Ok(guard) = ENGINE_INSTANCE.lock()
                        && let Some(ref engine) = *guard
                    {
                        ui.controls.load_config(engine.config());
                    }
                    ui.controls
                        .update_tab_visibility(ui.controls.tab_bar.get_cur_sel());
                }
            }
            0
        }
        WM_PAINT => {
            unsafe {
                let mut ps: PAINTSTRUCT = std::mem::zeroed();
                let hdc = BeginPaint(hwnd, &mut ps);
                if hdc != 0 {
                    let is_dark = is_current_dark();
                    paint_control_panel(hwnd, hdc, is_dark);
                    EndPaint(hwnd, &ps);
                }
            }
            0
        }
        WM_SETTINGCHANGE => {
            let cfg_theme = {
                let guard = ENGINE_INSTANCE.lock().unwrap();
                guard
                    .as_ref()
                    .map(|e| e.config().theme)
                    .unwrap_or(crate::engine::config::UiTheme::Auto)
            };
            if cfg_theme == crate::engine::config::UiTheme::Auto
                && let Ok(ui_guard) = UI_MANAGER.try_lock()
                && let Some(ref ui) = *ui_guard
            {
                let is_dark = resolve_is_dark(cfg_theme);
                apply_ui_theme(ui.h_panel, &ui.controls, is_dark);
            }
            0
        }
        WM_HSCROLL => {
            commands::handle_hscroll(lparam);
            0
        }
        WM_COMMAND => commands::handle_command(hwnd, wparam),
        WM_NOTIFY => {
            let nmhdr = unsafe { &*(lparam as *const NMHDR) };
            if nmhdr.id_from == IDC_LIST_MACRO as usize
                && (nmhdr.code == 0xFFFFFFFE || nmhdr.code == 0xFFFFFFFD)
            {
                if let Ok(ui_guard) = UI_MANAGER.try_lock()
                    && let Some(ref ui) = *ui_guard
                {
                    macro_events::handle_macro_list_notify(ui);
                }
            }
            0
        }
        WM_CTLCOLOREDIT | WM_CTLCOLORLISTBOX => {
            colors::handle_ctlcolor_edit_listbox(hwnd, msg, wparam, lparam)
        }
        WM_CTLCOLORSTATIC => colors::handle_ctlcolor_static(hwnd, msg, wparam, lparam),
        WM_SETCURSOR => colors::handle_setcursor(hwnd, msg, wparam, lparam),
        WM_CLOSE => {
            unsafe {
                ShowWindow(hwnd, SW_HIDE);
            }
            crate::platform::win32::trim_working_set();
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

