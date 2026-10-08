//! MKey Pure Win32 UI System
//! Fully componentized, lightweight, zero-dependency native Windows UI.

pub mod colors;
pub mod components;
pub mod paint;
pub mod theme;
pub mod views;
pub mod wnd_proc;

pub use theme::{apply_ui_theme, is_current_dark, resolve_is_dark};

use crate::ui::components::load_icon_from_memory;
use crate::ui::components::window::*;
use crate::ui::theme::recreate_theme_brushes;
use crate::ui::views::{ControlPanelControls, TrayHandler};
use crate::ui::wnd_proc::{control_panel_wnd_proc, tray_helper_wnd_proc};
use std::ptr::null_mut;
use std::sync::Mutex;

static APP_ICO_BYTES: &[u8] = include_bytes!("../../assets/icon.ico");

#[link(name = "user32")]
unsafe extern "system" {
    fn LoadCursorW(h_instance: isize, lp_cursor_name: usize) -> isize;
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
                crate::language::set_current_language(c.language);
                ui.controls.update_language(c.language);
                let strings = crate::language::get_strings(c.language);
                SetWindowTextW(ui.h_panel, to_wide(strings.window_title).as_ptr());
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

/// Initializes the complete Win32 UI system (Tray + Control Panel)
pub fn init_ui() -> bool {
    init_common_controls();

    let class_helper_name = to_wide("MKeyHelperClass");
    let class_panel_name = to_wide("MKeyControlPanelClass");

    let (cfg_theme, cfg_lang, init_config) = {
        let guard = crate::platform::win32::ENGINE_INSTANCE.lock().unwrap();
        if let Some(ref e) = *guard {
            (e.config().theme, e.config().language, Some(e.config().clone()))
        } else {
            (
                crate::engine::config::UiTheme::Auto,
                crate::language::Language::Vietnamese,
                None,
            )
        }
    };
    crate::language::set_current_language(cfg_lang);
    let is_dark = resolve_is_dark(cfg_theme);
    set_preferred_app_mode(is_dark);

    unsafe {
        recreate_theme_brushes(is_dark);

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
            to_wide(crate::language::current().helper_window_title).as_ptr(),
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
            to_wide(crate::language::current().window_title).as_ptr(),
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

        // Load initial config and synchronize language
        if let Some(ref c) = init_config {
            controls.load_config(c);
            controls.update_language(cfg_lang);
        }

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
