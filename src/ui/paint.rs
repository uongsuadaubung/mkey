//! MKey Control Panel GDI Painting & Custom Card Rendering

use crate::ui::colors::ThemePalette;
use crate::ui::components::window::*;
use crate::ui::theme::get_panel_bg_brush;

/// Custom paints the Control Panel window background, rounded card containers, and borders
pub fn paint_control_panel(hwnd: isize, hdc: isize, is_dark: bool) {
    unsafe {
        let palette = ThemePalette::get(is_dark);

        let mut client_rc = RECT::default();
        GetClientRect(hwnd, &mut client_rc);

        // 1. Fill entire window background
        let bg_brush = get_panel_bg_brush();
        if bg_brush != 0 {
            FillRect(hdc, &client_rc, bg_brush);
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
    }
}
