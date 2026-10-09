//! Pure Win32 API Low-Level Keyboard Hook & Key Injection in Rust
//! No external crates required — uses direct Win32 FFI bindings.

pub mod app_detect;
pub mod injector;
pub mod types;

pub use app_detect::*;
pub use injector::*;
pub use types::*;

use crate::engine::action::EngineAction;
use crate::engine::VietnameseEngine;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

static CTRL_SHIFT_ARMED: AtomicBool = AtomicBool::new(false);

fn is_ctrl_vk(vk: u32) -> bool {
    matches!(vk, 0x11 | 0xA2 | 0xA3) // VK_CONTROL, VK_LCONTROL, VK_RCONTROL
}

fn is_shift_vk(vk: u32) -> bool {
    matches!(vk, 0x10 | 0xA0 | 0xA1) // VK_SHIFT, VK_LSHIFT, VK_RSHIFT
}

// Global thread-safe Engine instance accessed by the hook callback and UI
pub static ENGINE_INSTANCE: Mutex<Option<VietnameseEngine>> = Mutex::new(None);
static HOOK_HANDLE: Mutex<isize> = Mutex::new(0);
static MOUSE_HOOK_HANDLE: Mutex<isize> = Mutex::new(0);

/// Mouse hook callback: resets the engine word session whenever user clicks somewhere
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn low_level_mouse_proc(
    n_code: i32,
    w_param: usize,
    l_param: isize,
) -> isize {
    unsafe {
        if n_code >= 0
            && (w_param == WM_LBUTTONDOWN
                || w_param == WM_RBUTTONDOWN
                || w_param == WM_MBUTTONDOWN)
            && let Ok(mut guard) = ENGINE_INSTANCE.lock()
            && let Some(ref mut engine) = *guard
        {
            if engine.config().debug && !engine.buffer.is_empty() {
                eprintln!(
                    "[DBG][WIN32_MOUSE] Mouse button {:#X} clicked -> Reset engine buffer",
                    w_param
                );
            }
            engine.reset();
        }
        CallNextHookEx(0, n_code, w_param, l_param)
    }
}

