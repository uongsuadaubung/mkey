use mkey::language::{Language, current, current_language, get_strings, set_current_language};

#[test]
fn test_default_language_is_vietnamese() {
    assert_eq!(Language::default(), Language::Vietnamese);
    set_current_language(Language::Vietnamese);
    assert_eq!(current_language(), Language::Vietnamese);
    assert_eq!(current().window_title, "MKey - Bảng điều khiển");
}

#[test]
fn test_switch_language_to_english_and_back() {
    set_current_language(Language::English);
    assert_eq!(current_language(), Language::English);
    assert_eq!(current().window_title, "MKey - Control Panel");
    assert_eq!(
        current().tab_titles(),
        ["Typing", "Macro", "System", "About"]
    );
    assert_eq!(current().btn_exit, "Exit");
    assert_eq!(current().btn_defaults, "Defaults");
    assert_eq!(current().btn_close, "Close");

    // Switch back to Vietnamese
    set_current_language(Language::Vietnamese);
    assert_eq!(current_language(), Language::Vietnamese);
    assert_eq!(current().window_title, "MKey - Bảng điều khiển");
    assert_eq!(
        current().tab_titles(),
        ["Bộ gõ", "Gõ tắt", "Hệ thống", "Thông tin"]
    );
    assert_eq!(current().btn_exit, "Kết thúc");
    assert_eq!(current().btn_defaults, "Mặc định");
    assert_eq!(current().btn_close, "Đóng");
}

#[test]
fn test_language_string_dictionaries_completeness() {
    for lang in [Language::Vietnamese, Language::English] {
        let s = get_strings(lang);
        assert!(!s.window_title.is_empty());
        assert!(!s.helper_window_title.is_empty());
        assert!(!s.label_method.is_empty());
        assert!(!s.method_telex.is_empty());
        assert!(!s.method_vni.is_empty());
        assert!(!s.method_simple_telex.is_empty());
        assert!(!s.label_mode.is_empty());
        assert!(!s.mode_vietnamese.is_empty());
        assert!(!s.mode_english.is_empty());
        assert!(!s.label_switch.is_empty());
        assert!(!s.check_ctrl_shift.is_empty());

        assert_eq!(s.tab_titles().len(), 4);
        assert!(!s.tab_typing.is_empty());
        assert!(!s.tab_macro.is_empty());
        assert!(!s.tab_system.is_empty());
        assert!(!s.tab_about.is_empty());

        assert!(!s.typing_title.is_empty());
        assert!(!s.check_restore_wrong.is_empty());
        assert!(!s.check_auto_upper.is_empty());
        assert!(!s.sound_title.is_empty());
        assert!(!s.check_sound_enabled.is_empty());
        assert!(!s.label_switch_type.is_empty());
        assert!(!s.label_volume.is_empty());
        assert!(!s.btn_test_sound.is_empty());

        assert!(!s.check_use_macro.is_empty());
        assert!(!s.edit_macro_key_placeholder.is_empty());
        assert!(!s.edit_macro_val_placeholder.is_empty());
        assert!(!s.btn_add_macro.is_empty());
        assert!(!s.btn_edit_macro.is_empty());
        assert!(!s.btn_cancel_macro.is_empty());
        assert!(!s.btn_del_macro.is_empty());
        assert!(!s.col_macro_key.is_empty());
        assert!(!s.col_macro_val.is_empty());

        assert!(!s.sys_title.is_empty());
        assert!(!s.check_autostart.is_empty());
        assert!(!s.check_show_dialog.is_empty());
        assert!(!s.check_debug_log.is_empty());
        assert!(!s.btn_open_log.is_empty());
        assert!(!s.btn_check_update.is_empty());
        assert!(!s.btn_checking_update.is_empty());
        assert!(!s.label_theme_title.is_empty());
        assert!(!s.label_theme.is_empty());
        assert!(!s.theme_auto.is_empty());
        assert!(!s.theme_light.is_empty());
        assert!(!s.theme_dark.is_empty());
        assert!(!s.label_language.is_empty());

        assert!(!s.about_title.is_empty());
        assert!(!s.about_ver.is_empty());
        assert!(!s.about_author.is_empty());
        assert!(!s.about_email.is_empty());
        assert!(!s.about_github.is_empty());

        assert!(!s.btn_exit.is_empty());
        assert!(!s.btn_defaults.is_empty());
        assert!(!s.btn_close.is_empty());

        assert!(!s.tray_tooltip_vi.is_empty());
        assert!(!s.tray_tooltip_en.is_empty());
        assert!(!s.tray_toggle_vi.is_empty());
        assert!(!s.tray_method_telex.is_empty());
        assert!(!s.tray_method_vni.is_empty());
        assert!(!s.tray_method_simple_telex.is_empty());
        assert!(!s.tray_control_panel.is_empty());
        assert!(!s.tray_exit.is_empty());
    }
}

#[test]
fn test_language_from_str_and_display() {
    assert_eq!("vi".parse::<Language>().unwrap(), Language::Vietnamese);
    assert_eq!("en".parse::<Language>().unwrap(), Language::English);
    assert_eq!("english".parse::<Language>().unwrap(), Language::English);
    assert_eq!("unknown".parse::<Language>().unwrap(), Language::Vietnamese);

    assert_eq!(Language::Vietnamese.to_string(), "vi");
    assert_eq!(Language::English.to_string(), "en");
}

