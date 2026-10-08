//! Pure Win32 API Low-Level Keyboard Hook & Key Injection in Rust
//! No external crates required — uses direct Win32 FFI bindings.

use crate::engine::{action::EngineAction, VietnameseEngine};
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

static CTRL_SHIFT_ARMED: AtomicBool = AtomicBool::new(false);

#[inline]
fn is_ctrl_vk(vk: u32) -> bool {
    matches!(vk, 0x11 | 0xA2 | 0xA3) // VK_CONTROL, VK_LCONTROL, VK_RCONTROL
}

#[inline]
fn is_shift_vk(vk: u32) -> bool {
    matches!(vk, 0x10 | 0xA0 | 0xA1) // VK_SHIFT, VK_LSHIFT, VK_RSHIFT
}

// Win32 Constants
pub const WH_KEYBOARD_LL: i32 = 13;
pub const WM_KEYDOWN: usize = 0x0100;
pub const WM_KEYUP: usize = 0x0101;
pub const WM_SYSKEYDOWN: usize = 0x0104;
pub const WM_SYSKEYUP: usize = 0x0105;

pub const INPUT_KEYBOARD: u32 = 1;
pub const KEYEVENTF_KEYUP: u32 = 0x0002;
pub const KEYEVENTF_UNICODE: u32 = 0x0004;

pub const VK_BACK: u16 = 0x08;
pub const VK_TAB: u16 = 0x09;
pub const VK_RETURN: u16 = 0x0D;
pub const VK_SHIFT: i32 = 0x10;
pub const VK_CONTROL: i32 = 0x11;
pub const VK_MENU: i32 = 0x12; // Alt
pub const VK_CAPITAL: i32 = 0x14; // Caps Lock
pub const VK_ESCAPE: i32 = 0x1B;
pub const VK_SPACE: u16 = 0x20;
pub const VK_PRIOR: u32 = 0x21; // Page Up
pub const VK_NEXT: u32 = 0x22;  // Page Down
pub const VK_END: u32 = 0x23;
pub const VK_HOME: u32 = 0x24;
pub const VK_LEFT: u32 = 0x25;
pub const VK_UP: u32 = 0x26;
pub const VK_RIGHT: u32 = 0x27;
pub const VK_DOWN: u32 = 0x28;
pub const VK_INSERT: u32 = 0x2D;
pub const VK_DELETE: u32 = 0x2E;
pub const VK_LWIN: i32 = 0x5B;
pub const VK_RWIN: i32 = 0x5C;

pub const WH_MOUSE_LL: i32 = 14;
pub const WM_LBUTTONDOWN: usize = 0x0201;
pub const WM_RBUTTONDOWN: usize = 0x0204;
pub const WM_MBUTTONDOWN: usize = 0x0207;

/// Magic identifier to tag our own synthetic input events and prevent infinite hook loops
pub const MAGIC_EXTRA_INFO: usize = 0x4D4B4559; // "MKEY" in ASCII

// Win32 FFI Structures
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KBDLLHOOKSTRUCT {
    pub vk_code: u32,
    pub scan_code: u32,
    pub flags: u32,
    pub time: u32,
    pub dw_extra_info: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KEYBDINPUT {
    pub w_vk: u16,
    pub w_scan: u16,
    pub dw_flags: u32,
    pub time: u32,
    pub dw_extra_info: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MOUSEINPUT {
    pub dx: i32,
    pub dy: i32,
    pub mouse_data: u32,
    pub dw_flags: u32,
    pub time: u32,
    pub dw_extra_info: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HARDWAREINPUT {
    pub u_msg: u32,
    pub w_param_l: u16,
    pub w_param_h: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union INPUT_UNION {
    pub mi: MOUSEINPUT,
    pub ki: KEYBDINPUT,
    pub hi: HARDWAREINPUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct INPUT {
    pub r#type: u32,
    pub u: INPUT_UNION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MSG {
    pub hwnd: isize,
    pub message: u32,
    pub w_param: usize,
    pub l_param: isize,
    pub time: u32,
    pub pt: POINT,
}

pub type HOOKPROC = unsafe extern "system" fn(code: i32, w_param: usize, l_param: isize) -> isize;

// Win32 API function signatures linked directly to user32.dll and kernel32.dll
#[link(name = "user32")]
unsafe extern "system" {
    pub fn SetWindowsHookExW(
        idHook: i32,
        lpfn: HOOKPROC,
        hmod: isize,
        dwThreadId: u32,
    ) -> isize;

    pub fn UnhookWindowsHookEx(hhk: isize) -> i32;

    pub fn CallNextHookEx(
        hhk: isize,
        nCode: i32,
        wParam: usize,
        lParam: isize,
    ) -> isize;

    pub fn GetMessageW(
        lpMsg: *mut MSG,
        hWnd: isize,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32,
    ) -> i32;

    pub fn TranslateMessage(lpMsg: *const MSG) -> i32;

    pub fn DispatchMessageW(lpMsg: *const MSG) -> isize;

    pub fn SendInput(
        cInputs: u32,
        pInputs: *const INPUT,
        cbSize: i32,
    ) -> u32;

    pub fn GetKeyState(nVirtKey: i32) -> i16;
    pub fn GetAsyncKeyState(vKey: i32) -> i16;

    pub fn GetKeyboardState(lpKeyState: *mut u8) -> i32;

    pub fn ToUnicode(
        wVirtKey: u32,
        wScanCode: u32,
        lpKeyState: *const u8,
        pwszBuff: *mut u16,
        cchBuff: i32,
        wFlags: u32,
    ) -> i32;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn GetModuleHandleW(lpModuleName: *const u16) -> isize;
}

// Global thread-safe Engine instance accessed by the hook callback and UI
pub static ENGINE_INSTANCE: Mutex<Option<VietnameseEngine>> = Mutex::new(None);
static HOOK_HANDLE: Mutex<isize> = Mutex::new(0);
static MOUSE_HOOK_HANDLE: Mutex<isize> = Mutex::new(0);

/// Sends backspaces and replacement string in a SINGLE atomic SendInput batch
pub fn send_replace(backspaces: usize, text: &str) {
    let utf16: Vec<u16> = text.encode_utf16().collect();
    let total_inputs = backspaces * 2 + utf16.len() * 2;
    if total_inputs == 0 {
        return;
    }

    let mut inputs = Vec::with_capacity(total_inputs);

    // 1. Send all backspaces
    for _ in 0..backspaces {
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            u: INPUT_UNION {
                ki: KEYBDINPUT {
                    w_vk: VK_BACK,
                    w_scan: 0x0E,
                    dw_flags: 0,
                    time: 0,
                    dw_extra_info: MAGIC_EXTRA_INFO,
                },
            },
        });
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            u: INPUT_UNION {
                ki: KEYBDINPUT {
                    w_vk: VK_BACK,
                    w_scan: 0x0E,
                    dw_flags: KEYEVENTF_KEYUP,
                    time: 0,
                    dw_extra_info: MAGIC_EXTRA_INFO,
                },
            },
        });
    }

    // 2. Send replacement unicode characters in the SAME batch
    for &code_unit in &utf16 {
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            u: INPUT_UNION {
                ki: KEYBDINPUT {
                    w_vk: 0,
                    w_scan: code_unit,
                    dw_flags: KEYEVENTF_UNICODE,
                    time: 0,
                    dw_extra_info: MAGIC_EXTRA_INFO,
                },
            },
        });
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            u: INPUT_UNION {
                ki: KEYBDINPUT {
                    w_vk: 0,
                    w_scan: code_unit,
                    dw_flags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dw_extra_info: MAGIC_EXTRA_INFO,
                },
            },
        });
    }

    unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        );
    }
}

