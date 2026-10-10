//! Main Control Panel View for MKey using Modern Win32 Fluent Card Components

use crate::engine::{EngineConfig, config::InputMethod};
use crate::ui::components::*;

// Control IDs
pub const IDC_COMBO_METHOD: u32 = 301;
pub const IDC_COMBO_MODE: u32 = 302;
pub const IDC_CHECK_CTRL_SHIFT: u32 = 303;

pub const IDC_TAB_MAIN: u32 = 400;

// Tab 0: Bộ gõ & Âm thanh
pub const IDC_CHECK_RESTORE_WRONG: u32 = 402;
pub const IDC_CHECK_AUTO_UPPER: u32 = 405;
pub const IDC_CHECK_SOUND_ENABLED: u32 = 440;
pub const IDC_COMBO_SWITCH_TYPE: u32 = 441;
pub const IDC_SLIDER_SOUND_VOLUME: u32 = 442;
pub const IDC_BTN_TEST_SOUND: u32 = 443;
pub const IDC_LABEL_VOLUME_VAL: u32 = 444;

// Tab 1: Gõ tắt
pub const IDC_CHECK_USE_MACRO: u32 = 406;
pub const IDC_CHECK_MACRO_EN: u32 = 414;
pub const IDC_EDIT_MACRO_KEY: u32 = 407;
pub const IDC_EDIT_MACRO_VALUE: u32 = 408;
pub const IDC_BTN_ADD_MACRO: u32 = 409;
pub const IDC_LIST_MACRO: u32 = 410;
pub const IDC_BTN_DEL_MACRO: u32 = 411;
pub const IDC_BTN_CANCEL_MACRO: u32 = 412;
pub const IDC_BTN_EDIT_MACRO: u32 = 413;
pub const IDC_COMBO_MACRO_TYPE: u32 = 417;

// Tab 2: Hệ thống
pub const IDC_CHECK_AUTOSTART: u32 = 420;
pub const IDC_CHECK_SHOW_DIALOG: u32 = 424;
pub const IDC_CHECK_DEBUG_LOG: u32 = 421;
pub const IDC_BTN_OPEN_LOG: u32 = 422;
pub const IDC_COMBO_THEME: u32 = 423;
pub const IDC_COMBO_LANG: u32 = 425;
pub const IDC_BTN_CHECK_UPDATE: u32 = 426;

// Tab 3: Thông tin
pub const IDC_LABEL_EMAIL: u32 = 430;
pub const IDC_LABEL_GITHUB: u32 = 431;

// Footer Buttons
pub const IDC_BTN_EXIT: u32 = 500;
pub const IDC_BTN_DEFAULTS: u32 = 501;
pub const IDC_BTN_CLOSE: u32 = 502;

pub struct ControlPanelControls {
    // Header Card
    pub label_method: Label,
    pub label_mode: Label,
    pub label_switch: Label,
    pub combo_method: ComboBox,
    pub combo_mode: ComboBox,
    pub check_ctrl_shift: CheckBox,

    // Navigation TabBar
    pub tab_bar: TabBar,

    // Tab 0 Controls
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

    // Tab 1 Controls
    pub check_use_macro: CheckBox,
    pub check_macro_in_english: CheckBox,
    pub edit_macro_key: TextBox,
    pub edit_macro_value: TextBox,
    pub combo_macro_type: ComboBox,
    pub btn_add_macro: PushButton,
    pub btn_edit_macro: PushButton,
    pub btn_cancel_macro: PushButton,
    pub btn_del_macro: PushButton,
    pub list_macro: ListView,

    // Tab 2 Controls
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

    // Tab 3 Controls
    pub label_about_title: Label,
    pub label_about_ver: Label,
    pub label_about_author: Label,
    pub label_about_email: Label,
    pub label_about_github: Label,

    // Footer
    pub btn_exit: PushButton,
    pub btn_defaults: PushButton,
    pub btn_close: PushButton,
}

