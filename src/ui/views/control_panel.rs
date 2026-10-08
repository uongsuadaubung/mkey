//! Main Control Panel View for MKey using Modern Win32 Fluent Card Components

use crate::engine::{EngineConfig, config::InputMethod};
use crate::ui::components::*;

// Control IDs
pub const IDC_COMBO_METHOD: u32 = 301;
pub const IDC_COMBO_MODE: u32 = 302;
pub const IDC_CHECK_CTRL_SHIFT: u32 = 303;

pub const IDC_TAB_MAIN: u32 = 400;

// Tab 0: Bộ gõ
pub const IDC_CHECK_SPELLING: u32 = 401;
pub const IDC_CHECK_RESTORE_WRONG: u32 = 402;
pub const IDC_CHECK_AUTO_UPPER: u32 = 405;

// Tab 1: Gõ tắt
pub const IDC_CHECK_USE_MACRO: u32 = 406;
pub const IDC_EDIT_MACRO_KEY: u32 = 407;
pub const IDC_EDIT_MACRO_VALUE: u32 = 408;
pub const IDC_BTN_ADD_MACRO: u32 = 409;
pub const IDC_LIST_MACRO: u32 = 410;
pub const IDC_BTN_DEL_MACRO: u32 = 411;
pub const IDC_BTN_CANCEL_MACRO: u32 = 412;
pub const IDC_BTN_EDIT_MACRO: u32 = 413;

