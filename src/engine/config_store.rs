//! Unified configuration & macro persistence store for MKey
//! File location: ~/.config/mkey/config.ini

use crate::engine::config::EngineConfig;
use crate::engine::macro_table::MacroTable;
use std::fs;
use std::path::PathBuf;

/// Returns the absolute path to ~/.config/mkey/config.ini
pub fn get_config_path() -> PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".config")
        .join("mkey")
        .join("config.ini")
}

/// Returns the path to the directory containing switch soundpacks if it exists.
/// Checks ~/.config/mkey/switches, ~/.config/mkey/switchs, ./switches, ./switchs.
pub fn get_switches_dir() -> Option<PathBuf> {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    let config_dir = PathBuf::from(home).join(".config").join("mkey");
    let candidates = [
        config_dir.join("switches"),
        config_dir.join("switchs"),
        PathBuf::from("switches"),
        PathBuf::from("switchs"),
    ];
    candidates.into_iter().find(|p| p.is_dir())
}

/// Lists all available switch profiles (subdirectories in the switches directory)
pub fn list_switch_profiles() -> Vec<String> {
    let mut profiles = Vec::new();
    if let Some(dir) = get_switches_dir()
        && let Ok(entries) = fs::read_dir(dir)
    {
        for entry in entries.flatten() {
            if let Ok(ft) = entry.file_type()
                && ft.is_dir()
                && let Some(name) = entry.file_name().to_str()
            {
                profiles.push(name.to_string());
            }
        }
    }
    profiles.sort();
    profiles
}

/// Returns whether the mechanical keyboard sound feature is available
pub fn is_sound_available() -> bool {
    !list_switch_profiles().is_empty()
}

/// Loads both EngineConfig and MacroTable from config.ini.
/// If file doesn't exist, initializes default config and saves it to disk.
/// Autostart state is read directly from Windows Registry as the source of truth.
pub fn load_config_and_macros() -> (EngineConfig, MacroTable) {
    let path = get_config_path();
    let (config, macros) = if !path.exists() {
        let config = EngineConfig::default();
        let macros = MacroTable::with_defaults();
        let _ = save_config_and_macros(&config, &macros);
        (config, macros)
    } else {
        match fs::read_to_string(&path) {
            Ok(content) => parse_config_and_macros(&content),
            Err(_) => (EngineConfig::default(), MacroTable::with_defaults()),
        }
    };
    crate::language::set_current_language(config.language);
    (config, macros)
}

/// Parse INI content into EngineConfig and MacroTable
pub fn parse_config_and_macros(content: &str) -> (EngineConfig, MacroTable) {
    let mut config = EngineConfig::default();
    let mut macros = MacroTable::new();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].trim().to_lowercase();
            continue;
        }

        let (key, val) = if let Some((k, v)) = line.split_once('=') {
            (k.trim(), v.trim())
        } else if let Some((k, v)) = line.split_once(':') {
            (k.trim(), v.trim())
        } else {
            continue;
        };

        if current_section == "macro" {
            if !key.is_empty() && !val.is_empty() {
                let (real_val, mtype) = if let Some((v, t)) = val.split_once(':') {
                    (v.trim(), crate::engine::macro_table::MacroType::from(t))
                } else {
                    (val, crate::engine::macro_table::MacroType::Normal)
                };
                macros.insert_typed_no_cache(key, real_val, mtype);
            }
        } else {
            let key_l = key.to_lowercase();
            let val_l = val.to_lowercase();
            let val_bool = val_l == "true" || val_l == "1" || val_l == "yes";

            match (current_section.as_str(), key_l.as_str()) {
                ("sound", "enabled") | (_, "sound_enabled") | (_, "sound") => {
                    config.sound_enabled = val_bool;
                }
                (_, "method") => {
                    config.method = val_l.parse().unwrap_or_default();
                }
                (_, "enabled") => config.enabled = val_bool,
                (_, "switch_key") | (_, "switch_with_ctrl_shift") => {
                    config.switch_with_ctrl_shift = val_l.contains("ctrl") || val_bool;
                }
                (_, "restore_on_wrong") | (_, "restore_on_wrong_spelling") => {
                    config.restore_on_wrong_spelling = val_bool;
                }
                (_, "auto_uppercase_first") | (_, "auto_uppercase_first_char") => {
                    config.auto_uppercase_first_char = val_bool;
                }
                (_, "use_macro") => config.use_macro = val_bool,
                (_, "bracket_w") => config.bracket_w = val_bool,
                (_, "remember_history") | (_, "remember_history_across_space") => {
                    config.remember_history_across_space = val_bool;
                }
                (_, "show_dialog_on_startup") | (_, "show_dialog") => {
                    config.show_dialog_on_startup = val_bool;
                }
                (_, "debug") => config.debug = val_bool,
                (_, "theme") => {
                    config.theme = val_l.parse().unwrap_or_default();
                }
                (_, "language") | (_, "lang") => {
                    config.language = val_l.parse().unwrap_or_default();
                }
                (_, "sound_profile") | (_, "switch") | (_, "switch_type") => {
                    config.sound_profile = val.trim().to_string();
                }
                (_, "sound_volume") | (_, "volume") => {
                    config.sound_volume = val_l.parse().unwrap_or(50).clamp(0, 100);
                }
                _ => {}
            }
        }
    }

    macros.rebuild_cache();
    (config, macros)
}

