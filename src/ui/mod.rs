//! MKey Pure Win32 UI System
//! Fully componentized, lightweight, zero-dependency native Windows UI.

pub mod colors;
pub mod components;
pub mod views;

use crate::engine::{EngineConfig, config::InputMethod};
use crate::ui::colors::ThemePalette;
use crate::ui::components::POINT;
use crate::ui::components::load_icon_from_memory;
use crate::ui::components::window::*;
use crate::ui::views::{
    ControlPanelControls, IDC_BTN_ADD_MACRO, IDC_BTN_CANCEL_MACRO, IDC_BTN_CLOSE, IDC_BTN_DEFAULTS,
    IDC_BTN_DEL_MACRO, IDC_BTN_EDIT_MACRO, IDC_BTN_EXIT, IDC_BTN_OPEN_LOG, IDC_CHECK_AUTO_UPPER,
    IDC_CHECK_AUTOSTART, IDC_CHECK_CTRL_SHIFT, IDC_CHECK_DEBUG_LOG, IDC_CHECK_RESTORE_WRONG,
    IDC_CHECK_SHOW_DIALOG, IDC_CHECK_SPELLING, IDC_CHECK_USE_MACRO, IDC_COMBO_METHOD, IDC_COMBO_MODE,
    IDC_COMBO_THEME, IDC_EDIT_MACRO_KEY, IDC_EDIT_MACRO_VALUE, IDC_LABEL_EMAIL, IDC_LABEL_GITHUB,
    IDC_LIST_MACRO, IDC_TAB_MAIN, IDM_CONTROL_PANEL, IDM_EXIT, IDM_SIMPLE_TELEX, IDM_TELEX,
    IDM_TOGGLE_VIET, IDM_VNI, TrayHandler, WM_TRAY_MESSAGE,
};
use std::ptr::null_mut;
use std::sync::Mutex;

static APP_ICO_BYTES: &[u8] = include_bytes!("../../assets/icon.ico");

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
const WM_CTLCOLORDLG: u32 = 0x0136;
const WM_CTLCOLORSTATIC: u32 = 0x0138;

static mut PANEL_BG_BRUSH: isize = 0;
static mut TAB_CARD_BRUSH: isize = 0;
static mut INPUT_BG_BRUSH: isize = 0;
static mut BORDER_BRUSH: isize = 0;
static mut CURRENT_IS_DARK: bool = false;

pub fn is_current_dark() -> bool {
    unsafe { CURRENT_IS_DARK }
}

/// Resolves whether dark mode should be active given the theme setting
pub fn resolve_is_dark(theme: crate::engine::config::UiTheme) -> bool {
    match theme {
        crate::engine::config::UiTheme::Auto => crate::ui::components::is_windows_dark_taskbar(),
        crate::engine::config::UiTheme::Light => false,
        crate::engine::config::UiTheme::Dark => true,
    }
}

static mut IS_APPLYING_THEME: bool = false;

