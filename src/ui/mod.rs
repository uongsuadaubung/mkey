//! MKey Pure Win32 UI System
//! Fully componentized, lightweight, zero-dependency native Windows UI.

pub mod components;
pub mod views;

use std::ptr::null_mut;
use std::sync::Mutex;
use crate::engine::{config::InputMethod, EngineConfig};
use crate::ui::components::load_icon_from_memory;
use crate::ui::components::window::*;
use crate::ui::views::{
    ControlPanelControls, TrayHandler, IDC_BTN_ADD_MACRO, IDC_BTN_CANCEL_MACRO, IDC_BTN_CLOSE, IDC_BTN_DEFAULTS,
    IDC_BTN_DEL_MACRO, IDC_BTN_EDIT_MACRO, IDC_BTN_EXIT, IDC_BTN_OPEN_LOG, IDC_EDIT_MACRO_KEY,
    IDC_EDIT_MACRO_VALUE, IDC_LIST_MACRO, IDC_TAB_MAIN, IDM_CONTROL_PANEL, IDM_EXIT,
    IDM_SIMPLE_TELEX, IDM_TELEX, IDM_TOGGLE_VIET, IDM_VNI, WM_TRAY_MESSAGE,
};

static APP_ICO_BYTES: &[u8] = include_bytes!("../../assets/icon.ico");

#[repr(C)]
struct POINT {
    x: i32,
    y: i32,
}

#[repr(C)]
struct NMHDR {
    hwnd_from: isize,
    id_from: usize,
    code: u32,
}

#[link(name = "user32")]
unsafe extern "system" {
    fn GetCursorPos(lp_point: *mut POINT) -> i32;
    fn PostQuitMessage(n_exit_code: i32);
}

const WM_DESTROY: u32 = 0x0002;
const WM_CLOSE: u32 = 0x0010;
const WM_SETTINGCHANGE: u32 = 0x001A;
const WM_COMMAND: u32 = 0x0111;
const WM_NOTIFY: u32 = 0x004E;

const WM_LBUTTONUP: usize = 0x0202;
const WM_LBUTTONDBLCLK: usize = 0x0203;
const WM_RBUTTONUP: usize = 0x0205;

const TCN_FIRST: u32 = (0u32).wrapping_sub(550); // -550 = 0xFFFF_FDDA
const TCN_SELCHANGE: u32 = TCN_FIRST - 1;       // -551 = 0xFFFF_FDD9
const WM_CTLCOLORDLG: u32 = 0x0136;
const WM_CTLCOLORSTATIC: u32 = 0x0138;

static mut PANEL_BG_BRUSH: isize = 0;

pub static UI_MANAGER: Mutex<Option<UiState>> = Mutex::new(None);

pub struct UiState {
    pub h_helper: isize,
    pub h_panel: isize,
    pub tray: TrayHandler,
    pub controls: ControlPanelControls,
    pub hfont_normal: isize,
    pub hfont_bold: isize,
}

