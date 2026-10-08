//! Keystroke Injection & Native SendInput Synthesizer

use super::app_detect::{AutocompleteFixType, detect_autocomplete_context};
use super::types::{
    INPUT, INPUT_KEYBOARD, INPUT_UNION, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
    MAGIC_EXTRA_INFO, SendInput, VK_BACK, VK_END, VK_RIGHT, VK_SHIFT,
};

#[inline]
pub fn push_backspace(inputs: &mut Vec<INPUT>) {
    inputs.push(INPUT {
        r#type: INPUT_KEYBOARD,
        u: INPUT_UNION {
            ki: KEYBDINPUT {
                w_vk: VK_BACK as u16,
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
                w_vk: VK_BACK as u16,
                w_scan: 0x0E,
                dw_flags: KEYEVENTF_KEYUP,
                time: 0,
                dw_extra_info: MAGIC_EXTRA_INFO,
            },
        },
    });
}

#[inline]
pub fn push_unicode_char(inputs: &mut Vec<INPUT>, code_unit: u16) {
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

#[inline]
pub fn push_key_event(inputs: &mut Vec<INPUT>, vk: u16, scan: u16, flags: u32) {
    inputs.push(INPUT {
        r#type: INPUT_KEYBOARD,
        u: INPUT_UNION {
            ki: KEYBDINPUT {
                w_vk: vk,
                w_scan: scan,
                dw_flags: flags,
                time: 0,
                dw_extra_info: MAGIC_EXTRA_INFO,
            },
        },
    });
}

#[inline]
pub fn push_combine_key(inputs: &mut Vec<INPUT>, mod_vk: u16, key_vk: u16, key_flags: u32) {
    push_key_event(inputs, mod_vk, 0, 0);
    push_key_event(inputs, key_vk, 0, key_flags);
    push_key_event(inputs, key_vk, 0, key_flags | KEYEVENTF_KEYUP);
    push_key_event(inputs, mod_vk, 0, KEYEVENTF_KEYUP);
}

/// Sends backspaces and replacement string in a SINGLE atomic SendInput batch.
/// - In normal apps & web pages: sends pure standard Backspaces and replacement characters (no invisible chars).
/// - In browser Omnibox / Excel: collapses inline autocomplete selection cleanly without polluting document text.
pub fn send_replace(backspaces: usize, text: &str) {
    let utf16: Vec<u16> = text.encode_utf16().collect();
    if backspaces == 0 && utf16.is_empty() {
        return;
    }

    let mut inputs = Vec::with_capacity(4 + (backspaces + 1) * 2 + utf16.len() * 2);

    let fix_type = detect_autocomplete_context();
    match fix_type {
        AutocompleteFixType::ChromiumOmnibox => {
            // Omnibox fix:
            // Shift + Right collapses/drops current omnibox autocomplete selection
            // while positioning caret at true end of typed text without any extra Backspace needed
            push_combine_key(&mut inputs, VK_SHIFT as u16, VK_RIGHT as u16, 0);
            for _ in 0..backspaces {
                push_backspace(&mut inputs);
            }
        }
        AutocompleteFixType::GenericAutocomplete => {
            // Firefox / Excel fix:
            // End key deselects autocomplete suggestion and puts caret right at end of typed word
            push_key_event(&mut inputs, VK_END as u16, 0, 0);
            push_key_event(&mut inputs, VK_END as u16, 0, KEYEVENTF_KEYUP);
            for _ in 0..backspaces {
                push_backspace(&mut inputs);
            }
        }
        AutocompleteFixType::None => {
            // Normal desktop applications & web pages (Chrome_RenderWidgetHostHWND, Word, Notepad, VSCode, etc.)
            for _ in 0..backspaces {
                push_backspace(&mut inputs);
            }
        }
    }

    // 2. Synthesize all replacement characters
    for &ch in &utf16 {
        push_unicode_char(&mut inputs, ch);
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