impl ControlPanelControls {
    pub fn create(parent: isize, hfont_normal: isize, hfont_bold: isize) -> Option<Self> {
        let strings = crate::language::current();

        // 1. Header Card controls (x: 20..440, y: 14..78)
        let label_method = Label::create(parent, strings.label_method, 35, 26, 65, 20, hfont_bold)?;
        let combo_method =
            ComboBox::create(parent, IDC_COMBO_METHOD, 105, 22, 155, 150, hfont_normal)?;
        combo_method.add_item(strings.method_telex);
        combo_method.add_item(strings.method_vni);
        combo_method.add_item(strings.method_simple_telex);
        combo_method.set_selected(0);

        let label_mode = Label::create(parent, strings.label_mode, 35, 52, 65, 20, hfont_bold)?;
        let combo_mode = ComboBox::create(parent, IDC_COMBO_MODE, 105, 48, 155, 150, hfont_normal)?;
        combo_mode.add_item(strings.mode_vietnamese);
        combo_mode.add_item(strings.mode_english);
        combo_mode.set_selected(0);

        let label_switch =
            Label::create(parent, strings.label_switch, 280, 26, 95, 20, hfont_bold)?;
        let check_ctrl_shift = CheckBox::create(
            parent,
            IDC_CHECK_CTRL_SHIFT,
            strings.check_ctrl_shift,
            280,
            48,
            120,
            22,
            hfont_normal,
        )?;
        check_ctrl_shift.set_checked(true);

        // 2. Modern Segmented Pill TabBar (x: 20, y: 88, width: 420, height: 36)
        let tab_bar = TabBar::create(
            parent,
            IDC_TAB_MAIN,
            20,
            88,
            420,
            36,
            hfont_normal,
            hfont_bold,
        )?;

        // 3. Tab 0: Bộ gõ & Âm thanh Controls (Body Card y: 134..444)
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

        // 4. Tab 1: Gõ tắt Controls
        let check_use_macro = CheckBox::create(
            parent,
            IDC_CHECK_USE_MACRO,
            strings.check_use_macro,
            40,
            146,
            360,
            24,
            hfont_normal,
        )?;
        let check_macro_in_english = CheckBox::create(
            parent,
            IDC_CHECK_MACRO_EN,
            strings.check_macro_in_english,
            40,
            176,
            360,
            24,
            hfont_normal,
        )?;
        let edit_macro_key = TextBox::create(
            parent,
            IDC_EDIT_MACRO_KEY,
            "",
            strings.edit_macro_key_placeholder,
            40,
            206,
            75,
            24,
            hfont_normal,
        )?;
        let edit_macro_value = TextBox::create(
            parent,
            IDC_EDIT_MACRO_VALUE,
            "",
            strings.edit_macro_val_placeholder,
            120,
            206,
            150,
            24,
            hfont_normal,
        )?;
        let combo_macro_type = ComboBox::create(
            parent,
            IDC_COMBO_MACRO_TYPE,
            275,
            206,
            125,
            120,
            hfont_normal,
        )?;
        combo_macro_type.add_item(strings.macro_type_normal);
        combo_macro_type.add_item(strings.macro_type_start);
        combo_macro_type.add_item(strings.macro_type_end);
        combo_macro_type.set_selected(0);

        let btn_add_macro = PushButton::create(
            parent,
            IDC_BTN_ADD_MACRO,
            strings.btn_add_macro,
            310,
            236,
            90,
            26,
            hfont_normal,
        )?;
        let btn_edit_macro = PushButton::create(
            parent,
            IDC_BTN_EDIT_MACRO,
            strings.btn_edit_macro,
            140,
            236,
            80,
            26,
            hfont_normal,
        )?;
        let btn_del_macro = PushButton::create(
            parent,
            IDC_BTN_DEL_MACRO,
            strings.btn_del_macro,
            230,
            236,
            80,
            26,
            hfont_normal,
        )?;
        let btn_cancel_macro = PushButton::create(
            parent,
            IDC_BTN_CANCEL_MACRO,
            strings.btn_cancel_macro,
            320,
            236,
            80,
            26,
            hfont_normal,
        )?;

        unsafe {
            use crate::ui::components::window::{SW_HIDE, ShowWindow};
            ShowWindow(btn_edit_macro.hwnd(), SW_HIDE);
            ShowWindow(btn_cancel_macro.hwnd(), SW_HIDE);
            ShowWindow(btn_del_macro.hwnd(), SW_HIDE);
        }

        let list_macro = ListView::create(parent, IDC_LIST_MACRO, 40, 268, 360, 168, hfont_normal)?;
        list_macro.add_column(0, strings.col_macro_key, 85);
        list_macro.add_column(1, strings.col_macro_val, 170);
        list_macro.add_column(2, strings.col_macro_type, 95);

        // 5. Tab 2: Hệ thống Controls
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

        // 6. Tab 3: Thông tin Controls
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

        // 7. Footer Buttons: [ Kết thúc ] (trái) - [ Mặc định ] (giữa) - [ Đóng ] (phải)
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

        let controls = Self {
            label_method,
            label_mode,
            label_switch,
            combo_method,
            combo_mode,
            check_ctrl_shift,
            tab_bar,
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
            check_use_macro,
            check_macro_in_english,
            edit_macro_key,
            edit_macro_value,
            combo_macro_type,
            btn_add_macro,
            btn_edit_macro,
            btn_cancel_macro,
            btn_del_macro,
            list_macro,
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
            label_about_title,
            label_about_ver,
            label_about_author,
            label_about_email,
            label_about_github,
            btn_exit,
            btn_defaults,
            btn_close,
        };
        controls.update_macro_checkboxes_state();
        controls.update_sound_controls_state();
        Some(controls)
    }

