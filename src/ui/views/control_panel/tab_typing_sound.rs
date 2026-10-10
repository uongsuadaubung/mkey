//! Tab 0: Typing & Sound controls (Kiểu gõ nâng cao, Bàn phím cơ & Âm thanh)

use crate::language::LanguageStrings;
use crate::ui::components::window::{SW_HIDE, SW_SHOW, ShowWindow};
use crate::ui::components::*;

pub const IDC_CHECK_RESTORE_WRONG: u32 = 402;
pub const IDC_CHECK_AUTO_UPPER: u32 = 405;
pub const IDC_CHECK_SOUND_ENABLED: u32 = 440;
pub const IDC_COMBO_SWITCH_TYPE: u32 = 441;
pub const IDC_SLIDER_SOUND_VOLUME: u32 = 442;
pub const IDC_BTN_TEST_SOUND: u32 = 443;
pub const IDC_LABEL_VOLUME_VAL: u32 = 444;

pub struct TabTypingSound {
    pub label_typing_title: Label,
    pub check_restore_wrong: CheckBox,
    pub check_auto_upper: CheckBox,
    pub label_sound_title: Label,
    pub check_sound_enabled: CheckBox,
    pub label_switch_type: Label,
    pub combo_switch_type: ComboBox,
    pub btn_test_sound: PushButton,
    pub label_volume: Label,
    pub slider_volume: Slider,
    pub label_volume_val: Label,
    pub sound_available: bool,
}

impl TabTypingSound {
    pub fn create(parent: isize, hfont_normal: isize, hfont_bold: isize) -> Option<Self> {
        let strings = crate::language::current();

        let label_typing_title =
            Label::create(parent, strings.typing_title, 40, 146, 360, 20, hfont_bold)?;
        let check_restore_wrong = CheckBox::create(
            parent,
            IDC_CHECK_RESTORE_WRONG,
            strings.check_restore_wrong,
            40,
            172,
            360,
            22,
            hfont_normal,
        )?;
        let check_auto_upper = CheckBox::create(
            parent,
            IDC_CHECK_AUTO_UPPER,
            strings.check_auto_upper,
            40,
            198,
            360,
            22,
            hfont_normal,
        )?;

        let label_sound_title =
            Label::create(parent, strings.sound_title, 40, 230, 360, 20, hfont_bold)?;
        let check_sound_enabled = CheckBox::create(
            parent,
            IDC_CHECK_SOUND_ENABLED,
            strings.check_sound_enabled,
            40,
            254,
            360,
            24,
            hfont_normal,
        )?;
        let label_switch_type = Label::create(
            parent,
            strings.label_switch_type,
            40,
            288,
            85,
            20,
            hfont_normal,
        )?;
        let combo_switch_type = ComboBox::create(
            parent,
            IDC_COMBO_SWITCH_TYPE,
            130,
            284,
            185,
            150,
            hfont_normal,
        )?;
        let switch_profiles = crate::engine::config_store::list_switch_profiles();
        let sound_available = !switch_profiles.is_empty();
        for p in &switch_profiles {
            combo_switch_type.add_item(p);
        }
        if switch_profiles.is_empty() {
            combo_switch_type.add_item("Default");
        }
        combo_switch_type.set_selected(0);

        let btn_test_sound = PushButton::create(
            parent,
            IDC_BTN_TEST_SOUND,
            strings.btn_test_sound,
            325,
            283,
            75,
            25,
            hfont_normal,
        )?;

        let label_volume =
            Label::create(parent, strings.label_volume, 40, 324, 85, 20, hfont_normal)?;
        let slider_volume =
            Slider::create(parent, IDC_SLIDER_SOUND_VOLUME, 130, 320, 210, 26, 0, 100)?;
        slider_volume.set_pos(50);

        let label_volume_val = Label::create_with_id(
            parent,
            IDC_LABEL_VOLUME_VAL,
            "50%",
            350,
            324,
            50,
            20,
            hfont_normal,
        )?;

        let tab = Self {
            label_typing_title,
            check_restore_wrong,
            check_auto_upper,
            label_sound_title,
            check_sound_enabled,
            label_switch_type,
            combo_switch_type,
            btn_test_sound,
            label_volume,
            slider_volume,
            label_volume_val,
            sound_available,
        };
        tab.update_sound_controls_state();
        Some(tab)
    }

