//! MKey Win32 Window Procedures & Message Handlers

use crate::engine::config::InputMethod;
use crate::engine::{EngineConfig, config_store};
use crate::platform::win32::ENGINE_INSTANCE;
use crate::ui::colors::ThemePalette;
use crate::ui::components::POINT;
use crate::ui::components::window::*;
use crate::ui::paint::paint_control_panel;
use crate::ui::theme::{
    apply_ui_theme, get_input_bg_brush, get_tab_card_brush, is_current_dark, resolve_is_dark,
};
use crate::ui::views::{
    IDC_BTN_ADD_MACRO, IDC_BTN_CANCEL_MACRO, IDC_BTN_CLOSE, IDC_BTN_DEFAULTS, IDC_BTN_DEL_MACRO,
    IDC_BTN_EDIT_MACRO, IDC_BTN_EXIT, IDC_BTN_OPEN_LOG, IDC_CHECK_AUTO_UPPER, IDC_CHECK_AUTOSTART,
    IDC_CHECK_CTRL_SHIFT, IDC_CHECK_DEBUG_LOG, IDC_CHECK_RESTORE_WRONG, IDC_CHECK_SHOW_DIALOG,
    IDC_CHECK_SPELLING, IDC_CHECK_USE_MACRO, IDC_COMBO_LANG, IDC_COMBO_METHOD, IDC_COMBO_MODE,
    IDC_COMBO_THEME, IDC_LABEL_EMAIL, IDC_LABEL_GITHUB, IDC_LIST_MACRO, IDC_TAB_MAIN,
    IDM_CONTROL_PANEL, IDM_EXIT, IDM_SIMPLE_TELEX, IDM_TELEX, IDM_TOGGLE_VIET, IDM_VNI,
    WM_TRAY_MESSAGE,
};
use crate::ui::{UI_MANAGER, show_control_panel, update_tray_icon};

const WM_DESTROY: u32 = 0x0002;
const WM_SETCURSOR: u32 = 0x0020;
const IDC_HAND: usize = 32649;
const WM_PAINT: u32 = 0x000F;
const WM_CLOSE: u32 = 0x0010;
const WM_ERASEBKGND: u32 = 0x0014;
const WM_SETTINGCHANGE: u32 = 0x001A;
const WM_COMMAND: u32 = 0x0111;
const WM_NOTIFY: u32 = 0x004E;

const WM_LBUTTONUP: usize = 0x0202;
const WM_LBUTTONDBLCLK: usize = 0x0203;
const WM_RBUTTONUP: usize = 0x0205;
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

#[link(name = "user32")]
unsafe extern "system" {
    fn GetCursorPos(lp_point: *mut POINT) -> i32;
    fn PostQuitMessage(n_exit_code: i32);
    fn LoadCursorW(h_instance: isize, lp_cursor_name: usize) -> isize;
    fn SetCursor(h_cursor: isize) -> isize;
    fn GetClassNameW(hWnd: isize, lpClassName: *mut u16, nMaxCount: i32) -> i32;
}

#[link(name = "shell32")]
unsafe extern "system" {
    fn ShellExecuteW(
        hwnd: isize,
        lp_operation: *const u16,
        lp_file: *const u16,
        lp_parameters: *const u16,
        lp_directory: *const u16,
        n_show_cmd: i32,
    ) -> isize;
}

fn open_url(url: &str) {
    unsafe {
        ShellExecuteW(
            0,
            to_wide("open").as_ptr(),
            to_wide(url).as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            1,
        );
    }
}

pub fn handle_menu_command(cmd: u32) {
    match cmd {
        IDM_TOGGLE_VIET => {
            let is_viet = {
                let mut guard = ENGINE_INSTANCE.lock().unwrap();
                if let Some(ref mut engine) = *guard {
                    let res = engine.toggle_enabled();
                    let _ =
                        config_store::save_config_and_macros(engine.config(), &engine.macro_table);
                    res
                } else {
                    false
                }
            };
            update_tray_icon(is_viet);
        }
        IDM_TELEX => {
            if let Ok(mut guard) = ENGINE_INSTANCE.lock()
                && let Some(ref mut engine) = *guard
            {
                engine.config_mut().method = InputMethod::Telex;
                let _ =
                    config_store::save_config_and_macros(engine.config(), &engine.macro_table);
            }
        }
        IDM_VNI => {
            if let Ok(mut guard) = ENGINE_INSTANCE.lock()
                && let Some(ref mut engine) = *guard
            {
                engine.config_mut().method = InputMethod::Vni;
                let _ =
                    config_store::save_config_and_macros(engine.config(), &engine.macro_table);
            }
        }
        IDM_SIMPLE_TELEX => {
            if let Ok(mut guard) = ENGINE_INSTANCE.lock()
                && let Some(ref mut engine) = *guard
            {
                engine.config_mut().method = InputMethod::SimpleTelex1;
                let _ =
                    config_store::save_config_and_macros(engine.config(), &engine.macro_table);
            }
        }
        IDM_CONTROL_PANEL => {
            show_control_panel();
        }
        IDM_EXIT => unsafe {
            PostQuitMessage(0);
        },
        _ => {}
    }
}