    /// Dynamically enables or disables child sound controls based on whether `check_sound_enabled` is checked
    pub fn update_sound_controls_state(&self) {
        let is_enabled = self.sound_available && self.check_sound_enabled.is_checked();
        self.combo_switch_type.set_enabled(is_enabled);
        self.btn_test_sound.set_enabled(is_enabled);
        self.slider_volume.set_enabled(is_enabled);
    }

    /// Dynamically enables or disables child macro checkboxes based on whether `check_use_macro` is checked
    pub fn update_macro_checkboxes_state(&self) {
        let is_enabled = self.check_use_macro.is_checked();
        self.check_macro_in_english.set_enabled(is_enabled);
        self.combo_macro_type.set_enabled(is_enabled);
    }

    /// Dynamically switches macro action buttons between Add mode (only "+ Thêm") and Edit mode ("Sửa", "Hủy", "Xóa")
    pub fn set_macro_edit_mode(&self, editing: bool) {
        unsafe {
            use crate::ui::components::window::{SW_HIDE, SW_SHOW, ShowWindow};
            use crate::ui::components::{
                HWND_TOP, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SetWindowPos,
            };

            if editing {
                ShowWindow(self.btn_add_macro.hwnd(), SW_HIDE);
                ShowWindow(self.btn_edit_macro.hwnd(), SW_SHOW);
                ShowWindow(self.btn_cancel_macro.hwnd(), SW_SHOW);
                ShowWindow(self.btn_del_macro.hwnd(), SW_SHOW);
                SetWindowPos(
                    self.btn_edit_macro.hwnd(),
                    HWND_TOP,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW,
                );
                SetWindowPos(
                    self.btn_cancel_macro.hwnd(),
                    HWND_TOP,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW,
                );
                SetWindowPos(
                    self.btn_del_macro.hwnd(),
                    HWND_TOP,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW,
                );
            } else {
                ShowWindow(self.btn_edit_macro.hwnd(), SW_HIDE);
                ShowWindow(self.btn_cancel_macro.hwnd(), SW_HIDE);
                ShowWindow(self.btn_del_macro.hwnd(), SW_HIDE);
                ShowWindow(self.btn_add_macro.hwnd(), SW_SHOW);
                SetWindowPos(
                    self.btn_add_macro.hwnd(),
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

    /// Shows or hides controls based on the selected tab index (0 to 3)
    pub fn update_tab_visibility(&self, tab_idx: usize) {
        unsafe {
            use crate::ui::components::window::ShowWindow;
            use crate::ui::components::window::{SW_HIDE, SW_SHOW};

            // Tab 0 controls
            let show_t0 = if tab_idx == 0 { SW_SHOW } else { SW_HIDE };
            ShowWindow(self.label_typing_title.hwnd(), show_t0);
            ShowWindow(self.check_restore_wrong.hwnd(), show_t0);
            ShowWindow(self.check_auto_upper.hwnd(), show_t0);

            let show_sound = if tab_idx == 0 && self.sound_available {
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

            if tab_idx == 0 && self.sound_available {
                use crate::ui::components::{
                    HWND_TOP, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SetWindowPos,
                };
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

            // Tab 1 controls
            let show_t1 = if tab_idx == 1 { SW_SHOW } else { SW_HIDE };
            ShowWindow(self.check_use_macro.hwnd(), show_t1);
            ShowWindow(self.check_macro_in_english.hwnd(), show_t1);
            ShowWindow(self.edit_macro_key.hwnd(), show_t1);
            ShowWindow(self.edit_macro_value.hwnd(), show_t1);
            ShowWindow(self.combo_macro_type.hwnd(), show_t1);
            ShowWindow(self.list_macro.hwnd(), show_t1);

            if tab_idx == 1 {
                use crate::ui::components::{
                    HWND_TOP, InvalidateRect, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SetWindowPos,
                    UpdateWindow,
                };
                SetWindowPos(
                    self.list_macro.hwnd(),
                    HWND_TOP,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW,
                );
                SetWindowPos(
                    self.edit_macro_key.hwnd(),
                    HWND_TOP,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW,
                );
                SetWindowPos(
                    self.edit_macro_value.hwnd(),
                    HWND_TOP,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW,
                );
                SetWindowPos(
                    self.combo_macro_type.hwnd(),
                    HWND_TOP,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW,
                );
                InvalidateRect(self.list_macro.hwnd(), std::ptr::null(), 1);
                UpdateWindow(self.list_macro.hwnd());

                let is_editing = self.list_macro.get_selected_index().is_some();
                self.set_macro_edit_mode(is_editing);
            } else {
                ShowWindow(self.btn_add_macro.hwnd(), SW_HIDE);
                ShowWindow(self.btn_edit_macro.hwnd(), SW_HIDE);
                ShowWindow(self.btn_cancel_macro.hwnd(), SW_HIDE);
                ShowWindow(self.btn_del_macro.hwnd(), SW_HIDE);
            }

            // Tab 2 controls
            let show_t2 = if tab_idx == 2 { SW_SHOW } else { SW_HIDE };
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

            // Tab 3 controls
            let show_t3 = if tab_idx == 3 { SW_SHOW } else { SW_HIDE };
            ShowWindow(self.label_about_title.hwnd(), show_t3);
            ShowWindow(self.label_about_ver.hwnd(), show_t3);
            ShowWindow(self.label_about_author.hwnd(), show_t3);
            ShowWindow(self.label_about_email.hwnd(), show_t3);
            ShowWindow(self.label_about_github.hwnd(), show_t3);
        }
    }

    /// Loads current EngineConfig into checkboxes and comboboxes
    pub fn load_config(&self, config: &EngineConfig) {
        self.combo_method.set_selected(config.method.into());
        self.combo_mode
            .set_selected(if config.enabled { 0 } else { 1 });
        self.check_ctrl_shift
            .set_checked(config.switch_with_ctrl_shift);

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
        self.check_use_macro.set_checked(config.use_macro);
        self.check_macro_in_english
            .set_checked(config.use_macro_in_english_mode);
        self.update_macro_checkboxes_state();

        // Tab 2: Luôn đọc trực tiếp từ Registry làm nguồn chuẩn xác nhất
        self.check_autostart
            .set_checked(crate::platform::is_windows_autostart_enabled());
        self.check_show_dialog
            .set_checked(config.show_dialog_on_startup);
        self.check_debug_log.set_checked(config.debug);

        self.combo_theme.set_selected(config.theme.into());
        self.combo_lang.set_selected(config.language.into());
    }

    /// Reads UI state into an EngineConfig
    pub fn read_config(&self, config: &mut EngineConfig) {
        if let Some(sel) = self.combo_method.get_selected() {
            let new_method = InputMethod::from(sel);
            if config.method != new_method {
                config.method = new_method;
                println!("[MKey] >> Đổi kiểu gõ sang: {:?}", new_method);
            }
        }

        if let Some(mode_sel) = self.combo_mode.get_selected() {
            let new_enabled = mode_sel == 0;
            if config.enabled != new_enabled {
                config.enabled = new_enabled;
                println!(
                    "[MKey] >> Chế độ gõ: {}",
                    if new_enabled {
                        "[V] Tiếng Việt"
                    } else {
                        "[E] Tiếng Anh"
                    }
                );
            }
        }

        config.switch_with_ctrl_shift = self.check_ctrl_shift.is_checked();
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

        config.use_macro = self.check_use_macro.is_checked();
        config.use_macro_in_english_mode = self.check_macro_in_english.is_checked();

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

    /// Populates or updates the Macro ListView with given entries
    pub fn populate_macros(&self, macros: &[crate::engine::macro_table::MacroEntry]) {
        self.list_macro.clear();
        let strings = crate::language::current();
        for (i, entry) in macros.iter().enumerate() {
            let type_str = match entry.macro_type {
                crate::engine::macro_table::MacroType::Normal => strings.macro_type_normal,
                crate::engine::macro_table::MacroType::StartConsonant => strings.macro_type_start,
                crate::engine::macro_table::MacroType::EndConsonant => strings.macro_type_end,
            };
            self.list_macro
                .add_item(i as i32, &entry.key, &entry.value, type_str);
        }
    }

    /// Dynamically refreshes all UI text labels, buttons, combobox options, and titles to match the given language
    pub fn update_language(&self, lang: crate::language::Language) {
        let strings = crate::language::get_strings(lang);
        self.label_method.set_text(strings.label_method);
        self.label_mode.set_text(strings.label_mode);
        self.label_switch.set_text(strings.label_switch);
        self.check_ctrl_shift.set_text(strings.check_ctrl_shift);

        let cur_mode = self.combo_mode.get_selected().unwrap_or(0);
        self.combo_mode.reset_items(
            &[strings.mode_vietnamese, strings.mode_english],
            Some(cur_mode),
        );

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

        self.check_use_macro.set_text(strings.check_use_macro);
        self.check_macro_in_english
            .set_text(strings.check_macro_in_english);
        let cur_macro_type = self.combo_macro_type.get_selected().unwrap_or(0);
        self.combo_macro_type.reset_items(
            &[
                strings.macro_type_normal,
                strings.macro_type_start,
                strings.macro_type_end,
            ],
            Some(cur_macro_type),
        );
        self.edit_macro_key
            .set_cue_banner(strings.edit_macro_key_placeholder);
        self.edit_macro_value
            .set_cue_banner(strings.edit_macro_val_placeholder);
        self.btn_add_macro.set_text(strings.btn_add_macro);
        self.btn_edit_macro.set_text(strings.btn_edit_macro);
        self.btn_cancel_macro.set_text(strings.btn_cancel_macro);
        self.btn_del_macro.set_text(strings.btn_del_macro);
        self.list_macro.refresh_header();

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

        self.label_about_title.set_text(strings.about_title);
        self.label_about_ver.set_text(strings.about_ver);
        self.label_about_author.set_text(strings.about_author);
        self.label_about_email.set_text(strings.about_email);
        self.label_about_github.set_text(strings.about_github);

        self.btn_exit.set_text(strings.btn_exit);
        self.btn_defaults.set_text(strings.btn_defaults);
        self.btn_close.set_text(strings.btn_close);
    }
}
