//! Main Control Panel View for MKey using Win32 Components

use crate::engine::{config::InputMethod, EngineConfig};
use crate::ui::components::*;

// Control IDs
pub const IDC_COMBO_METHOD: u32 = 301;
pub const IDC_COMBO_MODE: u32 = 302;
pub const IDC_CHECK_CTRL_SHIFT: u32 = 303;

pub const IDC_TAB_MAIN: u32 = 400;

// Tab 1: Bộ gõ
pub const IDC_CHECK_SPELLING: u32 = 401;
pub const IDC_CHECK_RESTORE_WRONG: u32 = 402;
pub const IDC_CHECK_AUTO_UPPER: u32 = 405;

// Tab 2: Gõ tắt
pub const IDC_CHECK_USE_MACRO: u32 = 406;
pub const IDC_EDIT_MACRO_KEY: u32 = 407;
pub const IDC_EDIT_MACRO_VALUE: u32 = 408;
pub const IDC_BTN_ADD_MACRO: u32 = 409;
pub const IDC_LIST_MACRO: u32 = 410;
pub const IDC_BTN_DEL_MACRO: u32 = 411;
pub const IDC_BTN_CANCEL_MACRO: u32 = 412;
pub const IDC_BTN_EDIT_MACRO: u32 = 413;

// Tab 3: Hệ thống
pub const IDC_CHECK_AUTOSTART: u32 = 420;
pub const IDC_CHECK_DEBUG_LOG: u32 = 421;
pub const IDC_BTN_OPEN_LOG: u32 = 422;

// Footer Buttons
pub const IDC_BTN_EXIT: u32 = 500;
pub const IDC_BTN_DEFAULTS: u32 = 501;
pub const IDC_BTN_CLOSE: u32 = 502;

pub struct ControlPanelControls {
    pub combo_method: ComboBox,
    pub combo_mode: ComboBox,
    pub check_ctrl_shift: CheckBox,
    pub tab_control: TabControl,

    // Tab 1 Controls
    pub check_spelling: CheckBox,
    pub check_restore_wrong: CheckBox,
    pub check_auto_upper: CheckBox,

    // Tab 2 Controls
    pub check_use_macro: CheckBox,
    pub edit_macro_key: TextBox,
    pub edit_macro_value: TextBox,
    pub btn_add_macro: PushButton,
    pub btn_edit_macro: PushButton,
    pub btn_cancel_macro: PushButton,
    pub btn_del_macro: PushButton,
    pub list_macro: ListView,

    // Tab 3 Controls
    pub check_autostart: CheckBox,
    pub check_debug_log: CheckBox,
    pub btn_open_log: PushButton,

    // Tab 4 Controls
    pub label_about_title: Label,
    pub label_about_ver: Label,
    pub label_about_author: Label,

    // Footer
    pub btn_exit: PushButton,
    pub btn_defaults: PushButton,
    pub btn_close: PushButton,
}

