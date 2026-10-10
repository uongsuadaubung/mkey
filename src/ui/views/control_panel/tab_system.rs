//! Tab 2: System controls (Khởi động cùng Windows, Giao diện, Ngôn ngữ, Nhật ký lỗi, Cập nhật)

use crate::language::LanguageStrings;
use crate::ui::components::window::{SW_HIDE, SW_SHOW, ShowWindow};
use crate::ui::components::*;

pub const IDC_CHECK_AUTOSTART: u32 = 420;
pub const IDC_CHECK_SHOW_DIALOG: u32 = 424;
pub const IDC_CHECK_DEBUG_LOG: u32 = 421;
pub const IDC_BTN_OPEN_LOG: u32 = 422;
pub const IDC_COMBO_THEME: u32 = 423;
pub const IDC_COMBO_LANG: u32 = 425;
pub const IDC_BTN_CHECK_UPDATE: u32 = 426;

pub struct TabSystem {
    pub label_sys_title: Label,
    pub check_autostart: CheckBox,
    pub check_show_dialog: CheckBox,
    pub check_debug_log: CheckBox,
    pub btn_open_log: PushButton,
    pub label_theme_title: Label,
    pub label_theme: Label,
    pub combo_theme: ComboBox,
    pub label_lang: Label,
    pub combo_lang: ComboBox,
    pub btn_check_update: PushButton,
}

impl TabSystem {
    pub fn create(parent: isize, hfont_normal: isize, hfont_bold: isize) -> Option<Self> {
        let strings = crate::language::current();

        let label_sys_title =
            Label::create(parent, strings.sys_title, 40, 150, 360, 22, hfont_bold)?;
        let check_autostart = CheckBox::create(
            parent,
            IDC_CHECK_AUTOSTART,
            strings.check_autostart,
            40,
            178,
            360,
            24,
            hfont_normal,
        )?;
        let check_show_dialog = CheckBox::create(
            parent,
            IDC_CHECK_SHOW_DIALOG,
            strings.check_show_dialog,
            40,
            206,
            360,
            24,
            hfont_normal,
        )?;
        let check_debug_log = CheckBox::create(
            parent,
            IDC_CHECK_DEBUG_LOG,
            strings.check_debug_log,
            40,
            236,
            250,
            24,
            hfont_normal,
        )?;
        let btn_open_log = PushButton::create(
            parent,
            IDC_BTN_OPEN_LOG,
            strings.btn_open_log,
            300,
            233,
            100,
            26,
            hfont_normal,
        )?;
        let label_theme_title = Label::create(
            parent,
            strings.label_theme_title,
            40,
            275,
            360,
            22,
            hfont_bold,
        )?;
        let label_theme =
            Label::create(parent, strings.label_theme, 40, 307, 120, 20, hfont_normal)?;
        let combo_theme =
            ComboBox::create(parent, IDC_COMBO_THEME, 165, 303, 235, 150, hfont_normal)?;
        combo_theme.add_item(strings.theme_auto);
        combo_theme.add_item(strings.theme_light);
        combo_theme.add_item(strings.theme_dark);
        combo_theme.set_selected(0);

        let label_lang = Label::create(
            parent,
            strings.label_language,
            40,
            345,
            120,
            20,
            hfont_normal,
        )?;
        let combo_lang =
            ComboBox::create(parent, IDC_COMBO_LANG, 165, 341, 235, 150, hfont_normal)?;
        combo_lang.add_item("Tiếng Việt");
        combo_lang.add_item("English");
        combo_lang.set_selected(0);

        let btn_check_update = PushButton::create(
            parent,
            IDC_BTN_CHECK_UPDATE,
            strings.btn_check_update,
            40,
            380,
            180,
            28,
            hfont_normal,
        )?;

        Some(Self {
            label_sys_title,
            check_autostart,
            check_show_dialog,
            check_debug_log,
            btn_open_log,
            label_theme_title,
            label_theme,
            combo_theme,
            label_lang,
            combo_lang,
            btn_check_update,
        })
    }