/// Serializes EngineConfig and MacroTable into unified INI string format
pub fn serialize_config_and_macros(config: &EngineConfig, macros: &MacroTable) -> String {
    let switch_str = if config.switch_with_ctrl_shift {
        "ctrl_shift"
    } else {
        "alt_z"
    };

    let mut out = String::from(
        "# Cấu hình MKey - Bộ gõ tiếng Việt hiện đại\n# Đường dẫn: ~/.config/mkey/config.ini\n\n",
    );

    out.push_str("[general]\n");
    out.push_str(&format!("method = {}\n", config.method));
    out.push_str(&format!("enabled = {}\n", config.enabled));
    out.push_str(&format!("switch_key = {}\n\n", switch_str));

    out.push_str("[spelling]\n");
    out.push_str(&format!(
        "restore_on_wrong = {}\n",
        config.restore_on_wrong_spelling
    ));
    out.push_str(&format!(
        "auto_uppercase_first = {}\n\n",
        config.auto_uppercase_first_char
    ));

    out.push_str("[features]\n");
    out.push_str(&format!("use_macro = {}\n", config.use_macro));
    out.push_str(&format!("bracket_w = {}\n", config.bracket_w));
    out.push_str(&format!(
        "remember_history = {}\n\n",
        config.remember_history_across_space
    ));

    out.push_str("[sound]\n");
    out.push_str(&format!("enabled = {}\n", config.sound_enabled));
    out.push_str(&format!("switch = {}\n", config.sound_profile));
    out.push_str(&format!("volume = {}\n\n", config.sound_volume));

    out.push_str("[system]\n");
    out.push_str(&format!("language = {}\n", config.language));
    out.push_str(&format!(
        "show_dialog_on_startup = {}\n",
        config.show_dialog_on_startup
    ));
    out.push_str(&format!("debug = {}\n", config.debug));
    out.push_str(&format!("theme = {}\n\n", config.theme));

    out.push_str("[macro]\n");
    out.push_str(
        "# Danh sách từ gõ tắt: <từ viết tắt> = <cụm từ thay thế>[:loại (normal/start/end)]\n",
    );
    for entry in macros.get_sorted_entries() {
        if entry.macro_type == crate::engine::macro_table::MacroType::Normal {
            out.push_str(&format!("{} = {}\n", entry.key, entry.value));
        } else {
            out.push_str(&format!(
                "{} = {}:{}\n",
                entry.key,
                entry.value,
                entry.macro_type.as_str()
            ));
        }
    }

    out
}

/// Serializes EngineConfig and MacroTable into unified INI format and saves to disk
pub fn save_config_and_macros(config: &EngineConfig, macros: &MacroTable) -> std::io::Result<()> {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let out = serialize_config_and_macros(config, macros);
    fs::write(&path, out)?;

    Ok(())
}

pub use crate::platform::{is_windows_autostart_enabled, set_windows_autostart};