impl ControlPanelControls {
    pub fn create(parent: isize, hfont_normal: isize, hfont_bold: isize) -> Option<Self> {
        // 1. Header: Kiểu gõ, Chế độ gõ & Phím chuyển
        Label::create(parent, "Kiểu gõ:", 20, 18, 70, 20, hfont_bold);
        let combo_method = ComboBox::create(parent, IDC_COMBO_METHOD, 95, 14, 175, 150, hfont_normal)?;
        combo_method.add_item("Telex");
        combo_method.add_item("VNI");
        combo_method.add_item("Simple Telex");
        combo_method.set_selected(0);

        Label::create(parent, "Chế độ:", 20, 48, 70, 20, hfont_bold);
        let combo_mode = ComboBox::create(parent, IDC_COMBO_MODE, 95, 44, 175, 150, hfont_normal)?;
        combo_mode.add_item("Tiếng Việt");
        combo_mode.add_item("Tiếng Anh");
        combo_mode.set_selected(0);

        GroupBox::create(parent, "Phím chuyển", 285, 8, 130, 62, hfont_normal);
        let check_ctrl_shift = CheckBox::create(parent, IDC_CHECK_CTRL_SHIFT, "Ctrl + Shift", 298, 30, 105, 22, hfont_normal)?;
        check_ctrl_shift.set_checked(true);

        // 2. Tab Control
        let tab_control = TabControl::create(parent, IDC_TAB_MAIN, 15, 80, 400, 310, hfont_normal)?;
        tab_control.insert_tab(0, "Bộ gõ");
        tab_control.insert_tab(1, "Gõ tắt");
        tab_control.insert_tab(2, "Hệ thống");
        tab_control.insert_tab(3, "Thông tin");
        unsafe {
            use crate::ui::components::{SetWindowPos, HWND_BOTTOM, SWP_NOSIZE, SWP_NOMOVE, SWP_NOACTIVATE};
            SetWindowPos(tab_control.hwnd(), HWND_BOTTOM, 0, 0, 0, 0, SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE);
        }

        // 3. Tab 1: Bộ gõ Controls
        let check_spelling = CheckBox::create(parent, IDC_CHECK_SPELLING, "Bật kiểm tra chính tả", 30, 135, 360, 22, hfont_normal)?;
        let check_restore_wrong = CheckBox::create(parent, IDC_CHECK_RESTORE_WRONG, "Tự khôi phục phím khi sai từ", 30, 185, 360, 22, hfont_normal)?;
        let check_auto_upper = CheckBox::create(parent, IDC_CHECK_AUTO_UPPER, "Tự động viết hoa chữ cái đầu câu", 30, 235, 360, 22, hfont_normal)?;

        // 4. Tab 2: Gõ tắt Controls
        let check_use_macro = CheckBox::create(parent, IDC_CHECK_USE_MACRO, "Cho phép gõ tắt (Macro)", 30, 112, 360, 22, hfont_normal)?;
        let edit_macro_key = TextBox::create(parent, IDC_EDIT_MACRO_KEY, "", "Từ viết tắt", 30, 142, 75, 24, hfont_normal)?;
        let edit_macro_value = TextBox::create(parent, IDC_EDIT_MACRO_VALUE, "", "Cụm từ thay thế", 110, 142, 135, 24, hfont_normal)?;
        let btn_add_macro = PushButton::create(parent, IDC_BTN_ADD_MACRO, "+ Thêm", 255, 141, 135, 26, hfont_normal)?;
        let btn_edit_macro = PushButton::create(parent, IDC_BTN_EDIT_MACRO, "Sửa", 255, 141, 42, 26, hfont_normal)?;
        let btn_cancel_macro = PushButton::create(parent, IDC_BTN_CANCEL_MACRO, "Hủy", 300, 141, 42, 26, hfont_normal)?;
        let btn_del_macro = PushButton::create(parent, IDC_BTN_DEL_MACRO, "Xóa", 345, 141, 45, 26, hfont_normal)?;

        unsafe {
            use crate::ui::components::window::{ShowWindow, SW_HIDE};
            ShowWindow(btn_edit_macro.hwnd(), SW_HIDE);
            ShowWindow(btn_cancel_macro.hwnd(), SW_HIDE);
            ShowWindow(btn_del_macro.hwnd(), SW_HIDE);
        }

        let list_macro = ListView::create(parent, IDC_LIST_MACRO, 30, 173, 360, 207, hfont_normal)?;
        list_macro.add_column(0, "Từ viết tắt", 120);
        list_macro.add_column(1, "Cụm từ thay thế", 220);

        // 5. Tab 3: Hệ thống Controls
        let check_autostart = CheckBox::create(parent, IDC_CHECK_AUTOSTART, "Khởi động cùng Windows", 30, 125, 360, 22, hfont_normal)?;
        let check_debug_log = CheckBox::create(parent, IDC_CHECK_DEBUG_LOG, "Bật ghi nhật ký lỗi (Debug Log)", 30, 155, 360, 22, hfont_normal)?;
        let btn_open_log = PushButton::create(parent, IDC_BTN_OPEN_LOG, "Mở file log...", 30, 185, 120, 28, hfont_normal)?;

        // 6. Tab 4: Thông tin Controls
        let label_about_title = Label::create(parent, "MKey - Bộ gõ tiếng Việt hiện đại", 30, 130, 360, 25, hfont_bold)?;
        let label_about_ver = Label::create(parent, "Phiên bản: 0.1.0 (Rust Native Win32 Edition)\nSiêu nhẹ, hiệu năng cực cao, an toàn bộ nhớ.", 30, 160, 360, 45, hfont_normal)?;
        let label_about_author = Label::create(parent, "Tác giả phát triển: Mạnh Kiên\nBản quyền (C) 2026 Mạnh Kiên. All rights reserved.", 30, 215, 360, 45, hfont_normal)?;

        // 7. Footer Buttons: [ Kết thúc ] (trái) - [ Mặc định ] (giữa) - [ Đóng ] (phải)
        let btn_exit = PushButton::create(parent, IDC_BTN_EXIT, "Kết thúc", 20, 405, 100, 28, hfont_normal)?;
        let btn_defaults = PushButton::create(parent, IDC_BTN_DEFAULTS, "Mặc định", 165, 405, 100, 28, hfont_normal)?;
        let btn_close = PushButton::create(parent, IDC_BTN_CLOSE, "Đóng", 310, 405, 100, 28, hfont_normal)?;

        Some(Self {
            combo_method,
            combo_mode,
            check_ctrl_shift,
            tab_control,
            check_spelling,
            check_restore_wrong,
            check_auto_upper,
            check_use_macro,
            edit_macro_key,
            edit_macro_value,
            btn_add_macro,
            btn_edit_macro,
            btn_cancel_macro,
            btn_del_macro,
            list_macro,
            check_autostart,
            check_debug_log,
            btn_open_log,
            label_about_title,
            label_about_ver,
            label_about_author,
            btn_exit,
            btn_defaults,
            btn_close,
        })
    }

