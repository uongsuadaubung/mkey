//! Tab 3: About controls (Thông tin tác giả, phiên bản, liên hệ)

use crate::language::LanguageStrings;
use crate::ui::components::window::{SW_HIDE, SW_SHOW, ShowWindow};
use crate::ui::components::*;

pub const IDC_LABEL_EMAIL: u32 = 430;
pub const IDC_LABEL_GITHUB: u32 = 431;

pub struct TabAbout {
    pub label_about_title: Label,
    pub label_about_ver: Label,
    pub label_about_author: Label,
    pub label_about_email: Label,
    pub label_about_github: Label,
}

impl TabAbout {
    pub fn create(parent: isize, hfont_normal: isize, hfont_bold: isize) -> Option<Self> {
        let strings = crate::language::current();

        let label_about_title =
            Label::create(parent, strings.about_title, 40, 150, 360, 24, hfont_bold)?;
        let label_about_ver =
            Label::create(parent, strings.about_ver, 40, 185, 360, 20, hfont_normal)?;
        let label_about_author =
            Label::create(parent, strings.about_author, 40, 215, 360, 20, hfont_normal)?;
        let label_about_email = Label::create_with_id(
            parent,
            IDC_LABEL_EMAIL,
            strings.about_email,
            40,
            245,
            360,
            20,
            hfont_normal,
        )?;
        let label_about_github = Label::create_with_id(
            parent,
            IDC_LABEL_GITHUB,
            strings.about_github,
            40,
            275,
            360,
            20,
            hfont_normal,
        )?;

        Some(Self {
            label_about_title,
            label_about_ver,
            label_about_author,
            label_about_email,
            label_about_github,
        })
    }

    pub fn update_visibility(&self, visible: bool) {
        unsafe {
            let show_t3 = if visible { SW_SHOW } else { SW_HIDE };
            ShowWindow(self.label_about_title.hwnd(), show_t3);
            ShowWindow(self.label_about_ver.hwnd(), show_t3);
            ShowWindow(self.label_about_author.hwnd(), show_t3);
            ShowWindow(self.label_about_email.hwnd(), show_t3);
            ShowWindow(self.label_about_github.hwnd(), show_t3);
        }
    }

    pub fn update_language(&self, strings: &LanguageStrings) {
        self.label_about_title.set_text(strings.about_title);
        self.label_about_ver.set_text(strings.about_ver);
        self.label_about_author.set_text(strings.about_author);
        self.label_about_email.set_text(strings.about_email);
        self.label_about_github.set_text(strings.about_github);
    }
}

