//! MKey UI Theme Engine & Dark Mode Integration

use crate::engine::config::UiTheme;
use crate::ui::colors::ThemePalette;
use crate::ui::components::window::*;
use crate::ui::views::ControlPanelControls;
use std::ptr::null_mut;

static mut PANEL_BG_BRUSH: isize = 0;
static mut TAB_CARD_BRUSH: isize = 0;
static mut INPUT_BG_BRUSH: isize = 0;
static mut BORDER_BRUSH: isize = 0;
static mut CURRENT_IS_DARK: bool = false;
static mut IS_APPLYING_THEME: bool = false;

pub fn is_current_dark() -> bool {
    unsafe { CURRENT_IS_DARK }
}

pub fn get_panel_bg_brush() -> isize {
    unsafe { PANEL_BG_BRUSH }
}

pub fn get_tab_card_brush() -> isize {
    unsafe { TAB_CARD_BRUSH }
}

pub fn get_input_bg_brush() -> isize {
    unsafe { INPUT_BG_BRUSH }
}

/// Resolves whether dark mode should be active given the theme setting
pub fn resolve_is_dark(theme: UiTheme) -> bool {
    match theme {
        UiTheme::Auto => crate::ui::components::is_windows_dark_taskbar(),
        UiTheme::Light => false,
        UiTheme::Dark => true,
    }
}

/// Recreates cached solid GDI brushes for the given theme mode
pub fn recreate_theme_brushes(is_dark: bool) {
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
    }
}

/// Applies cohesive, modern Light or Dark theme styling to all Control Panel elements
pub fn apply_ui_theme(hwnd: isize, controls: &ControlPanelControls, is_dark: bool) {
    unsafe {
        if IS_APPLYING_THEME {
            return;
        }
        IS_APPLYING_THEME = true;

        recreate_theme_brushes(is_dark);

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
            controls.combo_lang.hwnd(),
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
            controls.label_lang.hwnd(),
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