    /// Dynamically switches macro action buttons between Add mode (only "+ Thêm") and Edit mode ("Sửa", "Hủy", "Xóa")
    pub fn set_macro_edit_mode(&self, editing: bool) {
        unsafe {
            use crate::ui::components::window::{ShowWindow, SW_HIDE, SW_SHOW};
            use crate::ui::components::{SetWindowPos, HWND_TOP, SWP_NOSIZE, SWP_NOMOVE, SWP_SHOWWINDOW};

            if editing {
                ShowWindow(self.btn_add_macro.hwnd(), SW_HIDE);
                ShowWindow(self.btn_edit_macro.hwnd(), SW_SHOW);
                ShowWindow(self.btn_cancel_macro.hwnd(), SW_SHOW);
                ShowWindow(self.btn_del_macro.hwnd(), SW_SHOW);
                SetWindowPos(self.btn_edit_macro.hwnd(), HWND_TOP, 0, 0, 0, 0, SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW);
                SetWindowPos(self.btn_cancel_macro.hwnd(), HWND_TOP, 0, 0, 0, 0, SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW);
                SetWindowPos(self.btn_del_macro.hwnd(), HWND_TOP, 0, 0, 0, 0, SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW);
            } else {
                ShowWindow(self.btn_edit_macro.hwnd(), SW_HIDE);
                ShowWindow(self.btn_cancel_macro.hwnd(), SW_HIDE);
                ShowWindow(self.btn_del_macro.hwnd(), SW_HIDE);
                ShowWindow(self.btn_add_macro.hwnd(), SW_SHOW);
                SetWindowPos(self.btn_add_macro.hwnd(), HWND_TOP, 0, 0, 0, 0, SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW);
            }
        }
    }