/// Helper window procedure for System Tray events
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn tray_helper_wnd_proc(
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
                    let mut guard = ENGINE_INSTANCE.lock().unwrap();
                    if let Some(ref mut engine) = *guard {
                        let res = engine.toggle_enabled();
                        let _ = config_store::save_config_and_macros(
                            engine.config(),
                            &engine.macro_table,
                        );
                        res
                    } else {
                        false
                    }
                };
                update_tray_icon(is_viet);
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
                    let guard = ENGINE_INSTANCE.lock().unwrap();
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
            if let Ok(mut ui_guard) = UI_MANAGER.lock()
                && let Some(ref mut ui) = *ui_guard
            {
                ui.tray.refresh_icon();
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
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn control_panel_wnd_proc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    match msg {
        WM_ERASEBKGND => 1,
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
        WM_COMMAND => {
            let control_id = (wparam & 0xFFFF) as u32;
            match control_id {
                IDC_TAB_MAIN => {
                    if let Ok(ui_guard) = UI_MANAGER.try_lock()
                        && let Some(ref ui) = *ui_guard
                    {
                        let cur_tab = ui.controls.tab_bar.get_cur_sel();
                        ui.controls.update_tab_visibility(cur_tab);
                    }
                }
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
                    if let Ok(mut ui_guard) = UI_MANAGER.lock()
                        && let Some(ref mut ui) = *ui_guard
                    {
                        crate::language::set_current_language(def_config.language);
                        ui.controls.load_config(&def_config);
                        ui.controls.update_language(def_config.language);
                        unsafe {
                            use crate::ui::components::{InvalidateRect, SetWindowTextW, UpdateWindow};
                            let strings = crate::language::get_strings(def_config.language);
                            SetWindowTextW(ui.h_panel, to_wide(strings.window_title).as_ptr());
                            InvalidateRect(ui.controls.tab_bar.hwnd(), std::ptr::null(), 1);
                            UpdateWindow(ui.controls.tab_bar.hwnd());
                        }
                        let is_dark = resolve_is_dark(def_config.theme);
                        apply_ui_theme(ui.h_panel, &ui.controls, is_dark);
                        ui.tray.refresh_icon();
                    }
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

                    if let Some((key, val)) = pair
                        && !key.is_empty()
                        && !val.is_empty()
                    {
                        let macros = if let Ok(mut guard) = ENGINE_INSTANCE.lock() {
                            if let Some(ref mut engine) = *guard {
                                engine.macro_table.insert(&key, &val);
                                let _ = config_store::save_config_and_macros(
                                    engine.config(),
                                    &engine.macro_table,
                                );
                                println!("[MKey] Đã thêm gõ tắt: '{}' -> '{}'", key, val);
                                engine.macro_table.get_sorted_entries()
                            } else {
                                Vec::new()
                            }
                        } else {
                            Vec::new()
                        };

                        if let Ok(ui_guard) = UI_MANAGER.try_lock()
                            && let Some(ref ui) = *ui_guard
                        {
                            ui.controls.populate_macros(&macros);
                            ui.controls.edit_macro_key.clear();
                            ui.controls.edit_macro_value.clear();
                            ui.controls.list_macro.clear_selection();
                            ui.controls.set_macro_edit_mode(false);
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

                    if let Some((key, val)) = pair
                        && !key.is_empty()
                        && !val.is_empty()
                    {
                        let macros = if let Ok(mut guard) = ENGINE_INSTANCE.lock() {
                            if let Some(ref mut engine) = *guard {
                                engine.macro_table.insert(&key, &val);
                                let _ = config_store::save_config_and_macros(
                                    engine.config(),
                                    &engine.macro_table,
                                );
                                println!("[MKey] Đã cập nhật gõ tắt: '{}' -> '{}'", key, val);
                                engine.macro_table.get_sorted_entries()
                            } else {
                                Vec::new()
                            }
                        } else {
                            Vec::new()
                        };

                        if let Ok(ui_guard) = UI_MANAGER.try_lock()
                            && let Some(ref ui) = *ui_guard
                        {
                            ui.controls.populate_macros(&macros);
                            ui.controls.edit_macro_key.clear();
                            ui.controls.edit_macro_value.clear();
                            ui.controls.list_macro.clear_selection();
                            ui.controls.set_macro_edit_mode(false);
                        }
                    }
                }
                IDC_BTN_DEL_MACRO => {
                    let key_to_del = if let Ok(ui_guard) = UI_MANAGER.try_lock() {
                        if let Some(ref ui) = *ui_guard {
                            let key_input =
                                ui.controls.edit_macro_key.get_text().trim().to_string();
                            if !key_input.is_empty() {
                                key_input
                            } else if let Some(sel_idx) =
                                ui.controls.list_macro.get_selected_index()
                            {
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
                        let macros = if let Ok(mut guard) = ENGINE_INSTANCE.lock() {
                            if let Some(ref mut engine) = *guard {
                                if engine.macro_table.remove(&key_to_del).is_some() {
                                    let _ = config_store::save_config_and_macros(
                                        engine.config(),
                                        &engine.macro_table,
                                    );
                                    println!("[MKey] Đã xóa từ gõ tắt: '{}'", key_to_del);
                                }
                                engine.macro_table.get_sorted_entries()
                            } else {
                                Vec::new()
                            }
                        } else {
                            Vec::new()
                        };

                        if let Ok(ui_guard) = UI_MANAGER.try_lock()
                            && let Some(ref ui) = *ui_guard
                        {
                            ui.controls.populate_macros(&macros);
                            ui.controls.edit_macro_key.clear();
                            ui.controls.edit_macro_value.clear();
                            ui.controls.list_macro.clear_selection();
                            ui.controls.set_macro_edit_mode(false);
                        }
                    }
                }
                IDC_BTN_CANCEL_MACRO => {
                    if let Ok(ui_guard) = UI_MANAGER.try_lock()
                        && let Some(ref ui) = *ui_guard
                    {
                        ui.controls.edit_macro_key.clear();
                        ui.controls.edit_macro_value.clear();
                        ui.controls.list_macro.clear_selection();
                        ui.controls.set_macro_edit_mode(false);
                    }
                }
                IDC_BTN_OPEN_LOG => {
                    let log_file = {
                        let guard = ENGINE_INSTANCE.lock().unwrap();
                        guard
                            .as_ref()
                            .and_then(|e| e.config().debug_file_path.clone())
                            .unwrap_or_else(|| "mkey_debug.log".to_string())
                    };
                    if !std::path::Path::new(&log_file).exists() {
                        let _ = std::fs::File::create(&log_file);
                    }
                    let _ = std::process::Command::new("notepad.exe")
                        .arg(&log_file)
                        .spawn();
                }
                IDC_LABEL_EMAIL => {
                    open_url("mailto:manhkien13041997@gmail.com");
                }
                IDC_LABEL_GITHUB => {
                    open_url("https://github.com/uongsuadaubung/mkey");
                }
                _ => {
                    let notif_code = (wparam >> 16) as u16;
                    const CBN_SELCHANGE: u16 = 1;
                    const BN_CLICKED: u16 = 0;

                    let is_combo = control_id == IDC_COMBO_METHOD
                        || control_id == IDC_COMBO_MODE
                        || control_id == IDC_COMBO_THEME
                        || control_id == IDC_COMBO_LANG;
                    let is_check = control_id == IDC_CHECK_CTRL_SHIFT
                        || control_id == IDC_CHECK_SPELLING
                        || control_id == IDC_CHECK_RESTORE_WRONG
                        || control_id == IDC_CHECK_AUTO_UPPER
                        || control_id == IDC_CHECK_USE_MACRO
                        || control_id == IDC_CHECK_AUTOSTART
                        || control_id == IDC_CHECK_SHOW_DIALOG
                        || control_id == IDC_CHECK_DEBUG_LOG;

                    if is_combo {
                        if notif_code != CBN_SELCHANGE {
                            return 0;
                        }
                    } else if is_check {
                        if notif_code != BN_CLICKED {
                            return 0;
                        }
                    } else {
                        return 0;
                    }

                    // Checkbox or combobox toggled -> Sync to EngineConfig
                    let mut is_viet = true;
                    let mut theme_opt = None;
                    let mut lang_opt = None;

                    // 1. Read updated config
                    if let Ok(mut ui_guard) = UI_MANAGER.try_lock()
                        && let Some(ref mut ui) = *ui_guard
                        && let Ok(mut guard) = ENGINE_INSTANCE.lock()
                        && let Some(ref mut engine) = *guard
                    {
                        ui.controls.read_config(engine.config_mut());
                        let _ = config_store::save_config_and_macros(
                            engine.config(),
                            &engine.macro_table,
                        );
                        is_viet = engine.config().enabled;
                        theme_opt = Some(engine.config().theme);
                        lang_opt = Some(engine.config().language);
                    }

                    // 2. Apply theme or language without holding UI_MANAGER lock during child redraws
                    if control_id == IDC_COMBO_THEME {
                        if let Some(theme) = theme_opt {
                            let is_dark = resolve_is_dark(theme);
                            if let Ok(mut ui_guard) = UI_MANAGER.try_lock()
                                && let Some(ref mut ui) = *ui_guard
                            {
                                apply_ui_theme(ui.h_panel, &ui.controls, is_dark);
                                ui.tray.refresh_icon();
                            }
                        }
                    } else if control_id == IDC_COMBO_LANG {
                        if let Some(lang) = lang_opt {
                            crate::language::set_current_language(lang);
                            if let Ok(mut ui_guard) = UI_MANAGER.try_lock()
                                && let Some(ref mut ui) = *ui_guard
                            {
                                ui.controls.update_language(lang);
                                unsafe {
                                    use crate::ui::components::{InvalidateRect, SetWindowTextW, UpdateWindow};
                                    let strings = crate::language::get_strings(lang);
                                    SetWindowTextW(ui.h_panel, to_wide(strings.window_title).as_ptr());
                                    InvalidateRect(ui.controls.tab_bar.hwnd(), std::ptr::null(), 1);
                                    UpdateWindow(ui.controls.tab_bar.hwnd());
                                }
                                let is_dark = resolve_is_dark(
                                    theme_opt.unwrap_or(crate::engine::config::UiTheme::Auto),
                                );
                                apply_ui_theme(ui.h_panel, &ui.controls, is_dark);
                                ui.tray.refresh_icon();
                            }
                        }
                    } else if control_id == IDC_COMBO_MODE {
                        update_tray_icon(is_viet);
                    }
                }
            }
            0
        }
        WM_NOTIFY => {
            let nmhdr = unsafe { &*(lparam as *const NMHDR) };
            if nmhdr.id_from == IDC_LIST_MACRO as usize
                && (nmhdr.code == 0xFFFFFFFE || nmhdr.code == 0xFFFFFFFD)
            {
                // When clicking or selecting an item in the ListView, populate the edit boxes and switch to edit mode
                if let Ok(ui_guard) = UI_MANAGER.try_lock()
                    && let Some(ref ui) = *ui_guard
                    && let Some(sel_idx) = ui.controls.list_macro.get_selected_index()
                {
                    let k = ui.controls.list_macro.get_item_text(sel_idx, 0);
                    let v = ui.controls.list_macro.get_item_text(sel_idx, 1);
                    if !k.is_empty() {
                        ui.controls.edit_macro_key.set_text(&k);
                        ui.controls.edit_macro_value.set_text(&v);
                        ui.controls.set_macro_edit_mode(true);
                    }
                }
            }
            0
        }
        WM_CTLCOLOREDIT | WM_CTLCOLORLISTBOX => {
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
        WM_CTLCOLORSTATIC => {
            let hdc = wparam as isize;
            let hwnd_ctrl = lparam;
            unsafe {
                let palette = ThemePalette::get(is_current_dark());
                let mut class_buf = [0u16; 32];
                let len = GetClassNameW(hwnd_ctrl, class_buf.as_mut_ptr(), 32);
                let class_name = String::from_utf16_lossy(&class_buf[..len.max(0) as usize]);

                if class_name.eq_ignore_ascii_case("combobox")
                    || class_name.eq_ignore_ascii_case("edit")
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

                    let is_link = if let Ok(ui_guard) = UI_MANAGER.try_lock() {
                        if let Some(ref ui) = *ui_guard {
                            hwnd_ctrl == ui.controls.label_about_email.hwnd()
                                || hwnd_ctrl == ui.controls.label_about_github.hwnd()
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if is_link {
                        SetTextColor(hdc, palette.text_link);
                    } else {
                        SetTextColor(hdc, palette.text_primary);
                    }

                    let card_br = get_tab_card_brush();
                    if card_br != 0 {
                        return card_br;
                    }
                }
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
        }
        WM_SETCURSOR => {
            let child_hwnd = wparam as isize;
            let (is_link, is_edit) = if let Ok(ui_guard) = UI_MANAGER.try_lock() {
                if let Some(ref ui) = *ui_guard {
                    let link = child_hwnd == ui.controls.label_about_email.hwnd()
                        || child_hwnd == ui.controls.label_about_github.hwnd();
                    let edit = child_hwnd == ui.controls.edit_macro_key.hwnd()
                        || child_hwnd == ui.controls.edit_macro_value.hwnd();
                    (link, edit)
                } else {
                    (false, false)
                }
            } else {
                (false, false)
            };

            if is_link {
                unsafe {
                    SetCursor(LoadCursorW(0, IDC_HAND));
                }
                1
            } else if is_edit {
                unsafe {
                    SetCursor(LoadCursorW(0, IDC_IBEAM));
                }
                1
            } else {
                unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
            }
        }
        WM_CLOSE => {
            unsafe {
                ShowWindow(hwnd, SW_HIDE);
            }
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}
