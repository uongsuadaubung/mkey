//! Tab 1: Macro controls (Gõ tắt)

use crate::language::LanguageStrings;
use crate::ui::components::window::{SW_HIDE, SW_SHOW, ShowWindow};
use crate::ui::components::*;

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

pub struct TabMacro {
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
}

impl TabMacro {
    pub fn create(parent: isize, hfont_normal: isize) -> Option<Self> {
        let strings = crate::language::current();

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
            ShowWindow(btn_edit_macro.hwnd(), SW_HIDE);
            ShowWindow(btn_cancel_macro.hwnd(), SW_HIDE);
            ShowWindow(btn_del_macro.hwnd(), SW_HIDE);
        }

        let list_macro = ListView::create(parent, IDC_LIST_MACRO, 40, 268, 360, 168, hfont_normal)?;
        list_macro.add_column(0, strings.col_macro_key, 85);
        list_macro.add_column(1, strings.col_macro_val, 170);
        list_macro.add_column(2, strings.col_macro_type, 95);

        let tab = Self {
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
        };
        tab.update_macro_checkboxes_state();
        Some(tab)
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

    pub fn update_visibility(&self, visible: bool) {
        unsafe {
            let show_t1 = if visible { SW_SHOW } else { SW_HIDE };
            ShowWindow(self.check_use_macro.hwnd(), show_t1);
            ShowWindow(self.check_macro_in_english.hwnd(), show_t1);
            ShowWindow(self.edit_macro_key.hwnd(), show_t1);
            ShowWindow(self.edit_macro_value.hwnd(), show_t1);
            ShowWindow(self.combo_macro_type.hwnd(), show_t1);
            ShowWindow(self.list_macro.hwnd(), show_t1);

            if visible {
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
        }
    }

    pub fn load_config(&self, config: &crate::engine::EngineConfig) {
        self.check_use_macro.set_checked(config.use_macro);
        self.check_macro_in_english
            .set_checked(config.use_macro_in_english_mode);
        self.update_macro_checkboxes_state();
    }

    pub fn read_config(&self, config: &mut crate::engine::EngineConfig) {
        config.use_macro = self.check_use_macro.is_checked();
        config.use_macro_in_english_mode = self.check_macro_in_english.is_checked();
    }

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

    pub fn update_language(&self, strings: &LanguageStrings) {
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
    }
}