#[test]
fn test_control_panel_language_switch() {
    use mkey::ui::components::window::init_common_controls;
    use mkey::ui::views::ControlPanelControls;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    init_common_controls();

    fn to_wide(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(Some(0)).collect()
    }

    #[link(name = "user32")]
    unsafe extern "system" {
        fn CreateWindowExW(
            dw_ex_style: u32,
            lp_class_name: *const u16,
            lp_window_name: *const u16,
            dw_style: u32,
            x: i32,
            y: i32,
            n_width: i32,
            n_height: i32,
            h_wnd_parent: isize,
            h_menu: isize,
            h_instance: isize,
            lp_param: *mut std::ffi::c_void,
        ) -> isize;
        fn DestroyWindow(h_wnd: isize) -> i32;
        fn GetWindowTextW(h_wnd: isize, lp_string: *mut u16, n_max_count: i32) -> i32;
    }

    let class_name = to_wide("STATIC");
    let parent = unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            std::ptr::null(),
            0,
            0,
            0,
            500,
            500,
            0,
            0,
            0,
            std::ptr::null_mut(),
        )
    };
    assert_ne!(parent, 0);

    let controls = ControlPanelControls::create(parent, 0, 0).unwrap();

    // Default: Vietnamese
    let mut buf = vec![0u16; 64];
    let len = unsafe { GetWindowTextW(controls.btn_exit.hwnd(), buf.as_mut_ptr(), 64) };
    assert_eq!(String::from_utf16_lossy(&buf[..len as usize]), "Kết thúc");

    // Switch to English
    controls.update_language(Language::English);
    let len = unsafe { GetWindowTextW(controls.btn_exit.hwnd(), buf.as_mut_ptr(), 64) };
    assert_eq!(String::from_utf16_lossy(&buf[..len as usize]), "Exit");

    let len = unsafe { GetWindowTextW(controls.btn_close.hwnd(), buf.as_mut_ptr(), 64) };
    assert_eq!(String::from_utf16_lossy(&buf[..len as usize]), "Close");

    let len =
        unsafe { GetWindowTextW(controls.check_macro_in_english.hwnd(), buf.as_mut_ptr(), 64) };
    assert_eq!(
        String::from_utf16_lossy(&buf[..len as usize]),
        "Allow shorthand in English mode"
    );

    assert_eq!(controls.combo_macro_type.get_item_text(0), "Whole word");
    assert_eq!(
        controls.combo_macro_type.get_item_text(1),
        "Start consonant"
    );
    assert_eq!(controls.combo_macro_type.get_item_text(2), "End consonant");

    assert_eq!(controls.combo_mode.get_item_text(0), "Vietnamese");
    assert_eq!(controls.combo_mode.get_item_text(1), "English");
    assert_eq!(controls.combo_theme.get_item_text(0), "System (Auto)");
    assert_eq!(controls.combo_theme.get_item_text(1), "Light");
    assert_eq!(controls.combo_theme.get_item_text(2), "Dark");
    assert_eq!(controls.combo_lang.get_selected(), Some(1));

    // Switch back to Vietnamese
    controls.update_language(Language::Vietnamese);
    let len = unsafe { GetWindowTextW(controls.btn_exit.hwnd(), buf.as_mut_ptr(), 64) };
    assert_eq!(String::from_utf16_lossy(&buf[..len as usize]), "Kết thúc");

    let len = unsafe { GetWindowTextW(controls.btn_close.hwnd(), buf.as_mut_ptr(), 64) };
    assert_eq!(String::from_utf16_lossy(&buf[..len as usize]), "Đóng");

    let len =
        unsafe { GetWindowTextW(controls.check_macro_in_english.hwnd(), buf.as_mut_ptr(), 64) };
    assert_eq!(
        String::from_utf16_lossy(&buf[..len as usize]),
        "Cho phép gõ tắt cả khi ở chế độ tiếng Anh"
    );

    assert_eq!(controls.combo_macro_type.get_item_text(0), "Toàn từ");
    assert_eq!(controls.combo_macro_type.get_item_text(1), "Phụ âm đầu");
    assert_eq!(controls.combo_macro_type.get_item_text(2), "Phụ âm cuối");

    assert_eq!(controls.combo_mode.get_item_text(0), "Tiếng Việt");
    assert_eq!(controls.combo_mode.get_item_text(1), "Tiếng Anh");
    assert_eq!(
        controls.combo_theme.get_item_text(0),
        "Theo hệ thống (Auto)"
    );
    assert_eq!(controls.combo_theme.get_item_text(1), "Sáng (Light)");
    assert_eq!(controls.combo_theme.get_item_text(2), "Tối (Dark)");
    assert_eq!(controls.combo_lang.get_selected(), Some(0));

    // Dynamic enable/disable of child macro controls based on check_use_macro
    controls.check_use_macro.set_checked(false);
    controls.update_macro_checkboxes_state();
    assert!(!controls.check_macro_in_english.is_enabled());

    controls.check_use_macro.set_checked(true);
    controls.update_macro_checkboxes_state();
    assert!(controls.check_macro_in_english.is_enabled());

    // Dynamic enable/disable of sound controls based on check_sound_enabled
    controls.check_sound_enabled.set_checked(false);
    controls.update_sound_controls_state();
    assert!(!controls.combo_switch_type.is_enabled());
    assert!(!controls.slider_volume.is_enabled());
    assert!(!controls.btn_test_sound.is_enabled());

    controls.check_sound_enabled.set_checked(true);
    controls.update_sound_controls_state();
    if controls.sound_available {
        assert!(controls.combo_switch_type.is_enabled());
        assert!(controls.slider_volume.is_enabled());
        assert!(controls.btn_test_sound.is_enabled());
    } else {
        assert!(!controls.combo_switch_type.is_enabled());
        assert!(!controls.slider_volume.is_enabled());
        assert!(!controls.btn_test_sound.is_enabled());
    }

    unsafe {
        DestroyWindow(parent);
    }
}
