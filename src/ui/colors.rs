//! MKey UI Theme Color Definitions
//! Centralized color constants for Light and Dark modes.
//! All colors in the application MUST be defined here in ThemePalette.
//! No component should construct or hardcode arbitrary RGB colors.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemePalette {
    /// Window background (dialog background, header, footer)
    pub bg_window: u32,
    /// Card background (tab interior card body)
    pub bg_card: u32,
    /// Header background for listviews/tables
    pub bg_card_header: u32,
    /// Input background (edit boxes, list views)
    pub bg_input: u32,
    /// Primary text color (labels, checkboxes, button texts, table header text)
    pub text_primary: u32,
    /// Secondary / muted text color (descriptions, hints, placeholder cue banner)
    pub text_secondary: u32,
    /// Text inside edit boxes and list views
    pub text_input: u32,
    /// Border color for cards, listviews, frames
    pub border: u32,
    /// Subtle divider line between columns or sections
    pub border_separator: u32,
    /// Segmented tab track background
    pub tab_track_bg: u32,
    /// Segmented tab track border
    pub tab_track_border: u32,
    /// Active tab pill background
    pub tab_active_bg: u32,
    /// Active tab pill border
    pub tab_active_border: u32,
    /// Inactive tab hover background
    pub tab_hover_bg: u32,
    /// Hyperlink text color (email, github links)
    pub text_link: u32,
}

/// Private helper to pack RGB bytes into a Win32 COLORREF (0x00BBGGRR).
/// Strictly private to this module so all colors are centralized in ThemePalette.
const fn rgb(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) | ((g as u32) << 8) | ((b as u32) << 16)
}

/// Dark Theme Palette (Windows 11 Fluent Dark inspired)
pub const DARK_PALETTE: ThemePalette = ThemePalette {
    bg_window: rgb(32, 32, 32),         // #202020 - Solid dark window
    bg_card: rgb(43, 43, 44),           // #2B2B2C - Card interior
    bg_card_header: rgb(36, 36, 38),    // #242426 - Table/listview header
    bg_input: rgb(48, 48, 50),          // #303032 - Input background
    text_primary: rgb(245, 245, 245),   // #F5F5F5 - Bright, crisp readable white
    text_secondary: rgb(170, 170, 170), // #AAAAAA - Soft secondary silver gray
    text_input: rgb(255, 255, 255),     // #FFFFFF - Pure white input text
    border: rgb(68, 68, 70),            // #444446 - Refined card border
    border_separator: rgb(58, 58, 62),  // #3A3A3E - Subtle column divider
    tab_track_bg: rgb(36, 36, 38),      // #242426 - Tab track background
    tab_track_border: rgb(52, 52, 56),  // #343438 - Tab track border
    tab_active_bg: rgb(56, 56, 60),     // #38383C - Active pill background
    tab_active_border: rgb(76, 76, 82), // #4C4C52 - Active pill border
    tab_hover_bg: rgb(44, 44, 48),      // #2C2C30 - Hover pill background
    text_link: rgb(88, 166, 255),       // #58A6FF - Crisp Fluent blue link
};

/// Light Theme Palette (Windows 11 Fluent Light inspired)
pub const LIGHT_PALETTE: ThemePalette = ThemePalette {
    bg_window: rgb(243, 243, 243),      // #F3F3F3 - Soft light gray window
    bg_card: rgb(255, 255, 255),        // #FFFFFF - Pure white card
    bg_card_header: rgb(240, 240, 243), // #F0F0F3 - Table/listview header
    bg_input: rgb(255, 255, 255),       // #FFFFFF - White input background
    text_primary: rgb(25, 25, 25),      // #191919 - High contrast near-black
    text_secondary: rgb(100, 100, 100), // #646464 - Neutral gray secondary
    text_input: rgb(25, 25, 25),        // #191919 - Dark input text
    border: rgb(220, 220, 224),         // #DCDCE0 - Subtle border
    border_separator: rgb(215, 215, 220), // #D7D7DC - Subtle column divider
    tab_track_bg: rgb(234, 234, 238),   // #EAEAEF - Tab track background
    tab_track_border: rgb(220, 220, 224), // #DCDCE0 - Tab track border
    tab_active_bg: rgb(255, 255, 255),  // #FFFFFF - Active pill background
    tab_active_border: rgb(212, 212, 216), // #D4D4D8 - Active pill border
    tab_hover_bg: rgb(242, 242, 246),   // #F2F2F6 - Hover pill background
    text_link: rgb(0, 102, 204),        // #0066CC - Readable Fluent blue link
};

impl ThemePalette {
    #[inline]
    pub const fn get(is_dark: bool) -> &'static ThemePalette {
        if is_dark {
            &DARK_PALETTE
        } else {
            &LIGHT_PALETTE
        }
    }
}
