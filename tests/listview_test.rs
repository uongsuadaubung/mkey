#[test]
fn test_listview_initialization() {
    use mkey::ui::components::listview::ListView;
    use mkey::ui::components::window::init_common_controls;

    init_common_controls();
    // Test creating a listview with parent = 0 (or message-only / desktop window)
    // Actually, create a simple hidden parent window
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

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
        fn SendMessageW(h_wnd: isize, msg: u32, w_param: usize, l_param: isize) -> isize;
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
            400,
            300,
            0,
            0,
            0,
            std::ptr::null_mut(),
        )
    };
    assert_ne!(parent, 0, "Parent window creation failed");

    let lv = ListView::create(parent, 1001, 10, 10, 300, 200, 0);
    assert!(lv.is_some(), "ListView creation failed!");
    let lv = lv.unwrap();

    lv.add_column(0, "Từ viết tắt", 100);
    lv.add_column(1, "Cụm từ thay thế", 150);

    lv.add_item(0, "ko", "không");
    lv.add_item(1, "dc", "được");

    const LVM_FIRST: u32 = 0x1000;
    const LVM_GETITEMCOUNT: u32 = LVM_FIRST + 4;
    let count = unsafe { SendMessageW(lv.hwnd(), LVM_GETITEMCOUNT, 0, 0) };
    println!("ListView item count: {}", count);
    assert_eq!(count, 2, "Expected 2 items in ListView");

    // Check text of subitem 0 and 1
    const LVM_GETITEMTEXTW: u32 = LVM_FIRST + 115;
    let mut buf = vec![0u16; 64];
    #[repr(C)]
    struct LVITEMW_MINI {
        mask: u32,
        i_item: i32,
        i_sub_item: i32,
        state: u32,
        state_mask: u32,
        psz_text: *mut u16,
        cch_text_max: i32,
    }
    let mut item_check = LVITEMW_MINI {
        mask: 0,
        i_item: 0,
        i_sub_item: 1,
        state: 0,
        state_mask: 0,
        psz_text: buf.as_mut_ptr(),
        cch_text_max: 64,
    };
    let len = unsafe {
        SendMessageW(
            lv.hwnd(),
            LVM_GETITEMTEXTW,
            0,
            &mut item_check as *mut _ as isize,
        )
    };
    let text = String::from_utf16_lossy(&buf[..len as usize]);
    println!("Retrieved item 0 subitem 1 text: '{}'", text);
    assert_eq!(text, "không");

    unsafe {
        DestroyWindow(parent);
    }
}

#[test]
fn test_tab_and_listview_coexistence() {
    use mkey::ui::components::listview::ListView;
    use mkey::ui::components::tab_control::TabControl;
    use mkey::ui::components::window::init_common_controls;

    init_common_controls();

    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

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
        fn IsWindowVisible(h_wnd: isize) -> i32;
        fn ShowWindow(h_wnd: isize, n_cmd_show: i32) -> i32;
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

    let tab = TabControl::create(parent, 400, 15, 80, 400, 310, 0);
    assert!(tab.is_some(), "TabControl creation failed");
    let tab = tab.unwrap();
    tab.insert_tab(0, "Tab 1");
    tab.insert_tab(1, "Tab 2");

    let lv = ListView::create(parent, 410, 30, 145, 360, 195, 0);
    assert!(lv.is_some(), "ListView creation failed");
    let lv = lv.unwrap();

    unsafe {
        ShowWindow(parent, 5);
        ShowWindow(tab.hwnd(), 5);
        ShowWindow(lv.hwnd(), 5);

        assert_ne!(IsWindowVisible(lv.hwnd()), 0, "ListView must be visible!");
        DestroyWindow(parent);
    }
}

