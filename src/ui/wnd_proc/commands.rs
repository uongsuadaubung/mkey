//! WM_COMMAND and WM_HSCROLL dispatcher

use super::macro_events::*;
use super::tray::open_url;
use crate::engine::{EngineConfig, config_store};
use crate::platform::win32::ENGINE_INSTANCE;
use crate::ui::components::window::*;
use crate::ui::theme::{apply_ui_theme, resolve_is_dark};
use crate::ui::views::*;
use crate::ui::{UI_MANAGER, update_tray_icon};

#[link(name = "user32")]
unsafe extern "system" {
    fn PostQuitMessage(n_exit_code: i32);
}

pub fn handle_hscroll(lparam: isize) {
    let slider_hwnd = lparam;
    if let Ok(ui_guard) = UI_MANAGER.try_lock()
        && let Some(ref ui) = *ui_guard
        && slider_hwnd == ui.controls.tab_typing.slider_volume.hwnd()
    {
        let pos = ui.controls.tab_typing.slider_volume.get_pos();
        ui.controls
            .tab_typing
            .label_volume_val
            .set_text(&format!("{pos}%"));

        if let Ok(mut guard) = ENGINE_INSTANCE.lock()
            && let Some(ref mut engine) = *guard
        {
            engine.config_mut().sound_volume = pos as u8;
            config_store::save_config_and_macros_debounced(engine.config(), &engine.macro_table);
            crate::platform::win32::sound::reconfigure_sound(
                engine.config().sound_enabled,
                &engine.config().sound_profile,
                engine.config().sound_volume,
            );
        }
    }
}

pub fn handle_command(hwnd: isize, wparam: usize) -> isize {
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
            println!("[UI] Người dùng nhấn Kết thúc -> Thoát MKey");
            config_store::flush_config_debounced();
            unsafe {
                PostQuitMessage(0);
            }
        }
        IDC_BTN_CLOSE => {
            unsafe {
                ShowWindow(hwnd, SW_HIDE);
            }
            crate::platform::win32::trim_working_set();
        }
        IDC_BTN_DEFAULTS => {
            handle_defaults_button();
        }
        IDC_BTN_SWITCH_KEY => {
            crate::ui::views::show_hotkey_dialog(hwnd);
        }
        IDC_BTN_ADD_MACRO => {
            if let Ok(ui_guard) = UI_MANAGER.try_lock()
                && let Some(ref ui) = *ui_guard
            {
                handle_add_macro(ui);
            }
        }
        IDC_BTN_EDIT_MACRO => {
            if let Ok(ui_guard) = UI_MANAGER.try_lock()
                && let Some(ref ui) = *ui_guard
            {
                handle_edit_macro(ui);
            }
        }
        IDC_BTN_DEL_MACRO => {
            if let Ok(ui_guard) = UI_MANAGER.try_lock()
                && let Some(ref ui) = *ui_guard
            {
                handle_del_macro(ui);
            }
        }
        IDC_BTN_CANCEL_MACRO => {
            if let Ok(ui_guard) = UI_MANAGER.try_lock()
                && let Some(ref ui) = *ui_guard
            {
                handle_cancel_macro(ui);
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
        IDC_BTN_CHECK_UPDATE => {
            handle_check_update_button();
        }
        IDC_LABEL_EMAIL => {
            open_url("mailto:manhkien13041997@gmail.com");
        }
        IDC_LABEL_GITHUB => {
            open_url("https://github.com/uongsuadaubung/mkey");
        }
        IDC_BTN_TEST_SOUND => {
            crate::platform::win32::sound::play_test_sound();
        }
        _ => {
            handle_toggle_or_selection(wparam, control_id);
        }
    }
    0
}

fn handle_defaults_button() {
    let def_config = EngineConfig::default();
    if let Ok(mut guard) = ENGINE_INSTANCE.lock()
        && let Some(ref mut engine) = *guard
    {
        *engine.config_mut() = def_config.clone();
        let _ = config_store::save_config_and_macros(engine.config(), &engine.macro_table);
    }
    crate::platform::win32::sound::reconfigure_sound(
        def_config.sound_enabled,
        &def_config.sound_profile,
        def_config.sound_volume,
    );
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

fn handle_check_update_button() {
    if let Ok(ui_guard) = UI_MANAGER.try_lock()
        && let Some(ref ui) = *ui_guard
    {
        let hwnd_panel = ui.h_panel;
        let strings = crate::language::current();
        ui.controls
            .tab_system
            .btn_check_update
            .set_text(strings.btn_checking_update);
        ui.controls.tab_system.btn_check_update.set_enabled(false);

        std::thread::spawn(move || {
            let status = crate::updater::check_for_updates();

            if let Ok(ui_guard) = UI_MANAGER.try_lock()
                && let Some(ref ui) = *ui_guard
            {
                let s = crate::language::current();
                ui.controls
                    .tab_system
                    .btn_check_update
                    .set_text(s.btn_check_update);
                ui.controls.tab_system.btn_check_update.set_enabled(true);
            }

            crate::updater::show_update_result_dialog(hwnd_panel, &status);
        });
    }
}

fn handle_toggle_or_selection(wparam: usize, control_id: u32) {
    let notif_code = (wparam >> 16) as u16;
    const CBN_SELCHANGE: u16 = 1;
    const BN_CLICKED: u16 = 0;

    let is_combo = control_id == IDC_COMBO_METHOD
        || control_id == IDC_COMBO_MODE
        || control_id == IDC_COMBO_THEME
        || control_id == IDC_COMBO_LANG
        || control_id == IDC_COMBO_SWITCH_TYPE;
    let is_check = control_id == IDC_CHECK_SWITCH_KEY
        || control_id == IDC_CHECK_RESTORE_WRONG
        || control_id == IDC_CHECK_AUTO_UPPER
        || control_id == IDC_CHECK_SOUND_ENABLED
        || control_id == IDC_CHECK_USE_MACRO
        || control_id == IDC_CHECK_MACRO_EN
        || control_id == IDC_CHECK_AUTOSTART
        || control_id == IDC_CHECK_SHOW_DIALOG
        || control_id == IDC_CHECK_DEBUG_LOG;

    if is_combo {
        if notif_code != CBN_SELCHANGE {
            return;
        }
    } else if is_check {
        if notif_code != BN_CLICKED {
            return;
        }
    } else {
        return;
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
        if control_id == IDC_CHECK_USE_MACRO {
            ui.controls.update_macro_checkboxes_state();
        } else if control_id == IDC_CHECK_SOUND_ENABLED {
            ui.controls.update_sound_controls_state();
        }
        ui.controls.read_config(engine.config_mut());
        config_store::save_config_and_macros_debounced(engine.config(), &engine.macro_table);
        if control_id == IDC_CHECK_SOUND_ENABLED || control_id == IDC_COMBO_SWITCH_TYPE {
            crate::platform::win32::sound::reconfigure_sound(
                engine.config().sound_enabled,
                &engine.config().sound_profile,
                engine.config().sound_volume,
            );
        }
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
                let is_dark =
                    resolve_is_dark(theme_opt.unwrap_or(crate::engine::config::UiTheme::Auto));
                apply_ui_theme(ui.h_panel, &ui.controls, is_dark);
                ui.tray.refresh_icon();
            }
        }
    } else if control_id == IDC_COMBO_MODE {
        update_tray_icon(is_viet);
    }
}