/// Sends simulated Backspace keystrokes to active window
pub fn send_backspaces(count: usize) {
    send_replace(count, "");
}

/// Sends simulated UTF-16 Unicode characters to active window
pub fn send_unicode_string(text: &str) {
    send_replace(0, text);
}

/// Mouse hook callback: resets the engine word session whenever user clicks somewhere
pub unsafe extern "system" fn low_level_mouse_proc(
    n_code: i32,
    w_param: usize,
    l_param: isize,
) -> isize {
    unsafe {
        if n_code >= 0 && (w_param == WM_LBUTTONDOWN || w_param == WM_RBUTTONDOWN || w_param == WM_MBUTTONDOWN) {
            if let Ok(mut guard) = ENGINE_INSTANCE.lock() {
                if let Some(ref mut engine) = *guard {
                    if engine.config().debug && !engine.buffer.is_empty() {
                        eprintln!("[DBG][WIN32_MOUSE] Mouse button {:#X} clicked -> Reset engine buffer", w_param);
                    }
                    engine.reset();
                }
            }
        }
        CallNextHookEx(0, n_code, w_param, l_param)
    }
}

/// The Low-Level Keyboard Hook procedure
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
                if (is_ctrl_vk(vk) || is_shift_vk(vk)) && CTRL_SHIFT_ARMED.swap(false, Ordering::SeqCst) {
                    if let Ok(mut guard) = ENGINE_INSTANCE.lock() {
                        if let Some(ref mut engine) = *guard {
                            let new_state = engine.toggle_enabled();
                            if new_state {
                                println!("\n[MKey] >> Chế độ gõ: [V] TIẾNG VIỆT");
                            } else {
                                println!("\n[MKey] >> Chế độ gõ: [E] TIẾNG ANH");
                            }
                            crate::ui::update_tray_icon(new_state);
                        }
                    }
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
                } else {
                    CTRL_SHIFT_ARMED.store(false, Ordering::SeqCst);
                }

                // If Ctrl, Alt, Win, or navigation/editing keys are pressed, reset engine session
                if is_ctrl
                    || is_alt
                    || is_win
                    || matches!(
                        vk,
                        0x09 /* TAB */ | 0x1B /* ESC */ | VK_PRIOR | VK_NEXT | VK_END
                            | VK_HOME | VK_LEFT | VK_UP | VK_RIGHT | VK_DOWN | VK_INSERT
                            | VK_DELETE
                    )
                {
                    if let Ok(mut guard) = ENGINE_INSTANCE.lock() {
                        if let Some(ref mut engine) = *guard {
                            if engine.config().debug && !engine.buffer.is_empty() {
                                eprintln!("[DBG][WIN32_NAV] VK 0x{:02X} pressed -> Reset engine buffer", vk);
                            }
                            engine.reset();
                        }
                    }
                    return CallNextHookEx(0, n_code, w_param, l_param);
                }

                // Handle Backspace
                if vk == VK_BACK as u32 {
                    if let Ok(mut guard) = ENGINE_INSTANCE.lock() {
                        if let Some(ref mut engine) = *guard {
                            engine.on_backspace();
                        }
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

                if count > 0 {
                    if let Some(ch) = char::decode_utf16(buff[0..count as usize].iter().cloned())
                        .next()
                        .and_then(|r| r.ok())
                    {
                        if let Ok(mut guard) = ENGINE_INSTANCE.lock() {
                            if let Some(ref mut engine) = *guard {
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
                }
            }
        }

        CallNextHookEx(0, n_code, w_param, l_param)
    }
}

/// Installs the Windows keyboard & mouse hooks and starts the Win32 message pump
pub fn run_hook_loop(engine: VietnameseEngine, is_autostart: bool) {
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
        if !is_autostart {
            crate::ui::show_control_panel();
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