/// Helper window procedure for System Tray events
unsafe extern "system" fn tray_helper_wnd_proc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    match msg {
        WM_TRAY_MESSAGE => {
            let event = lparam as usize;
            if event == WM_LBUTTONUP {
                // Single click -> Toggle Vietnamese / English
                let is_viet = {
                    let mut guard = crate::platform::win32::ENGINE_INSTANCE.lock().unwrap();
                    if let Some(ref mut engine) = *guard {
                        engine.toggle_enabled()
                    } else {
                        false
                    }
                };
                if let Ok(mut ui_guard) = UI_MANAGER.lock() {
                    if let Some(ref mut ui) = *ui_guard {
                        ui.tray.set_vietnamese_mode(is_viet);
                    }
                }
            } else if event == WM_LBUTTONDBLCLK {
                // Double click -> Show Control Panel
                show_control_panel();
            } else if event == WM_RBUTTONUP {
                // Right click -> Context Menu
                let mut pt = POINT { x: 0, y: 0 };
                unsafe {
                    GetCursorPos(&mut pt);
                }

                let config = {
                    let guard = crate::platform::win32::ENGINE_INSTANCE.lock().unwrap();
                    if let Some(ref engine) = *guard {
                        engine.config().clone()
                    } else {
                        EngineConfig::default()
                    }
                };

                let cmd = if let Ok(mut ui_guard) = UI_MANAGER.lock() {
                    if let Some(ref mut ui) = *ui_guard {
                        ui.tray.show_context_menu(hwnd, pt.x, pt.y, &config)
                    } else {
                        0
                    }
                } else {
                    0
                };

                handle_menu_command(cmd);
            }
            0
        }
        WM_SETTINGCHANGE => {
            // Windows Theme changed (Light <-> Dark mode) -> auto update tray icon
            if let Ok(mut ui_guard) = UI_MANAGER.lock() {
                if let Some(ref mut ui) = *ui_guard {
                    ui.tray.refresh_icon();
                }
            }
            0
        }
        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

/// Control Panel Dialog window procedure
unsafe extern "system" fn control_panel_wnd_proc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    match msg {
        WM_COMMAND => {
            let control_id = (wparam & 0xFFFF) as u32;
            match control_id {
                IDC_BTN_EXIT => {
                    // Exit MKey completely
                    println!("[UI] Người dùng nhấn Kết thúc -> Thoát MKey");
                    unsafe {
                        PostQuitMessage(0);
                    }
                }
                IDC_BTN_CLOSE => {
                    // Hide window when clicking "Đóng"
                    unsafe {
                        ShowWindow(hwnd, SW_HIDE);
                    }
                }
                IDC_BTN_DEFAULTS => {
                    let def_config = EngineConfig::default();
                    if let Ok(mut ui_guard) = UI_MANAGER.lock() {
                        if let Some(ref mut ui) = *ui_guard {
                            ui.controls.load_config(&def_config);
                        }
                    }
                    if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock() {
                        if let Some(ref mut engine) = *guard {
                            *engine.config_mut() = def_config;
                            let _ = crate::engine::config_store::save_config_and_macros(engine.config(), &engine.macro_table);
                        }
                    }
                }
                IDC_EDIT_MACRO_KEY | IDC_EDIT_MACRO_VALUE => {
                    // Ignore text change notifications so they don't trigger sync to EngineConfig
                }
                IDC_BTN_ADD_MACRO => {
                    let pair = if let Ok(ui_guard) = UI_MANAGER.try_lock() {
                        ui_guard.as_ref().map(|ui| {
                            (
                                ui.controls.edit_macro_key.get_text().trim().to_string(),
                                ui.controls.edit_macro_value.get_text().trim().to_string(),
                            )
                        })
                    } else {
                        None
                    };

                    if let Some((key, val)) = pair {
                        if !key.is_empty() && !val.is_empty() {
                            let macros = if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock() {
                                if let Some(ref mut engine) = *guard {
                                    engine.macro_table.insert(&key, &val);
                                    let _ = crate::engine::config_store::save_config_and_macros(engine.config(), &engine.macro_table);
                                    println!("[MKey] Đã thêm gõ tắt: '{}' -> '{}'", key, val);
                                    engine.macro_table.get_sorted_entries()
                                } else {
                                    Vec::new()
                                }
                            } else {
                                Vec::new()
                            };

                            if let Ok(ui_guard) = UI_MANAGER.try_lock() {
                                if let Some(ref ui) = *ui_guard {
                                    ui.controls.populate_macros(&macros);
                                    ui.controls.edit_macro_key.clear();
                                    ui.controls.edit_macro_value.clear();
                                    ui.controls.list_macro.clear_selection();
                                    ui.controls.set_macro_edit_mode(false);
                                }
                            }
                        }
                    }
                }
                IDC_BTN_EDIT_MACRO => {
                    let pair = if let Ok(ui_guard) = UI_MANAGER.try_lock() {
                        ui_guard.as_ref().map(|ui| {
                            (
                                ui.controls.edit_macro_key.get_text().trim().to_string(),
                                ui.controls.edit_macro_value.get_text().trim().to_string(),
                            )
                        })
                    } else {
                        None
                    };

                    if let Some((key, val)) = pair {
                        if !key.is_empty() && !val.is_empty() {
                            let macros = if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock() {
                                if let Some(ref mut engine) = *guard {
                                    engine.macro_table.insert(&key, &val);
                                    let _ = crate::engine::config_store::save_config_and_macros(engine.config(), &engine.macro_table);
                                    println!("[MKey] Đã cập nhật gõ tắt: '{}' -> '{}'", key, val);
                                    engine.macro_table.get_sorted_entries()
                                } else {
                                    Vec::new()
                                }
                            } else {
                                Vec::new()
                            };

                            if let Ok(ui_guard) = UI_MANAGER.try_lock() {
                                if let Some(ref ui) = *ui_guard {
                                    ui.controls.populate_macros(&macros);
                                    ui.controls.edit_macro_key.clear();
                                    ui.controls.edit_macro_value.clear();
                                    ui.controls.list_macro.clear_selection();
                                    ui.controls.set_macro_edit_mode(false);
                                }
                            }
                        }
                    }
                }
                IDC_BTN_DEL_MACRO => {
                    let key_to_del = if let Ok(ui_guard) = UI_MANAGER.try_lock() {
                        if let Some(ref ui) = *ui_guard {
                            let key_input = ui.controls.edit_macro_key.get_text().trim().to_string();
                            if !key_input.is_empty() {
                                key_input
                            } else if let Some(sel_idx) = ui.controls.list_macro.get_selected_index() {
                                ui.controls.list_macro.get_item_text(sel_idx, 0)
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };

                    if !key_to_del.is_empty() {
                        let macros = if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock() {
                            if let Some(ref mut engine) = *guard {
                                if engine.macro_table.remove(&key_to_del).is_some() {
                                    let _ = crate::engine::config_store::save_config_and_macros(engine.config(), &engine.macro_table);
                                    println!("[MKey] Đã xóa từ gõ tắt: '{}'", key_to_del);
                                }
                                engine.macro_table.get_sorted_entries()
                            } else {
                                Vec::new()
                            }
                        } else {
                            Vec::new()
                        };

                        if let Ok(ui_guard) = UI_MANAGER.try_lock() {
                            if let Some(ref ui) = *ui_guard {
                                ui.controls.populate_macros(&macros);
                                ui.controls.edit_macro_key.clear();
                                ui.controls.edit_macro_value.clear();
                                ui.controls.list_macro.clear_selection();
                                ui.controls.set_macro_edit_mode(false);
                            }
                        }
                    }
                }
                IDC_BTN_CANCEL_MACRO => {
                    if let Ok(ui_guard) = UI_MANAGER.try_lock() {
                        if let Some(ref ui) = *ui_guard {
                            ui.controls.edit_macro_key.clear();
                            ui.controls.edit_macro_value.clear();
                            ui.controls.list_macro.clear_selection();
                            ui.controls.set_macro_edit_mode(false);
                        }
                    }
                }
                IDC_BTN_OPEN_LOG => {
                    let log_file = {
                        let guard = crate::platform::win32::ENGINE_INSTANCE.lock().unwrap();
                        guard
                            .as_ref()
                            .and_then(|e| e.config().debug_file_path.clone())
                            .unwrap_or_else(|| "mkey_debug.log".to_string())
                    };
                    if !std::path::Path::new(&log_file).exists() {
                        let _ = std::fs::File::create(&log_file);
                    }
                    let _ = std::process::Command::new("notepad.exe").arg(&log_file).spawn();
                }
                _ => {
                    // Checkbox or combobox toggled -> Sync to EngineConfig
                    let mut is_viet = true;
                    if let Ok(mut ui_guard) = UI_MANAGER.try_lock() {
                        if let Some(ref mut ui) = *ui_guard {
                            if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock() {
                                if let Some(ref mut engine) = *guard {
                                    ui.controls.read_config(engine.config_mut());
                                    let _ = crate::engine::config_store::save_config_and_macros(engine.config(), &engine.macro_table);
                                    is_viet = engine.config().enabled;
                                }
                            }
                        }
                    }
                    update_tray_icon(is_viet);
                }
            }
            0
        }
        WM_NOTIFY => {
            let nmhdr = unsafe { &*(lparam as *const NMHDR) };
            if nmhdr.id_from == IDC_TAB_MAIN as usize && nmhdr.code == TCN_SELCHANGE {
                if let Ok(ui_guard) = UI_MANAGER.try_lock() {
                    if let Some(ref ui) = *ui_guard {
                        let cur_tab = ui.controls.tab_control.get_cur_sel();
                        ui.controls.update_tab_visibility(cur_tab);
                    }
                }
            } else if nmhdr.id_from == IDC_LIST_MACRO as usize && (nmhdr.code == 0xFFFFFFFE || nmhdr.code == 0xFFFFFFFD) {
                // When clicking or selecting an item in the ListView, populate the edit boxes and switch to edit mode
                if let Ok(ui_guard) = UI_MANAGER.try_lock() {
                    if let Some(ref ui) = *ui_guard {
                        if let Some(sel_idx) = ui.controls.list_macro.get_selected_index() {
                            let k = ui.controls.list_macro.get_item_text(sel_idx, 0);
                            let v = ui.controls.list_macro.get_item_text(sel_idx, 1);
                            if !k.is_empty() {
                                ui.controls.edit_macro_key.set_text(&k);
                                ui.controls.edit_macro_value.set_text(&v);
                                ui.controls.set_macro_edit_mode(true);
                            }
                        }
                    }
                }
            }
            0
        }
        WM_CTLCOLORSTATIC | WM_CTLCOLORDLG => {
            let hdc = wparam as isize;
            unsafe {
                SetBkMode(hdc, TRANSPARENT);
                if PANEL_BG_BRUSH != 0 {
                    return PANEL_BG_BRUSH;
                }
            }
            unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
        }
        WM_CLOSE => {
            // Clicking 'X' button hides window into tray instead of exiting
            unsafe {
                ShowWindow(hwnd, SW_HIDE);
            }
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

pub fn show_control_panel() {
    let (cfg, macros) = {
        if let Ok(guard) = crate::platform::win32::ENGINE_INSTANCE.lock() {
            if let Some(ref engine) = *guard {
                (Some(engine.config().clone()), engine.macro_table.get_sorted_entries())
            } else {
                (None, Vec::new())
            }
        } else {
            (None, Vec::new())
        }
    };

    if let Ok(ui_guard) = UI_MANAGER.try_lock() {
        if let Some(ref ui) = *ui_guard {
            unsafe {
                if let Some(ref c) = cfg {
                    ui.controls.load_config(c);
                }
                ui.controls.populate_macros(&macros);
                ui.controls.update_tab_visibility(ui.controls.tab_control.get_cur_sel());
                let is_dark = crate::ui::components::is_windows_dark_taskbar();
                apply_modern_window_styling(ui.h_panel, is_dark);
                ShowWindow(ui.h_panel, SW_SHOW);
                SetForegroundWindow(ui.h_panel);
            }
        }
    }
}

pub fn update_tray_icon(is_vietnamese: bool) {
    if let Ok(mut ui_guard) = UI_MANAGER.lock() {
        if let Some(ref mut ui) = *ui_guard {
            ui.tray.set_vietnamese_mode(is_vietnamese);
            ui.controls.combo_mode.set_selected(if is_vietnamese { 0 } else { 1 });
        }
    }
}

fn handle_menu_command(cmd: u32) {
    match cmd {
        IDM_TOGGLE_VIET => {
            let is_viet = {
                let mut guard = crate::platform::win32::ENGINE_INSTANCE.lock().unwrap();
                if let Some(ref mut engine) = *guard {
                    let res = engine.toggle_enabled();
                    let _ = crate::engine::config_store::save_config_and_macros(engine.config(), &engine.macro_table);
                    res
                } else {
                    false
                }
            };
            update_tray_icon(is_viet);
        }
        IDM_TELEX => {
            if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock() {
                if let Some(ref mut engine) = *guard {
                    engine.config_mut().method = InputMethod::Telex;
                    let _ = crate::engine::config_store::save_config_and_macros(engine.config(), &engine.macro_table);
                }
            }
        }
        IDM_VNI => {
            if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock() {
                if let Some(ref mut engine) = *guard {
                    engine.config_mut().method = InputMethod::Vni;
                    let _ = crate::engine::config_store::save_config_and_macros(engine.config(), &engine.macro_table);
                }
            }
        }
        IDM_SIMPLE_TELEX => {
            if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock() {
                if let Some(ref mut engine) = *guard {
                    engine.config_mut().method = InputMethod::SimpleTelex1;
                    let _ = crate::engine::config_store::save_config_and_macros(engine.config(), &engine.macro_table);
                }
            }
        }
        IDM_CONTROL_PANEL => {
            show_control_panel();
        }
        IDM_EXIT => {
            unsafe {
                PostQuitMessage(0);
            }
        }
        _ => {}
    }
}

/// Initializes the complete Win32 UI system (Tray + Control Panel)
pub fn init_ui() -> bool {
    init_common_controls();

    let class_helper_name = to_wide("MKeyHelperClass");
    let class_panel_name = to_wide("MKeyControlPanelClass");

    let is_dark = crate::ui::components::is_windows_dark_taskbar();
    let bg_color = if is_dark { rgb(32, 32, 32) } else { rgb(250, 250, 250) };
    let h_bg_brush = unsafe { CreateSolidBrush(bg_color) };
    unsafe {
        PANEL_BG_BRUSH = h_bg_brush;
    }

    unsafe {
        // 1. Register Helper Window Class
        let mut wc_helper: WNDCLASSEXW = std::mem::zeroed();
        wc_helper.cb_size = std::mem::size_of::<WNDCLASSEXW>() as u32;
        wc_helper.lpfn_wnd_proc = Some(tray_helper_wnd_proc);
        wc_helper.lpsz_class_name = class_helper_name.as_ptr();
        RegisterClassExW(&wc_helper);

        // 2. Load Application Icons for Taskbar and Title Bar
        let hicon_big = load_icon_from_memory(APP_ICO_BYTES, 32, 32).unwrap_or(0);
        let hicon_sm = load_icon_from_memory(APP_ICO_BYTES, 16, 16).unwrap_or(0);

        // Register Control Panel Window Class
        let mut wc_panel: WNDCLASSEXW = std::mem::zeroed();
        wc_panel.cb_size = std::mem::size_of::<WNDCLASSEXW>() as u32;
        wc_panel.lpfn_wnd_proc = Some(control_panel_wnd_proc);
        wc_panel.lpsz_class_name = class_panel_name.as_ptr();
        wc_panel.h_br_background = h_bg_brush;
        wc_panel.h_icon = hicon_big;
        wc_panel.h_icon_sm = hicon_sm;
        RegisterClassExW(&wc_panel);

        // 3. Create Hidden Helper Window
        let h_helper = CreateWindowExW(
            0,
            class_helper_name.as_ptr(),
            to_wide("MKeyHelper").as_ptr(),
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            null_mut(),
        );

        if h_helper == 0 {
            eprintln!("[UI Error] Không thể tạo cửa sổ Helper!");
            return false;
        }

        // 4. Create Tray Handler
        let tray = TrayHandler::new(h_helper);

        // 5. Create Fonts (Segoe UI 9pt normal, 9pt semibold)
        let hfont_normal = create_app_font(9, FW_NORMAL);
        let hfont_bold = create_app_font(9, FW_SEMIBOLD);

        // 6. Create Control Panel Window with WS_EX_APPWINDOW to display on Windows Taskbar
        let panel_width = 445;
        let panel_height = 490;
        let h_panel = CreateWindowExW(
            WS_EX_APPWINDOW | WS_EX_CONTROLPARENT,
            class_panel_name.as_ptr(),
            to_wide("MKey - Bảng điều khiển").as_ptr(),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_CLIPCHILDREN,
            0,
            0,
            panel_width,
            panel_height,
            0,
            0,
            0,
            null_mut(),
        );

        if h_panel == 0 {
            eprintln!("[UI Error] Không thể tạo cửa sổ Control Panel!");
            return false;
        }

        // Set official App Icon on the Window & Windows Taskbar button
        if hicon_big != 0 {
            SendMessageW(h_panel, WM_SETICON, ICON_BIG, hicon_big);
        }
        if hicon_sm != 0 {
            SendMessageW(h_panel, WM_SETICON, ICON_SMALL, hicon_sm);
        }

        // Apply Windows 11 rounded corners and dark/light title bar
        apply_modern_window_styling(h_panel, is_dark);

        // Center on screen
        center_window(h_panel, panel_width, panel_height);

        // Create all components inside Control Panel
        let controls = match ControlPanelControls::create(h_panel, hfont_normal, hfont_bold) {
            Some(c) => c,
            None => {
                eprintln!("[UI Error] Không thể khởi tạo các controls của Bảng điều khiển!");
                return false;
            }
        };

        // Initial tab visibility
        controls.update_tab_visibility(0);

        let mut ui_guard = UI_MANAGER.lock().unwrap();
        *ui_guard = Some(UiState {
            h_helper,
            h_panel,
            tray,
            controls,
            hfont_normal,
            hfont_bold,
        });

        true
    }
}
