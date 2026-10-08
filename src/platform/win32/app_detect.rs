//! Active Application Context & Browser Omnibox Autocomplete Detection

use super::types::{
    CloseHandle, GetClassNameW, GetForegroundWindow, GetGUIThreadInfo, GetWindowThreadProcessId,
    OpenProcess, QueryFullProcessImageNameW, GUITHREADINFO, PROCESS_QUERY_LIMITED_INFORMATION,
};
use std::sync::atomic::{AtomicU8, AtomicU32, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AppKind {
    Other = 0,
    Chromium = 1,
    GenericAutocomplete = 2,
}

impl From<&str> for AppKind {
    fn from(exe_name: &str) -> Self {
        match exe_name {
            "chrome.exe" | "msedge.exe" | "brave.exe" | "opera.exe" | "vivaldi.exe"
            | "coc_coc.exe" => AppKind::Chromium,
            "firefox.exe" | "excel.exe" => AppKind::GenericAutocomplete,
            _ => AppKind::Other,
        }
    }
}

impl From<u8> for AppKind {
    fn from(val: u8) -> Self {
        match val {
            1 => AppKind::Chromium,
            2 => AppKind::GenericAutocomplete,
            _ => AppKind::Other,
        }
    }
}

static CACHED_PID: AtomicU32 = AtomicU32::new(0);
static CACHED_APP_KIND: AtomicU8 = AtomicU8::new(0);

pub fn get_app_kind_for_pid(pid: u32) -> AppKind {
    let cached_pid = CACHED_PID.load(Ordering::Relaxed);
    if cached_pid == pid {
        return AppKind::from(CACHED_APP_KIND.load(Ordering::Relaxed));
    }

    let kind = unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if handle != 0 {
            let mut buf = [0u16; 1024];
            let mut size = buf.len() as u32;
            let ok = QueryFullProcessImageNameW(handle, 0, buf.as_mut_ptr(), &mut size);
            CloseHandle(handle);

            if ok != 0 && size > 0 {
                let full_path = String::from_utf16_lossy(&buf[..size as usize]);
                let exe_name = full_path.rsplit('\\').next().unwrap_or("").to_lowercase();
                AppKind::from(exe_name.as_str())
            } else {
                AppKind::Other
            }
        } else {
            AppKind::Other
        }
    };

    CACHED_PID.store(pid, Ordering::Relaxed);
    CACHED_APP_KIND.store(kind as u8, Ordering::Relaxed);
    kind
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutocompleteFixType {
    None,
    ChromiumOmnibox,
    GenericAutocomplete,
}

pub fn detect_autocomplete_context() -> AutocompleteFixType {
    unsafe {
        let foreground = GetForegroundWindow();
        if foreground == 0 {
            return AutocompleteFixType::None;
        }

        let mut pid = 0u32;
        let thread_id = GetWindowThreadProcessId(foreground, &mut pid);
        if pid == 0 {
            return AutocompleteFixType::None;
        }

        let app_kind = get_app_kind_for_pid(pid);
        if app_kind == AppKind::Other {
            return AutocompleteFixType::None;
        }

        let mut gui = GUITHREADINFO::default();
        let focus_hwnd = if GetGUIThreadInfo(thread_id, &mut gui) != 0 && gui.hwnd_focus != 0 {
            gui.hwnd_focus
        } else {
            foreground
        };

        let mut class_buf = [0u16; 128];
        let class_len = GetClassNameW(focus_hwnd, class_buf.as_mut_ptr(), 128);
        let class_name = if class_len > 0 {
            String::from_utf16_lossy(&class_buf[..class_len as usize])
        } else {
            String::new()
        };

        match app_kind {
            AppKind::Chromium => {
                // When focus is inside a web page (Facebook, Google Docs, ChatGPT, YouTube, etc.):
                // Chromium uses "Chrome_RenderWidgetHostHWND".
                // In that case, do NOT apply autocomplete fix: type completely normally!
                if class_name == "Chrome_RenderWidgetHostHWND" {
                    AutocompleteFixType::None
                } else {
                    AutocompleteFixType::ChromiumOmnibox
                }
            }
            AppKind::GenericAutocomplete => {
                if class_name == "MozillaContentWindowClass" {
                    AutocompleteFixType::None
                } else {
                    AutocompleteFixType::GenericAutocomplete
                }
            }
            AppKind::Other => AutocompleteFixType::None,
        }
    }
}