#[test]
fn test_button_text_and_macro_contains() {
    use mkey::engine::macro_table::MacroTable;
    use mkey::ui::components::button::PushButton;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

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

    // 1. Verify MacroTable contains_key
    let mut table = MacroTable::new();
    table.insert("ko", "không");
    assert!(table.contains_key("ko"));
    assert!(table.contains_key("KO"));
    assert!(!table.contains_key("dc"));

    // 2. Verify PushButton set_text
    let class_name = to_wide("STATIC");
    let parent = unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            std::ptr::null(),
            0,
            0,
            0,
            200,
            200,
            0,
            0,
            0,
            std::ptr::null_mut(),
        )
    };
    assert_ne!(parent, 0);

    let btn = PushButton::create(parent, 501, "+ Thêm", 10, 10, 80, 26, 0).unwrap();
    let mut buf = vec![0u16; 64];
    let len = unsafe { GetWindowTextW(btn.hwnd(), buf.as_mut_ptr(), 64) };
    let text = String::from_utf16_lossy(&buf[..len as usize]);
    assert_eq!(text, "+ Thêm");

    // Dynamic switch to "Sửa"
    btn.set_text("Sửa");
    let len = unsafe { GetWindowTextW(btn.hwnd(), buf.as_mut_ptr(), 64) };
    let text = String::from_utf16_lossy(&buf[..len as usize]);
    assert_eq!(text, "Sửa");

    // Revert back to "+ Thêm"
    btn.set_text("+ Thêm");
    let len = unsafe { GetWindowTextW(btn.hwnd(), buf.as_mut_ptr(), 64) };
    let text = String::from_utf16_lossy(&buf[..len as usize]);
    assert_eq!(text, "+ Thêm");

    // 3. Verify ListView clear_selection
    let lv =
        mkey::ui::components::listview::ListView::create(parent, 502, 10, 50, 150, 100, 0).unwrap();
    lv.add_column(0, "Col", 80);
    lv.add_item(0, "A", "B");
    lv.clear_selection();
    assert_eq!(lv.get_selected_index(), None);

    unsafe {
        DestroyWindow(parent);
    }
}

#[test]
fn test_macro_dynamic_buttons_visibility() {
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
        fn IsWindowVisible(h_wnd: isize) -> i32;
        fn ShowWindow(h_wnd: isize, n_cmd_show: i32) -> i32;
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
    unsafe {
        ShowWindow(parent, 5);
    }
    controls.update_tab_visibility(1);

    // Initial state: not editing -> + Thêm is visible, Sửa/Hủy/Xóa are hidden
    unsafe {
        assert_ne!(
            IsWindowVisible(controls.btn_add_macro.hwnd()),
            0,
            "btn_add_macro should be visible"
        );
        assert_eq!(
            IsWindowVisible(controls.btn_edit_macro.hwnd()),
            0,
            "btn_edit_macro should be hidden"
        );
        assert_eq!(
            IsWindowVisible(controls.btn_cancel_macro.hwnd()),
            0,
            "btn_cancel_macro should be hidden"
        );
        assert_eq!(
            IsWindowVisible(controls.btn_del_macro.hwnd()),
            0,
            "btn_del_macro should be hidden"
        );
    }

    // Switch to edit mode -> + Thêm is hidden, Sửa/Hủy/Xóa are visible
    controls.set_macro_edit_mode(true);
    unsafe {
        assert_eq!(
            IsWindowVisible(controls.btn_add_macro.hwnd()),
            0,
            "btn_add_macro should be hidden in edit mode"
        );
        assert_ne!(
            IsWindowVisible(controls.btn_edit_macro.hwnd()),
            0,
            "btn_edit_macro should be visible in edit mode"
        );
        assert_ne!(
            IsWindowVisible(controls.btn_cancel_macro.hwnd()),
            0,
            "btn_cancel_macro should be visible in edit mode"
        );
        assert_ne!(
            IsWindowVisible(controls.btn_del_macro.hwnd()),
            0,
            "btn_del_macro should be visible in edit mode"
        );
    }

    // Switch back to normal mode -> + Thêm is visible, Sửa/Hủy/Xóa are hidden
    controls.set_macro_edit_mode(false);
    unsafe {
        assert_ne!(
            IsWindowVisible(controls.btn_add_macro.hwnd()),
            0,
            "btn_add_macro should be restored"
        );
        assert_eq!(
            IsWindowVisible(controls.btn_edit_macro.hwnd()),
            0,
            "btn_edit_macro should be hidden again"
        );
        assert_eq!(
            IsWindowVisible(controls.btn_cancel_macro.hwnd()),
            0,
            "btn_cancel_macro should be hidden again"
        );
        assert_eq!(
            IsWindowVisible(controls.btn_del_macro.hwnd()),
            0,
            "btn_del_macro should be hidden again"
        );
        DestroyWindow(parent);
    }
}
