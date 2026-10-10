//! Main Control Panel View for MKey using Modern Win32 Fluent Card Components

pub mod footer;
pub mod header;
pub mod tab_about;
pub mod tab_macro;
pub mod tab_system;
pub mod tab_typing_sound;

pub use footer::*;
pub use header::*;
pub use tab_about::*;
pub use tab_macro::*;
pub use tab_system::*;
pub use tab_typing_sound::*;

use crate::engine::EngineConfig;
use crate::language::Language;
use crate::ui::components::*;

pub const IDC_TAB_MAIN: u32 = 400;

pub struct ControlPanelControls {
    pub header: HeaderCard,
    pub tab_bar: TabBar,
    pub tab_typing: TabTypingSound,
    pub tab_macro: TabMacro,
    pub tab_system: TabSystem,
    pub tab_about: TabAbout,
    pub footer: FooterControls,
}

impl ControlPanelControls {
    pub fn create(parent: isize, hfont_normal: isize, hfont_bold: isize) -> Option<Self> {
        let header = HeaderCard::create(parent, hfont_normal, hfont_bold)?;

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

        let tab_typing = TabTypingSound::create(parent, hfont_normal, hfont_bold)?;
        let tab_macro = TabMacro::create(parent, hfont_normal)?;
        let tab_system = TabSystem::create(parent, hfont_normal, hfont_bold)?;
        let tab_about = TabAbout::create(parent, hfont_normal, hfont_bold)?;
        let footer = FooterControls::create(parent, hfont_normal)?;

        Some(Self {
            header,
            tab_bar,
            tab_typing,
            tab_macro,
            tab_system,
            tab_about,
            footer,
        })
    }

    /// Dynamically enables or disables child sound controls based on whether `check_sound_enabled` is checked
    pub fn update_sound_controls_state(&self) {
        self.tab_typing.update_sound_controls_state();
    }

    /// Dynamically enables or disables child macro checkboxes based on whether `check_use_macro` is checked
    pub fn update_macro_checkboxes_state(&self) {
        self.tab_macro.update_macro_checkboxes_state();
    }

    /// Dynamically switches macro action buttons between Add mode (only "+ Thêm") and Edit mode ("Sửa", "Hủy", "Xóa")
    pub fn set_macro_edit_mode(&self, editing: bool) {
        self.tab_macro.set_macro_edit_mode(editing);
    }

    /// Shows or hides controls based on the selected tab index (0 to 3)
    pub fn update_tab_visibility(&self, tab_idx: usize) {
        self.tab_typing.update_visibility(tab_idx == 0);
        self.tab_macro.update_visibility(tab_idx == 1);
        self.tab_system.update_visibility(tab_idx == 2);
        self.tab_about.update_visibility(tab_idx == 3);
    }

    /// Loads current EngineConfig into checkboxes and comboboxes
    pub fn load_config(&self, config: &EngineConfig) {
        self.header.load_config(config);
        self.tab_typing.load_config(config);
        self.tab_macro.load_config(config);
        self.tab_system.load_config(config);
    }

    /// Reads UI state into an EngineConfig
    pub fn read_config(&self, config: &mut EngineConfig) {
        self.header.read_config(config);
        self.tab_typing.read_config(config);
        self.tab_macro.read_config(config);
        self.tab_system.read_config(config);
    }

    /// Populates or updates the Macro ListView with given entries
    pub fn populate_macros(&self, macros: &[crate::engine::macro_table::MacroEntry]) {
        self.tab_macro.populate_macros(macros);
    }

    /// Dynamically refreshes all UI text labels, buttons, combobox options, and titles to match the given language
    pub fn update_language(&self, lang: Language) {
        let strings = crate::language::get_strings(lang);
        self.header.update_language(strings);
        self.tab_typing.update_language(strings);
        self.tab_macro.update_language(strings);
        self.tab_system.update_language(lang, strings);
        self.tab_about.update_language(strings);
        self.footer.update_language(strings);
    }
}

