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

/// Win32 Child window enumeration callback: automatically themes Buttons, Checkboxes,
/// ComboBoxes, TextBoxes, and Labels without requiring manual handle tracking.
unsafe extern "system" fn enum_child_theme_proc(child: isize, lparam: isize) -> i32 {
    let is_dark = lparam != 0;
    allow_window_dark_mode(child, is_dark);

    unsafe {
        let mut class_buf = [0u16; 32];
        let len = GetClassNameW(child, class_buf.as_mut_ptr(), 32);
        let class_name = String::from_utf16_lossy(&class_buf[..len.max(0) as usize]);

        let theme_str = if is_dark {
            "DarkMode_Explorer"
        } else {
            "Explorer"
        };
        let theme_w = to_wide(theme_str);

        if class_name.eq_ignore_ascii_case("Button") {
            SetWindowTheme(child, theme_w.as_ptr(), null_mut());
            InvalidateRect(child, null_mut(), 1);
        } else if class_name.eq_ignore_ascii_case("ComboBox")
            || class_name.eq_ignore_ascii_case("Edit")
        {
            let cfd_theme_str = if is_dark { "CFD" } else { "" };
            let theme_cfd_w = to_wide(cfd_theme_str);
            let res = SetWindowTheme(child, theme_cfd_w.as_ptr(), null_mut());
            if is_dark && res != 0 {
                SetWindowTheme(child, to_wide("DarkMode_CFD").as_ptr(), null_mut());
            }
            SetWindowPos(
                child,
                0,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
            );
        } else if class_name.eq_ignore_ascii_case("Static") {
            InvalidateRect(child, null_mut(), 1);
        }
    }

    1
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

        // Automatically enumerate and theme all child controls (Buttons, CheckBoxes, Combos, Edits, Labels)
        EnumChildWindows(hwnd, Some(enum_child_theme_proc), is_dark as isize);


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

