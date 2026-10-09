//! Keystroke Injection & Native SendInput Synthesizer

use super::app_detect::{AutocompleteFixType, detect_autocomplete_context};
use super::types::{
    INPUT, INPUT_KEYBOARD, INPUT_UNION, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
    MAGIC_EXTRA_INFO, SendInput, VK_BACK,
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
        AutocompleteFixType::ChromiumOmnibox | AutocompleteFixType::GenericAutocomplete => {
            // Autocomplete / Omnibox fix (Chrome, Edge, Brave, Firefox, Excel):
            // When user types in browser address bar with an active suggestion (e.g. "truye[nqq.com.vn/]"),
            // sending keys directly or sending Shift+Right / End causes the suggestion to be accepted,
            // resulting in unwanted appending (e.g. "truyenqq.com.vnê").
            //
            // Solution:
            // 1. Send Unicode U+202F (Narrow No-Break Space).
            //    In any text field with selected autocomplete text, typing a character immediately
            //    overwrites and neutralizes the active selection without moving caret to the end.
            // 2. Send 1 backspace to erase the temporary U+202F character.
            // 3. Send the normal `backspaces` to delete the target characters to be replaced.
            if backspaces > 0 && !utf16.is_empty() {
                push_unicode_char(&mut inputs, 0x202F);
                push_backspace(&mut inputs);
            }
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
