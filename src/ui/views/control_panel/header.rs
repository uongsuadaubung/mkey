//! Header card controls (Input Method, Mode, Switch shortcut)

use crate::engine::config::InputMethod;
use crate::language::LanguageStrings;
use crate::ui::components::*;

pub const IDC_COMBO_METHOD: u32 = 301;
pub const IDC_COMBO_MODE: u32 = 302;
pub const IDC_CHECK_CTRL_SHIFT: u32 = 303;

pub struct HeaderCard {
    pub label_method: Label,
    pub label_mode: Label,
    pub label_switch: Label,
    pub combo_method: ComboBox,
    pub combo_mode: ComboBox,
    pub check_ctrl_shift: CheckBox,
}

impl HeaderCard {
    pub fn create(parent: isize, hfont_normal: isize, hfont_bold: isize) -> Option<Self> {
        let strings = crate::language::current();

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

        Some(Self {
            label_method,
            label_mode,
            label_switch,
            combo_method,
            combo_mode,
            check_ctrl_shift,
        })
    }

    pub fn load_config(&self, config: &crate::engine::EngineConfig) {
        self.combo_method.set_selected(config.method.into());
        self.combo_mode
            .set_selected(if config.enabled { 0 } else { 1 });
        self.check_ctrl_shift
            .set_checked(config.switch_with_ctrl_shift);
    }

    pub fn read_config(&self, config: &mut crate::engine::EngineConfig) {
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
    }

    pub fn update_language(&self, strings: &LanguageStrings) {
        self.label_method.set_text(strings.label_method);
        self.label_mode.set_text(strings.label_mode);
        self.label_switch.set_text(strings.label_switch);
        self.check_ctrl_shift.set_text(strings.check_ctrl_shift);

        let cur_mode = self.combo_mode.get_selected().unwrap_or(0);
        self.combo_mode.reset_items(
            &[strings.mode_vietnamese, strings.mode_english],
            Some(cur_mode),
        );
    }
}