/// Applies cohesive, modern Light or Dark theme styling to all Control Panel elements
pub fn apply_ui_theme(hwnd: isize, controls: &ControlPanelControls, is_dark: bool) {
    unsafe {
        if IS_APPLYING_THEME {
            return;
        }
        IS_APPLYING_THEME = true;

        CURRENT_IS_DARK = is_dark;
        let palette = ThemePalette::get(is_dark);

        if PANEL_BG_BRUSH != 0 {
            DeleteObject(PANEL_BG_BRUSH);
        }
        if TAB_CARD_BRUSH != 0 {
            DeleteObject(TAB_CARD_BRUSH);
        }
        if INPUT_BG_BRUSH != 0 {
            DeleteObject(INPUT_BG_BRUSH);
        }
        if BORDER_BRUSH != 0 {
            DeleteObject(BORDER_BRUSH);
        }

        PANEL_BG_BRUSH = CreateSolidBrush(palette.bg_window);
        TAB_CARD_BRUSH = CreateSolidBrush(palette.bg_card);
        INPUT_BG_BRUSH = CreateSolidBrush(palette.bg_input);
        BORDER_BRUSH = CreateSolidBrush(palette.border);

        // Windows 11 title bar & rounded corners
        apply_modern_window_styling(hwnd, is_dark);

        // Windows 10/11 uxtheme native dark mode
        set_preferred_app_mode(is_dark);
        allow_window_dark_mode(hwnd, is_dark);

        let theme_str = if is_dark {
            "DarkMode_Explorer"
        } else {
            "Explorer"
        };
        let theme_w = to_wide(theme_str);

        // Theme all PushButtons
        let buttons = [
            controls.btn_exit.hwnd(),
            controls.btn_defaults.hwnd(),
            controls.btn_close.hwnd(),
            controls.btn_open_log.hwnd(),
            controls.btn_add_macro.hwnd(),
            controls.btn_edit_macro.hwnd(),
            controls.btn_cancel_macro.hwnd(),
            controls.btn_del_macro.hwnd(),
        ];
        for &h in &buttons {
            allow_window_dark_mode(h, is_dark);
            SetWindowTheme(h, theme_w.as_ptr(), null_mut());
        }

        // Theme all ComboBoxes (using CFD which natively supports Dark Mode in Win10/11)
        let combo_theme_str = if is_dark { "CFD" } else { "" };
        let theme_combo_w = to_wide(combo_theme_str);
        let combos = [
            controls.combo_method.hwnd(),
            controls.combo_mode.hwnd(),
            controls.combo_theme.hwnd(),
        ];
        for &h in &combos {
            allow_window_dark_mode(h, is_dark);
            let res = SetWindowTheme(h, theme_combo_w.as_ptr(), null_mut());
            if is_dark && res != 0 {
                SetWindowTheme(h, to_wide("DarkMode_CFD").as_ptr(), null_mut());
            }
            SetWindowPos(
                h,
                0,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
            );
        }

        // Theme all Edit controls (TextBoxes) using CFD to match dark card borders
        let edit_theme_str = if is_dark { "CFD" } else { "" };
        let theme_edit_w = to_wide(edit_theme_str);
        let edits = [
            controls.edit_macro_key.hwnd(),
            controls.edit_macro_value.hwnd(),
        ];
        for &h in &edits {
            allow_window_dark_mode(h, is_dark);
            let res = SetWindowTheme(h, theme_edit_w.as_ptr(), null_mut());
            if is_dark && res != 0 {
                SetWindowTheme(h, to_wide("DarkMode_CFD").as_ptr(), null_mut());
            }
            SetWindowPos(
                h,
                0,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
            );
        }

        // Theme ALL CheckBoxes (critical for crisp text in DarkMode)
        let checkboxes = [
            controls.check_ctrl_shift.hwnd(),
            controls.check_spelling.hwnd(),
            controls.check_restore_wrong.hwnd(),
            controls.check_auto_upper.hwnd(),
            controls.check_use_macro.hwnd(),
            controls.check_autostart.hwnd(),
            controls.check_show_dialog.hwnd(),
            controls.check_debug_log.hwnd(),
        ];
        for &h in &checkboxes {
            allow_window_dark_mode(h, is_dark);
            SetWindowTheme(h, theme_w.as_ptr(), null_mut());
        }

        // Explicitly refresh all labels
        let labels = [
            controls.label_method.hwnd(),
            controls.label_mode.hwnd(),
            controls.label_switch.hwnd(),
            controls.label_typing_title.hwnd(),
            controls.label_sys_title.hwnd(),
            controls.label_theme_title.hwnd(),
            controls.label_theme.hwnd(),
            controls.label_about_title.hwnd(),
            controls.label_about_ver.hwnd(),
            controls.label_about_author.hwnd(),
            controls.label_about_email.hwnd(),
            controls.label_about_github.hwnd(),
        ];
        for &h in &labels {
            allow_window_dark_mode(h, is_dark);
            InvalidateRect(h, null_mut(), 1);
        }

        // Redraw TabBar with updated colors
        InvalidateRect(controls.tab_bar.hwnd(), null_mut(), 1);
        UpdateWindow(controls.tab_bar.hwnd());

        // Theme ListView
        controls.list_macro.apply_theme(is_dark);

        // Recursively invalidate and redraw entire dialog and all child controls cleanly
        RedrawWindow(
            hwnd,
            null_mut(),
            0,
            RDW_INVALIDATE | RDW_ERASE | RDW_ALLCHILDREN | RDW_FRAME,
        );

        IS_APPLYING_THEME = false;
    }
}

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
                        let res = engine.toggle_enabled();
                        let _ = crate::engine::config_store::save_config_and_macros(
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
unsafe extern "system" fn control_panel_wnd_proc(
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
                    let is_dark = CURRENT_IS_DARK;
                    let palette = ThemePalette::get(is_dark);

                    let mut client_rc = RECT::default();
                    GetClientRect(hwnd, &mut client_rc);

                    // 1. Fill entire window background
                    if PANEL_BG_BRUSH != 0 {
                        FillRect(hdc, &client_rc, PANEL_BG_BRUSH);
                    }

                    // 2. Draw Header Card (x: 20..440, y: 14..78)
                    let header_card_rc = RECT {
                        left: 20,
                        top: 14,
                        right: 440,
                        bottom: 78,
                    };
                    let h_card_br = CreateSolidBrush(palette.bg_card);
                    let h_card_pen = CreatePen(PS_SOLID, 1, palette.border);
                    let ob = SelectObject(hdc, h_card_br);
                    let op = SelectObject(hdc, h_card_pen);

                    RoundRect(
                        hdc,
                        header_card_rc.left,
                        header_card_rc.top,
                        header_card_rc.right,
                        header_card_rc.bottom,
                        12,
                        12,
                    );

                    // 3. Draw Body Card (x: 20..440, y: 134..444)
                    let body_card_rc = RECT {
                        left: 20,
                        top: 134,
                        right: 440,
                        bottom: 444,
                    };
                    RoundRect(
                        hdc,
                        body_card_rc.left,
                        body_card_rc.top,
                        body_card_rc.right,
                        body_card_rc.bottom,
                        12,
                        12,
                    );

                    SelectObject(hdc, ob);
                    SelectObject(hdc, op);
                    DeleteObject(h_card_br);
                    DeleteObject(h_card_pen);

                    EndPaint(hwnd, &ps);
                }
            }
            0
        }
        WM_SETTINGCHANGE => {
            let cfg_theme = {
                let guard = crate::platform::win32::ENGINE_INSTANCE.lock().unwrap();
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
                        ui.controls.load_config(&def_config);
                    }
                    if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock()
                        && let Some(ref mut engine) = *guard
                    {
                        *engine.config_mut() = def_config;
                        let _ = crate::engine::config_store::save_config_and_macros(
                            engine.config(),
                            &engine.macro_table,
                        );
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

                    if let Some((key, val)) = pair
                        && !key.is_empty()
                        && !val.is_empty()
                    {
                        let macros =
                            if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock() {
                                if let Some(ref mut engine) = *guard {
                                    engine.macro_table.insert(&key, &val);
                                    let _ = crate::engine::config_store::save_config_and_macros(
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
                        let macros =
                            if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock() {
                                if let Some(ref mut engine) = *guard {
                                    engine.macro_table.insert(&key, &val);
                                    let _ = crate::engine::config_store::save_config_and_macros(
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
                        let macros =
                            if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock() {
                                if let Some(ref mut engine) = *guard {
                                    if engine.macro_table.remove(&key_to_del).is_some() {
                                        let _ = crate::engine::config_store::save_config_and_macros(
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
                        let guard = crate::platform::win32::ENGINE_INSTANCE.lock().unwrap();
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
                        || control_id == IDC_COMBO_THEME;
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

                    // 1. Read updated config
                    if let Ok(mut ui_guard) = UI_MANAGER.try_lock()
                        && let Some(ref mut ui) = *ui_guard
                        && let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock()
                        && let Some(ref mut engine) = *guard
                    {
                        ui.controls.read_config(engine.config_mut());
                        let _ = crate::engine::config_store::save_config_and_macros(
                            engine.config(),
                            &engine.macro_table,
                        );
                        is_viet = engine.config().enabled;
                        theme_opt = Some(engine.config().theme);
                    }

                    // 2. Apply theme without holding UI_MANAGER lock during child redraws
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
        WM_CTLCOLORSTATIC => {
            let hdc = wparam as isize;
            let hwnd_ctrl = lparam;
            unsafe {
                let palette = ThemePalette::get(CURRENT_IS_DARK);
                let mut class_buf = [0u16; 32];
                let len = GetClassNameW(hwnd_ctrl, class_buf.as_mut_ptr(), 32);
                let class_name = String::from_utf16_lossy(&class_buf[..len.max(0) as usize]);

                if class_name.eq_ignore_ascii_case("combobox")
                    || class_name.eq_ignore_ascii_case("edit")
                {
                    SetTextColor(hdc, palette.text_input);
                    SetBkColor(hdc, palette.bg_input);
                    if INPUT_BG_BRUSH != 0 {
                        return INPUT_BG_BRUSH;
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

                    if TAB_CARD_BRUSH != 0 {
                        return TAB_CARD_BRUSH;
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
                unsafe {
                    SetCursor(LoadCursorW(0, IDC_ARROW));
                }
                1
            }
        }
        WM_CTLCOLOREDIT | WM_CTLCOLORLISTBOX => {
            let hdc = wparam as isize;
            unsafe {
                let palette = ThemePalette::get(CURRENT_IS_DARK);
                SetTextColor(hdc, palette.text_input);
                SetBkColor(hdc, palette.bg_input);
                if INPUT_BG_BRUSH != 0 {
                    return INPUT_BG_BRUSH;
                }
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
        }
        WM_CTLCOLORDLG => unsafe {
            if PANEL_BG_BRUSH != 0 {
                return PANEL_BG_BRUSH;
            }
            DefWindowProcW(hwnd, msg, wparam, lparam)
        },
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
                (
                    Some(engine.config().clone()),
                    engine.macro_table.get_sorted_entries(),
                )
            } else {
                (None, Vec::new())
            }
        } else {
            (None, Vec::new())
        }
    };

    if let Ok(ui_guard) = UI_MANAGER.try_lock()
        && let Some(ref ui) = *ui_guard
    {
        unsafe {
            if let Some(ref c) = cfg {
                ui.controls.load_config(c);
                let is_dark = resolve_is_dark(c.theme);
                apply_ui_theme(ui.h_panel, &ui.controls, is_dark);
            }
            ui.controls.populate_macros(&macros);
            ui.controls
                .update_tab_visibility(ui.controls.tab_bar.get_cur_sel());
            ShowWindow(ui.h_panel, SW_SHOW);
            SetForegroundWindow(ui.h_panel);
        }
    }
}

pub fn update_tray_icon(is_vietnamese: bool) {
    if let Ok(mut ui_guard) = UI_MANAGER.lock()
        && let Some(ref mut ui) = *ui_guard
    {
        ui.tray.set_vietnamese_mode(is_vietnamese);
        ui.controls
            .combo_mode
            .set_selected(if is_vietnamese { 0 } else { 1 });
    }
}

fn handle_menu_command(cmd: u32) {
    match cmd {
        IDM_TOGGLE_VIET => {
            let is_viet = {
                let mut guard = crate::platform::win32::ENGINE_INSTANCE.lock().unwrap();
                if let Some(ref mut engine) = *guard {
                    let res = engine.toggle_enabled();
                    let _ = crate::engine::config_store::save_config_and_macros(
                        engine.config(),
                        &engine.macro_table,
                    );
                    res
                } else {
                    false
                }
            };
            update_tray_icon(is_viet);
        }
        IDM_TELEX => {
            if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock()
                && let Some(ref mut engine) = *guard
            {
                engine.config_mut().method = InputMethod::Telex;
                let _ = crate::engine::config_store::save_config_and_macros(
                    engine.config(),
                    &engine.macro_table,
                );
            }
        }
        IDM_VNI => {
            if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock()
                && let Some(ref mut engine) = *guard
            {
                engine.config_mut().method = InputMethod::Vni;
                let _ = crate::engine::config_store::save_config_and_macros(
                    engine.config(),
                    &engine.macro_table,
                );
            }
        }
        IDM_SIMPLE_TELEX => {
            if let Ok(mut guard) = crate::platform::win32::ENGINE_INSTANCE.lock()
                && let Some(ref mut engine) = *guard
            {
                engine.config_mut().method = InputMethod::SimpleTelex1;
                let _ = crate::engine::config_store::save_config_and_macros(
                    engine.config(),
                    &engine.macro_table,
                );
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

/// Initializes the complete Win32 UI system (Tray + Control Panel)
pub fn init_ui() -> bool {
    init_common_controls();

    let class_helper_name = to_wide("MKeyHelperClass");
    let class_panel_name = to_wide("MKeyControlPanelClass");

    let cfg_theme = {
        let guard = crate::platform::win32::ENGINE_INSTANCE.lock().unwrap();
        guard
            .as_ref()
            .map(|e| e.config().theme)
            .unwrap_or(crate::engine::config::UiTheme::Auto)
    };
    let is_dark = resolve_is_dark(cfg_theme);
    set_preferred_app_mode(is_dark);

    unsafe {
        CURRENT_IS_DARK = is_dark;
        let palette = ThemePalette::get(is_dark);
        if PANEL_BG_BRUSH != 0 {
            DeleteObject(PANEL_BG_BRUSH);
        }
        if TAB_CARD_BRUSH != 0 {
            DeleteObject(TAB_CARD_BRUSH);
        }
        if INPUT_BG_BRUSH != 0 {
            DeleteObject(INPUT_BG_BRUSH);
        }
        if BORDER_BRUSH != 0 {
            DeleteObject(BORDER_BRUSH);
        }

        PANEL_BG_BRUSH = CreateSolidBrush(palette.bg_window);
        TAB_CARD_BRUSH = CreateSolidBrush(palette.bg_card);
        INPUT_BG_BRUSH = CreateSolidBrush(palette.bg_input);
        BORDER_BRUSH = CreateSolidBrush(palette.border);

        // 1. Register Helper Window Class
        let mut wc_helper: WNDCLASSEXW = std::mem::zeroed();
        wc_helper.cb_size = std::mem::size_of::<WNDCLASSEXW>() as u32;
        wc_helper.lpfn_wnd_proc = Some(tray_helper_wnd_proc);
        wc_helper.lpsz_class_name = class_helper_name.as_ptr();
        wc_helper.h_cursor = LoadCursorW(0, IDC_ARROW);
        RegisterClassExW(&wc_helper);

        // 2. Load Application Icons for Taskbar and Title Bar
        let hicon_big = load_icon_from_memory(APP_ICO_BYTES, 32, 32).unwrap_or(0);
        let hicon_sm = load_icon_from_memory(APP_ICO_BYTES, 16, 16).unwrap_or(0);

        // Register Control Panel Window Class
        let mut wc_panel: WNDCLASSEXW = std::mem::zeroed();
        wc_panel.cb_size = std::mem::size_of::<WNDCLASSEXW>() as u32;
        wc_panel.lpfn_wnd_proc = Some(control_panel_wnd_proc);
        wc_panel.lpsz_class_name = class_panel_name.as_ptr();
        wc_panel.h_br_background = 0; // Handled dynamically by WM_ERASEBKGND with PANEL_BG_BRUSH
        wc_panel.h_icon = hicon_big;
        wc_panel.h_icon_sm = hicon_sm;
        wc_panel.h_cursor = LoadCursorW(0, IDC_ARROW);
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
        let panel_width = 460;
        let panel_height = 540;
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

        // Apply theme styling across all controls
        apply_ui_theme(h_panel, &controls, is_dark);

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