    pub fn update_visibility(&self, visible: bool) {
        unsafe {
            let show_t2 = if visible { SW_SHOW } else { SW_HIDE };
            ShowWindow(self.label_sys_title.hwnd(), show_t2);
            ShowWindow(self.check_autostart.hwnd(), show_t2);
            ShowWindow(self.check_show_dialog.hwnd(), show_t2);
            ShowWindow(self.check_debug_log.hwnd(), show_t2);
            ShowWindow(self.btn_open_log.hwnd(), show_t2);
            ShowWindow(self.label_theme_title.hwnd(), show_t2);
            ShowWindow(self.label_theme.hwnd(), show_t2);
            ShowWindow(self.combo_theme.hwnd(), show_t2);
            ShowWindow(self.label_lang.hwnd(), show_t2);
            ShowWindow(self.combo_lang.hwnd(), show_t2);
            ShowWindow(self.btn_check_update.hwnd(), show_t2);
        }
    }

    pub fn load_config(&self, config: &crate::engine::EngineConfig) {
        // Luôn đọc trực tiếp từ Registry làm nguồn chuẩn xác nhất
        self.check_autostart
            .set_checked(crate::platform::is_windows_autostart_enabled());
        self.check_show_dialog
            .set_checked(config.show_dialog_on_startup);
        self.check_debug_log.set_checked(config.debug);

        self.combo_theme.set_selected(config.theme.into());
        self.combo_lang.set_selected(config.language.into());
    }

    pub fn read_config(&self, config: &mut crate::engine::EngineConfig) {
        let new_autostart = self.check_autostart.is_checked();
        if crate::platform::is_windows_autostart_enabled() != new_autostart {
            crate::platform::set_windows_autostart(new_autostart);
            println!(
                "[MKey] >> Khởi động cùng Windows: {}",
                if new_autostart { "BẬT" } else { "TẮT" }
            );
        }

        config.show_dialog_on_startup = self.check_show_dialog.is_checked();

        let new_debug = self.check_debug_log.is_checked();
        if config.debug != new_debug {
            config.debug = new_debug;
            println!(
                "[MKey] >> Ghi nhật ký lỗi (Debug Log): {}",
                if new_debug { "BẬT" } else { "TẮT" }
            );
        }

        if let Some(theme_sel) = self.combo_theme.get_selected() {
            let new_theme = crate::engine::config::UiTheme::from(theme_sel);
            if config.theme != new_theme {
                config.theme = new_theme;
                println!("[MKey] >> Đổi giao diện sang: {:?}", new_theme);
            }
        }

        if let Some(lang_sel) = self.combo_lang.get_selected() {
            let new_lang = crate::language::Language::from(lang_sel);
            if config.language != new_lang {
                config.language = new_lang;
                println!("[MKey] >> Đổi ngôn ngữ giao diện sang: {:?}", new_lang);
            }
        }
    }

    pub fn update_language(&self, lang: crate::language::Language, strings: &LanguageStrings) {
        self.label_sys_title.set_text(strings.sys_title);
        self.check_autostart.set_text(strings.check_autostart);
        self.check_show_dialog.set_text(strings.check_show_dialog);
        self.check_debug_log.set_text(strings.check_debug_log);
        self.btn_open_log.set_text(strings.btn_open_log);
        self.label_theme_title.set_text(strings.label_theme_title);
        self.label_theme.set_text(strings.label_theme);

        let cur_theme = self.combo_theme.get_selected().unwrap_or(0);
        self.combo_theme.reset_items(
            &[strings.theme_auto, strings.theme_light, strings.theme_dark],
            Some(cur_theme),
        );

        self.label_lang.set_text(strings.label_language);
        self.combo_lang.set_selected(lang.into());
        self.btn_check_update.set_text(strings.btn_check_update);
    }
}