/// The Low-Level Keyboard Hook procedure
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn low_level_keyboard_proc(
    n_code: i32,
    w_param: usize,
    l_param: isize,
) -> isize {
    unsafe {
        if n_code >= 0 {
            let hook_struct = *(l_param as *const KBDLLHOOKSTRUCT);

            // 1. Ignore our own synthetic events to avoid infinite recursion
            if hook_struct.dw_extra_info == MAGIC_EXTRA_INFO {
                return CallNextHookEx(0, n_code, w_param, l_param);
            }

            let vk = hook_struct.vk_code;

            // Handle key-up events for hotkey triggers (e.g. Ctrl + Shift toggle)
            if w_param == WM_KEYUP || w_param == WM_SYSKEYUP {
                if (is_ctrl_vk(vk) || is_shift_vk(vk))
                    && CTRL_SHIFT_ARMED.swap(false, Ordering::SeqCst)
                    && let Ok(mut guard) = ENGINE_INSTANCE.lock()
                    && let Some(ref mut engine) = *guard
                {
                    let new_state = engine.toggle_enabled();
                    let cfg = engine.config().clone();
                    let macros = engine.macro_table.clone();
                    std::thread::spawn(move || {
                        if let Err(e) =
                            crate::engine::config_store::save_config_and_macros(&cfg, &macros)
                        {
                            eprintln!("[MKey] Lỗi lưu cấu hình: {e}");
                        }
                    });
                    if new_state {
                        println!("\n[MKey] >> Chế độ gõ: [V] TIẾNG VIỆT");
                    } else {
                        println!("\n[MKey] >> Chế độ gõ: [E] TIẾNG ANH");
                    }
                    crate::ui::update_tray_icon(new_state);
                }
                return CallNextHookEx(0, n_code, w_param, l_param);
            }

            // Only process key-down events
            if w_param == WM_KEYDOWN || w_param == WM_SYSKEYDOWN {
                // 2. Query hardware modifier states cleanly and reliably
                let is_caps = (GetKeyState(VK_CAPITAL) & 1) != 0;
                let is_shift = (GetAsyncKeyState(VK_SHIFT) as u16 & 0x8000) != 0;
                let is_ctrl = (GetAsyncKeyState(VK_CONTROL) as u16 & 0x8000) != 0;
                let is_alt = (GetAsyncKeyState(VK_MENU) as u16 & 0x8000) != 0;
                let is_win = (GetAsyncKeyState(VK_LWIN) as u16 & 0x8000) != 0
                    || (GetAsyncKeyState(VK_RWIN) as u16 & 0x8000) != 0;

                // Arm or disarm Ctrl + Shift hotkey toggle:
                // If both Ctrl and Shift are held down and neither Alt nor Win is active, arm.
                // If any non-modifier key is pressed (e.g. 'P' in Ctrl+Shift+P), disarm.
                if (is_ctrl_vk(vk) || is_shift_vk(vk)) && !is_alt && !is_win {
                    if is_ctrl && is_shift {
                        CTRL_SHIFT_ARMED.store(true, Ordering::SeqCst);
                    }
                    return CallNextHookEx(0, n_code, w_param, l_param);
                } else if !is_ctrl_vk(vk) && !is_shift_vk(vk) {
                    CTRL_SHIFT_ARMED.store(false, Ordering::SeqCst);
                }

                // Alt + Z hotkey toggle
                if vk == 'Z' as u32 && is_alt && !is_ctrl && !is_win {
                    if let Ok(mut guard) = ENGINE_INSTANCE.lock()
                        && let Some(ref mut engine) = *guard
                        && !engine.config().switch_with_ctrl_shift
                    {
                        let new_state = engine.toggle_enabled();
                        let cfg = engine.config().clone();
                        let macros = engine.macro_table.clone();
                        std::thread::spawn(move || {
                            if let Err(e) =
                                crate::engine::config_store::save_config_and_macros(&cfg, &macros)
                            {
                                eprintln!("[MKey] Lỗi lưu cấu hình: {e}");
                            }
                        });
                        if new_state {
                            println!("\n[MKey] >> Chế độ gõ: [V] TIẾNG VIỆT");
                        } else {
                            println!("\n[MKey] >> Chế độ gõ: [E] TIẾNG ANH");
                        }
                        crate::ui::update_tray_icon(new_state);
                        return 1;
                    }
                    return CallNextHookEx(0, n_code, w_param, l_param);
                }

                // If Ctrl, Alt, or Win are held down, let OS handle and reset buffer
                if is_ctrl || is_alt || is_win {
                    if let Ok(mut guard) = ENGINE_INSTANCE.lock()
                        && let Some(ref mut engine) = *guard
                    {
                        if engine.config().debug && !engine.buffer.is_empty() {
                            eprintln!(
                                "[DBG][WIN32_MOD] Modifier active (Ctrl: {is_ctrl}, Alt: {is_alt}, Win: {is_win}) -> Reset engine buffer"
                            );
                        }
                        engine.reset();
                    }
                    return CallNextHookEx(0, n_code, w_param, l_param);
                }

                // Navigation / Termination keys reset the current word session
                let is_nav = matches!(
                    vk,
                    VK_RETURN
                        | VK_TAB
                        | VK_ESCAPE
                        | VK_LEFT
                        | VK_RIGHT
                        | VK_UP
                        | VK_DOWN
                        | VK_HOME
                        | VK_END
                        | VK_PRIOR
                        | VK_NEXT
                        | VK_DELETE
                        | VK_INSERT
                );
                if is_nav {
                    if let Ok(mut guard) = ENGINE_INSTANCE.lock()
                        && let Some(ref mut engine) = *guard
                    {
                        if engine.config().debug && !engine.buffer.is_empty() {
                            eprintln!(
                                "[DBG][WIN32_NAV] VK 0x{:02X} pressed -> Reset engine buffer",
                                vk
                            );
                        }
                        engine.reset();
                    }
                    return CallNextHookEx(0, n_code, w_param, l_param);
                }

                // Handle Backspace
                if vk == VK_BACK {
                    if let Ok(mut guard) = ENGINE_INSTANCE.lock()
                        && let Some(ref mut engine) = *guard
                    {
                        engine.on_backspace();
                    }
                    return CallNextHookEx(0, n_code, w_param, l_param);
                }

                // 3. Convert VK code to unicode character
                let mut key_state = [0u8; 256];
                GetKeyboardState(key_state.as_mut_ptr());

                // Synchronize Shift / CapsLock in the keyboard state array for ToUnicode
                if is_shift {
                    key_state[VK_SHIFT as usize] = 0x80;
                } else {
                    key_state[VK_SHIFT as usize] = 0;
                }
                if is_caps {
                    key_state[VK_CAPITAL as usize] = 0x01;
                } else {
                    key_state[VK_CAPITAL as usize] = 0;
                }

                let mut buff = [0u16; 4];
                let count = ToUnicode(
                    vk,
                    hook_struct.scan_code,
                    key_state.as_ptr(),
                    buff.as_mut_ptr(),
                    4,
                    0,
                );

                if count > 0
                    && let Some(ch) = char::decode_utf16(buff[0..count as usize].iter().cloned())
                        .next()
                        .and_then(|r| r.ok())
                    && let Ok(mut guard) = ENGINE_INSTANCE.lock()
                    && let Some(ref mut engine) = *guard
                {
                    let action = engine.on_key(ch, is_shift, is_caps);
                    match action {
                        EngineAction::Passthrough => {
                            // Let OS handle original keystroke
                            return CallNextHookEx(0, n_code, w_param, l_param);
                        }
                        EngineAction::Replace { backspaces, output } => {
                            // Consume this key, send backspaces and replacement string in a single atomic SendInput call
                            send_replace(backspaces, &output);
                            return 1; // Intercept: do not pass to target window
                        }
                        EngineAction::Consume => {
                            return 1; // Drop key entirely
                        }
                    }
                }
            }
        }

        CallNextHookEx(0, n_code, w_param, l_param)
    }
}