    /// Shows or hides controls based on the selected tab index (0 to 3)
    pub fn update_tab_visibility(&self, tab_idx: usize) {
        unsafe {
            use crate::ui::components::window::ShowWindow;
            use crate::ui::components::window::{SW_HIDE, SW_SHOW};

            // Tab 1 controls
            let show_t1 = if tab_idx == 0 { SW_SHOW } else { SW_HIDE };
            ShowWindow(self.check_spelling.hwnd(), show_t1);
            ShowWindow(self.check_restore_wrong.hwnd(), show_t1);
            ShowWindow(self.check_auto_upper.hwnd(), show_t1);

            // Tab 2 controls
            let show_t2 = if tab_idx == 1 { SW_SHOW } else { SW_HIDE };
            ShowWindow(self.check_use_macro.hwnd(), show_t2);
            ShowWindow(self.edit_macro_key.hwnd(), show_t2);
            ShowWindow(self.edit_macro_value.hwnd(), show_t2);
            ShowWindow(self.list_macro.hwnd(), show_t2);

            if tab_idx == 1 {
                use crate::ui::components::{SetWindowPos, HWND_TOP, SWP_NOSIZE, SWP_NOMOVE, SWP_SHOWWINDOW, InvalidateRect, UpdateWindow};
                SetWindowPos(self.list_macro.hwnd(), HWND_TOP, 0, 0, 0, 0, SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW);
                SetWindowPos(self.edit_macro_key.hwnd(), HWND_TOP, 0, 0, 0, 0, SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW);
                SetWindowPos(self.edit_macro_value.hwnd(), HWND_TOP, 0, 0, 0, 0, SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW);
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

            // Tab 3 controls
            let show_t3 = if tab_idx == 2 { SW_SHOW } else { SW_HIDE };
            ShowWindow(self.check_autostart.hwnd(), show_t3);
            ShowWindow(self.check_debug_log.hwnd(), show_t3);
            ShowWindow(self.btn_open_log.hwnd(), show_t3);

            // Tab 4 controls
            let show_t4 = if tab_idx == 3 { SW_SHOW } else { SW_HIDE };
            ShowWindow(self.label_about_title.hwnd(), show_t4);
            ShowWindow(self.label_about_ver.hwnd(), show_t4);
            ShowWindow(self.label_about_author.hwnd(), show_t4);
        }
    }

    /// Loads current EngineConfig into checkboxes and comboboxes
    pub fn load_config(&self, config: &EngineConfig) {
        let method_idx = match config.method {
            InputMethod::Telex => 0,
            InputMethod::Vni => 1,
            InputMethod::SimpleTelex1 | InputMethod::SimpleTelex2 => 2,
        };
        self.combo_method.set_selected(method_idx);
        self.combo_mode.set_selected(if config.enabled { 0 } else { 1 });
        self.check_ctrl_shift.set_checked(config.switch_with_ctrl_shift);

        self.check_spelling.set_checked(config.check_spelling);
        self.check_restore_wrong.set_checked(config.restore_on_wrong_spelling);
        self.check_auto_upper.set_checked(config.auto_uppercase_first_char);
        self.check_use_macro.set_checked(config.use_macro);

        // Tab 3: Luôn đọc trực tiếp từ Registry làm nguồn chuẩn xác nhất
        self.check_autostart.set_checked(crate::platform::is_windows_autostart_enabled());
        self.check_debug_log.set_checked(config.debug);
    }

    /// Reads UI state into an EngineConfig
    pub fn read_config(&self, config: &mut EngineConfig) {
        if let Some(sel) = self.combo_method.get_selected() {
            let new_method = match sel {
                0 => InputMethod::Telex,
                1 => InputMethod::Vni,
                2 => InputMethod::SimpleTelex1,
                _ => InputMethod::Telex,
            };
            if config.method != new_method {
                config.method = new_method;
                println!("[MKey] >> Đổi kiểu gõ sang: {:?}", new_method);
            }
        }

        if let Some(mode_sel) = self.combo_mode.get_selected() {
            let new_enabled = mode_sel == 0;
            if config.enabled != new_enabled {
                config.enabled = new_enabled;
                println!("[MKey] >> Chế độ gõ: {}", if new_enabled { "[V] Tiếng Việt" } else { "[E] Tiếng Anh" });
            }
        }

        config.switch_with_ctrl_shift = self.check_ctrl_shift.is_checked();
        config.check_spelling = self.check_spelling.is_checked();
        config.restore_on_wrong_spelling = self.check_restore_wrong.is_checked();
        config.auto_uppercase_first_char = self.check_auto_upper.is_checked();
        config.use_macro = self.check_use_macro.is_checked();

        let new_autostart = self.check_autostart.is_checked();
        if crate::platform::is_windows_autostart_enabled() != new_autostart {
            config.autostart = new_autostart;
            crate::platform::set_windows_autostart(new_autostart);
            println!("[MKey] >> Khởi động cùng Windows: {}", if new_autostart { "BẬT" } else { "TẮT" });
        }

        let new_debug = self.check_debug_log.is_checked();
        if config.debug != new_debug {
            config.debug = new_debug;
            println!("[MKey] >> Ghi nhật ký lỗi (Debug Log): {}", if new_debug { "BẬT" } else { "TẮT" });
        }
    }

    /// Populates or updates the Macro ListView with given pairs
    pub fn populate_macros(&self, macros: &[(String, String)]) {
        self.list_macro.clear();
        for (i, (k, v)) in macros.iter().enumerate() {
            self.list_macro.add_item(i as i32, k, v);
        }
    }
}