// Tab 2: Hệ thống
pub const IDC_CHECK_AUTOSTART: u32 = 420;
pub const IDC_CHECK_SHOW_DIALOG: u32 = 424;
pub const IDC_CHECK_DEBUG_LOG: u32 = 421;
pub const IDC_BTN_OPEN_LOG: u32 = 422;
pub const IDC_COMBO_THEME: u32 = 423;

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
    pub check_spelling: CheckBox,
    pub check_restore_wrong: CheckBox,
    pub check_auto_upper: CheckBox,

    // Tab 1 Controls
    pub check_use_macro: CheckBox,
    pub edit_macro_key: TextBox,
    pub edit_macro_value: TextBox,
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
        // 1. Header Card controls (x: 20..440, y: 14..78)
        let label_method = Label::create(parent, "Kiểu gõ:", 35, 26, 65, 20, hfont_bold)?;
        let combo_method =
            ComboBox::create(parent, IDC_COMBO_METHOD, 105, 22, 155, 150, hfont_normal)?;
        combo_method.add_item("Telex");
        combo_method.add_item("VNI");
        combo_method.add_item("Simple Telex");
        combo_method.set_selected(0);

        let label_mode = Label::create(parent, "Chế độ:", 35, 52, 65, 20, hfont_bold)?;
        let combo_mode = ComboBox::create(parent, IDC_COMBO_MODE, 105, 48, 155, 150, hfont_normal)?;
        combo_mode.add_item("Tiếng Việt");
        combo_mode.add_item("Tiếng Anh");
        combo_mode.set_selected(0);

        let label_switch = Label::create(parent, "Phím chuyển:", 280, 26, 95, 20, hfont_bold)?;
        let check_ctrl_shift = CheckBox::create(
            parent,
            IDC_CHECK_CTRL_SHIFT,
            "Ctrl + Shift",
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

        // 3. Tab 0: Bộ gõ Controls (Body Card y: 134..444)
        let label_typing_title = Label::create(
            parent,
            "Tính năng hỗ trợ gõ & kiểm tra chính tả",
            40,
            150,
            360,
            22,
            hfont_bold,
        )?;
        let check_spelling = CheckBox::create(
            parent,
            IDC_CHECK_SPELLING,
            "Bật kiểm tra chính tả tiếng Việt",
            40,
            185,
            360,
            24,
            hfont_normal,
        )?;
        let check_restore_wrong = CheckBox::create(
            parent,
            IDC_CHECK_RESTORE_WRONG,
            "Tự động khôi phục phím khi gõ sai từ",
            40,
            225,
            360,
            24,
            hfont_normal,
        )?;
        let check_auto_upper = CheckBox::create(
            parent,
            IDC_CHECK_AUTO_UPPER,
            "Tự động viết hoa chữ cái đầu câu",
            40,
            265,
            360,
            24,
            hfont_normal,
        )?;

        // 4. Tab 1: Gõ tắt Controls
        let check_use_macro = CheckBox::create(
            parent,
            IDC_CHECK_USE_MACRO,
            "Cho phép sử dụng bảng gõ tắt (Macro)",
            40,
            148,
            360,
            24,
            hfont_normal,
        )?;
        let edit_macro_key = TextBox::create(
            parent,
            IDC_EDIT_MACRO_KEY,
            "",
            "Từ viết tắt",
            40,
            180,
            85,
            24,
            hfont_normal,
        )?;
        let edit_macro_value = TextBox::create(
            parent,
            IDC_EDIT_MACRO_VALUE,
            "",
            "Cụm từ thay thế",
            135,
            180,
            145,
            24,
            hfont_normal,
        )?;
        let btn_add_macro = PushButton::create(
            parent,
            IDC_BTN_ADD_MACRO,
            "+ Thêm",
            290,
            179,
            110,
            26,
            hfont_normal,
        )?;
        let btn_edit_macro = PushButton::create(
            parent,
            IDC_BTN_EDIT_MACRO,
            "Sửa",
            290,
            179,
            34,
            26,
            hfont_normal,
        )?;
        let btn_cancel_macro = PushButton::create(
            parent,
            IDC_BTN_CANCEL_MACRO,
            "Hủy",
            328,
            179,
            34,
            26,
            hfont_normal,
        )?;
        let btn_del_macro = PushButton::create(
            parent,
            IDC_BTN_DEL_MACRO,
            "Xóa",
            366,
            179,
            34,
            26,
            hfont_normal,
        )?;

        unsafe {
            use crate::ui::components::window::{SW_HIDE, ShowWindow};
            ShowWindow(btn_edit_macro.hwnd(), SW_HIDE);
            ShowWindow(btn_cancel_macro.hwnd(), SW_HIDE);
            ShowWindow(btn_del_macro.hwnd(), SW_HIDE);
        }

        let list_macro = ListView::create(parent, IDC_LIST_MACRO, 40, 214, 360, 215, hfont_normal)?;
        list_macro.add_column(0, "Từ viết tắt", 115);
        list_macro.add_column(1, "Cụm từ thay thế", 243);

        // 5. Tab 2: Hệ thống Controls
        let label_sys_title =
            Label::create(parent, "Khởi động & Nhật ký", 40, 150, 360, 22, hfont_bold)?;
        let check_autostart = CheckBox::create(
            parent,
            IDC_CHECK_AUTOSTART,
            "Khởi động cùng hệ điều hành Windows",
            40,
            178,
            360,
            24,
            hfont_normal,
        )?;
        let check_show_dialog = CheckBox::create(
            parent,
            IDC_CHECK_SHOW_DIALOG,
            "Bật hội thoại này khi khởi động",
            40,
            206,
            360,
            24,
            hfont_normal,
        )?;
        let check_debug_log = CheckBox::create(
            parent,
            IDC_CHECK_DEBUG_LOG,
            "Bật ghi nhật ký chẩn đoán (Debug Log)",
            40,
            236,
            250,
            24,
            hfont_normal,
        )?;
        let btn_open_log = PushButton::create(
            parent,
            IDC_BTN_OPEN_LOG,
            "Xem log...",
            300,
            233,
            100,
            26,
            hfont_normal,
        )?;
        let label_theme_title =
            Label::create(parent, "Giao diện (Theme)", 40, 275, 360, 22, hfont_bold)?;
        let label_theme =
            Label::create(parent, "Chế độ hiển thị:", 40, 307, 120, 20, hfont_normal)?;
        let combo_theme =
            ComboBox::create(parent, IDC_COMBO_THEME, 165, 303, 235, 150, hfont_normal)?;
        combo_theme.add_item("Theo hệ thống (Auto)");
        combo_theme.add_item("Sáng (Light)");
        combo_theme.add_item("Tối (Dark)");
        combo_theme.set_selected(0);

        // 6. Tab 3: Thông tin Controls
        let label_about_title = Label::create(
            parent,
            "MKey - Bộ gõ tiếng Việt",
            40,
            150,
            360,
            24,
            hfont_bold,
        )?;
        let label_about_ver =
            Label::create(parent, "Phiên bản: 0.1.0", 40, 185, 360, 20, hfont_normal)?;
        let label_about_author =
            Label::create(parent, "Tác giả: Mạnh Kiên", 40, 215, 360, 20, hfont_normal)?;
        let label_about_email = Label::create_with_id(
            parent,
            IDC_LABEL_EMAIL,
            "Email: manhkien13041997@gmail.com",
            40,
            245,
            360,
            20,
            hfont_normal,
        )?;
        let label_about_github = Label::create_with_id(
            parent,
            IDC_LABEL_GITHUB,
            "GitHub: https://github.com/uongsuadaubung/mkey",
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
            "Kết thúc",
            20,
            456,
            110,
            32,
            hfont_normal,
        )?;
        let btn_defaults = PushButton::create(
            parent,
            IDC_BTN_DEFAULTS,
            "Mặc định",
            175,
            456,
            110,
            32,
            hfont_normal,
        )?;
        let btn_close = PushButton::create(
            parent,
            IDC_BTN_CLOSE,
            "Đóng",
            330,
            456,
            110,
            32,
            hfont_normal,
        )?;

        Some(Self {
            label_method,
            label_mode,
            label_switch,
            combo_method,
            combo_mode,
            check_ctrl_shift,
            tab_bar,
            label_typing_title,
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
            label_sys_title,
            check_autostart,
            check_show_dialog,
            check_debug_log,
            btn_open_log,
            label_theme_title,
            label_theme,
            combo_theme,
            label_about_title,
            label_about_ver,
            label_about_author,
            label_about_email,
            label_about_github,
            btn_exit,
            btn_defaults,
            btn_close,
        })
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
            ShowWindow(self.check_spelling.hwnd(), show_t0);
            ShowWindow(self.check_restore_wrong.hwnd(), show_t0);
            ShowWindow(self.check_auto_upper.hwnd(), show_t0);

            // Tab 1 controls
            let show_t1 = if tab_idx == 1 { SW_SHOW } else { SW_HIDE };
            ShowWindow(self.check_use_macro.hwnd(), show_t1);
            ShowWindow(self.edit_macro_key.hwnd(), show_t1);
            ShowWindow(self.edit_macro_value.hwnd(), show_t1);
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

        self.check_spelling.set_checked(config.check_spelling);
        self.check_restore_wrong
            .set_checked(config.restore_on_wrong_spelling);
        self.check_auto_upper
            .set_checked(config.auto_uppercase_first_char);
        self.check_use_macro.set_checked(config.use_macro);

        // Tab 2: Luôn đọc trực tiếp từ Registry làm nguồn chuẩn xác nhất
        self.check_autostart
            .set_checked(crate::platform::is_windows_autostart_enabled());
        self.check_show_dialog
            .set_checked(config.show_dialog_on_startup);
        self.check_debug_log.set_checked(config.debug);

        self.combo_theme.set_selected(config.theme.into());
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
        config.check_spelling = self.check_spelling.is_checked();
        config.restore_on_wrong_spelling = self.check_restore_wrong.is_checked();
        config.auto_uppercase_first_char = self.check_auto_upper.is_checked();
        config.use_macro = self.check_use_macro.is_checked();

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
    }

    /// Populates or updates the Macro ListView with given pairs
    pub fn populate_macros(&self, macros: &[(String, String)]) {
        self.list_macro.clear();
        for (i, (k, v)) in macros.iter().enumerate() {
            self.list_macro.add_item(i as i32, k, v);
        }
    }
}