/// Installs the Windows keyboard & mouse hooks and starts the Win32 message pump
pub fn run_hook_loop(engine: VietnameseEngine, is_autostart: bool) {
    let show_dialog_on_startup = engine.config().show_dialog_on_startup;
    {
        let mut guard = ENGINE_INSTANCE.lock().unwrap();
        *guard = Some(engine);
    }

    unsafe {
        let kbd_hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            low_level_keyboard_proc,
            GetModuleHandleW(null_mut()),
            0,
        );

        if kbd_hook == 0 {
            eprintln!("[Error] Không thể cài đặt Windows Keyboard Hook!");
            return;
        }

        let mouse_hook = SetWindowsHookExW(
            WH_MOUSE_LL,
            low_level_mouse_proc,
            GetModuleHandleW(null_mut()),
            0,
        );

        {
            let mut h_guard = HOOK_HANDLE.lock().unwrap();
            *h_guard = kbd_hook;
        }
        if mouse_hook != 0 {
            let mut m_guard = MOUSE_HOOK_HANDLE.lock().unwrap();
            *m_guard = mouse_hook;
        }

        println!(">> Windows Hook đã kích hoạt! Hãy mở Notepad/Browser và gõ tiếng Việt để test.");
        println!(">> Nhấn Ctrl + C trong cửa sổ này để tắt.");

        // Initialize Native Win32 UI (System Tray & Control Panel)
        crate::ui::init_ui();
        if !is_autostart || show_dialog_on_startup {
            crate::ui::show_control_panel();
        } else {
            trim_working_set();
        }

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, 0, 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        // Cleanup
        let h = *HOOK_HANDLE.lock().unwrap();
        if h != 0 {
            UnhookWindowsHookEx(h);
        }
        let mh = *MOUSE_HOOK_HANDLE.lock().unwrap();
        if mh != 0 {
            UnhookWindowsHookEx(mh);
        }
    }
}

/// Trims the process working set, returning unused RAM and cached pages back to Windows.
/// Reduces resident physical RAM footprint down to ~1-2 MB.
pub fn trim_working_set() {
    unsafe {
        SetProcessWorkingSetSize(GetCurrentProcess(), usize::MAX, usize::MAX);
    }
}

