//! Footer buttons ([ Kết thúc ] - [ Mặc định ] - [ Đóng ])

use crate::language::LanguageStrings;
use crate::ui::components::*;

pub const IDC_BTN_EXIT: u32 = 500;
pub const IDC_BTN_DEFAULTS: u32 = 501;
pub const IDC_BTN_CLOSE: u32 = 502;

pub struct FooterControls {
    pub btn_exit: PushButton,
    pub btn_defaults: PushButton,
    pub btn_close: PushButton,
}

impl FooterControls {
    pub fn create(parent: isize, hfont_normal: isize) -> Option<Self> {
        let strings = crate::language::current();

        let btn_exit = PushButton::create(
            parent,
            IDC_BTN_EXIT,
            strings.btn_exit,
            20,
            456,
            110,
            32,
            hfont_normal,
        )?;
        let btn_defaults = PushButton::create(
            parent,
            IDC_BTN_DEFAULTS,
            strings.btn_defaults,
            175,
            456,
            110,
            32,
            hfont_normal,
        )?;
        let btn_close = PushButton::create(
            parent,
            IDC_BTN_CLOSE,
            strings.btn_close,
            330,
            456,
            110,
            32,
            hfont_normal,
        )?;

        Some(Self {
            btn_exit,
            btn_defaults,
            btn_close,
        })
    }

    pub fn update_language(&self, strings: &LanguageStrings) {
        self.btn_exit.set_text(strings.btn_exit);
        self.btn_defaults.set_text(strings.btn_defaults);
        self.btn_close.set_text(strings.btn_close);
    }
}