    /// Dynamically enables or disables child sound controls based on whether `check_sound_enabled` is checked
    pub fn update_sound_controls_state(&self) {
        let is_enabled = self.sound_available && self.check_sound_enabled.is_checked();
        self.combo_switch_type.set_enabled(is_enabled);
        self.btn_test_sound.set_enabled(is_enabled);
        self.slider_volume.set_enabled(is_enabled);
    }

    pub fn update_visibility(&self, visible: bool) {
        unsafe {
            let show_t0 = if visible { SW_SHOW } else { SW_HIDE };
            ShowWindow(self.label_typing_title.hwnd(), show_t0);
            ShowWindow(self.check_restore_wrong.hwnd(), show_t0);
            ShowWindow(self.check_auto_upper.hwnd(), show_t0);

            let show_sound = if visible && self.sound_available {
                SW_SHOW
            } else {
                SW_HIDE
            };
            ShowWindow(self.label_sound_title.hwnd(), show_sound);
            ShowWindow(self.check_sound_enabled.hwnd(), show_sound);
            ShowWindow(self.label_switch_type.hwnd(), show_sound);
            ShowWindow(self.combo_switch_type.hwnd(), show_sound);
            ShowWindow(self.btn_test_sound.hwnd(), show_sound);
            ShowWindow(self.label_volume.hwnd(), show_sound);
            ShowWindow(self.slider_volume.hwnd(), show_sound);
            ShowWindow(self.label_volume_val.hwnd(), show_sound);

            if visible && self.sound_available {
                SetWindowPos(
                    self.slider_volume.hwnd(),
                    HWND_TOP,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW,
                );
                SetWindowPos(
                    self.combo_switch_type.hwnd(),
                    HWND_TOP,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW,
                );
            }
        }
    }

    pub fn load_config(&self, config: &crate::engine::EngineConfig) {
        self.check_restore_wrong
            .set_checked(config.restore_on_wrong_spelling);
        self.check_auto_upper
            .set_checked(config.auto_uppercase_first_char);
        self.check_sound_enabled
            .set_checked(config.sound_enabled && self.sound_available);

        let profiles = crate::engine::config_store::list_switch_profiles();
        if let Some(pos) = profiles
            .iter()
            .position(|p| p.eq_ignore_ascii_case(&config.sound_profile))
        {
            self.combo_switch_type.set_selected(pos);
        } else if !profiles.is_empty() {
            self.combo_switch_type.set_selected(0);
        }
        self.slider_volume.set_pos(config.sound_volume as u32);
        self.label_volume_val
            .set_text(&format!("{}%", config.sound_volume));
        self.update_sound_controls_state();
    }

    pub fn read_config(&self, config: &mut crate::engine::EngineConfig) {
        config.restore_on_wrong_spelling = self.check_restore_wrong.is_checked();
        config.auto_uppercase_first_char = self.check_auto_upper.is_checked();
        config.sound_enabled = self.check_sound_enabled.is_checked() && self.sound_available;
        if let Some(profile_name) = self.combo_switch_type.get_selected_text() {
            config.sound_profile = profile_name;
        }
        config.sound_volume = self.slider_volume.get_pos() as u8;
        crate::platform::win32::sound::reconfigure_sound(
            config.sound_enabled,
            &config.sound_profile,
            config.sound_volume,
        );
    }

    pub fn update_language(&self, strings: &LanguageStrings) {
        self.label_typing_title.set_text(strings.typing_title);
        self.check_restore_wrong
            .set_text(strings.check_restore_wrong);
        self.check_auto_upper.set_text(strings.check_auto_upper);
        self.label_sound_title.set_text(strings.sound_title);
        self.check_sound_enabled
            .set_text(strings.check_sound_enabled);
        self.label_switch_type.set_text(strings.label_switch_type);
        self.label_volume.set_text(strings.label_volume);
        self.btn_test_sound.set_text(strings.btn_test_sound);

        let cur_switch = self.combo_switch_type.get_selected().unwrap_or(0);
        let profiles = crate::engine::config_store::list_switch_profiles();
        if !profiles.is_empty() {
            let profile_refs: Vec<&str> = profiles.iter().map(|s| s.as_str()).collect();
            self.combo_switch_type
                .reset_items(&profile_refs, Some(cur_switch));
        }
    }
}

