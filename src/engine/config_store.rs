//! Unified configuration & macro persistence store for MKey
//! File location: ~/.config/mkey/config.ini

use std::fs;
use std::path::PathBuf;
use crate::engine::config::{EngineConfig, InputMethod};
use crate::engine::macro_table::MacroTable;

/// Returns the absolute path to ~/.config/mkey/config.ini
pub fn get_config_path() -> PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config").join("mkey").join("config.ini")
}

/// Loads both EngineConfig and MacroTable from config.ini.
/// If file doesn't exist, initializes default config and saves it to disk.
/// Autostart state is read directly from Windows Registry as the source of truth.
pub fn load_config_and_macros() -> (EngineConfig, MacroTable) {
    let path = get_config_path();
    let (mut config, macros) = if !path.exists() {
        let config = EngineConfig::default();
        let macros = MacroTable::new();
        let _ = save_config_and_macros(&config, &macros);
        (config, macros)
    } else {
        match fs::read_to_string(&path) {
            Ok(content) => parse_config_and_macros(&content),
            Err(_) => (EngineConfig::default(), MacroTable::new()),
        }
    };

    // Luôn ưu tiên đọc trạng thái khởi động cùng Windows trực tiếp từ Registry
    config.autostart = crate::platform::is_windows_autostart_enabled();

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
                macros.insert(key, val);
            }
        } else {
            let key_l = key.to_lowercase();
            let val_l = val.to_lowercase();
            let val_bool = val_l == "true" || val_l == "1" || val_l == "yes";

            match key_l.as_str() {
                "method" => {
                    config.method = match val_l.as_str() {
                        "telex" => InputMethod::Telex,
                        "vni" => InputMethod::Vni,
                        "simple_telex1" | "simpletelex1" | "simpletelex" => InputMethod::SimpleTelex1,
                        "simple_telex2" | "simpletelex2" => InputMethod::SimpleTelex2,
                        _ => InputMethod::Telex,
                    };
                }
                "enabled" => config.enabled = val_bool,
                "switch_key" | "switch_with_ctrl_shift" => {
                    config.switch_with_ctrl_shift = val_l.contains("ctrl") || val_bool;
                }
                "check_spelling" => config.check_spelling = val_bool,
                "restore_on_wrong" | "restore_on_wrong_spelling" => config.restore_on_wrong_spelling = val_bool,
                "auto_uppercase_first" | "auto_uppercase_first_char" => config.auto_uppercase_first_char = val_bool,
                "use_macro" => config.use_macro = val_bool,
                "bracket_w" => config.bracket_w = val_bool,
                "quick_start_consonant" => config.quick_start_consonant = val_bool,
                "quick_end_consonant" => config.quick_end_consonant = val_bool,
                "remember_history" | "remember_history_across_space" => config.remember_history_across_space = val_bool,
                "autostart" => config.autostart = val_bool,
                "debug" => config.debug = val_bool,
                _ => {}
            }
        }
    }

    (config, macros)
}

/// Serializes EngineConfig and MacroTable into unified INI string format
pub fn serialize_config_and_macros(config: &EngineConfig, macros: &MacroTable) -> String {
    let method_str = match config.method {
        InputMethod::Telex => "telex",
        InputMethod::Vni => "vni",
        InputMethod::SimpleTelex1 => "simple_telex1",
        InputMethod::SimpleTelex2 => "simple_telex2",
    };

    let switch_str = if config.switch_with_ctrl_shift { "ctrl_shift" } else { "alt_z" };

    let mut out = String::from("# Cấu hình MKey - Bộ gõ tiếng Việt hiện đại\n# Đường dẫn: ~/.config/mkey/config.ini\n\n");

    out.push_str("[general]\n");
    out.push_str(&format!("method = {}\n", method_str));
    out.push_str(&format!("enabled = {}\n", config.enabled));
    out.push_str(&format!("switch_key = {}\n\n", switch_str));

    out.push_str("[spelling]\n");
    out.push_str(&format!("check_spelling = {}\n", config.check_spelling));
    out.push_str(&format!("restore_on_wrong = {}\n", config.restore_on_wrong_spelling));
    out.push_str(&format!("auto_uppercase_first = {}\n\n", config.auto_uppercase_first_char));

    out.push_str("[features]\n");
    out.push_str(&format!("use_macro = {}\n", config.use_macro));
    out.push_str(&format!("bracket_w = {}\n", config.bracket_w));
    out.push_str(&format!("quick_start_consonant = {}\n", config.quick_start_consonant));
    out.push_str(&format!("quick_end_consonant = {}\n", config.quick_end_consonant));
    out.push_str(&format!("remember_history = {}\n\n", config.remember_history_across_space));

    out.push_str("[system]\n");
    out.push_str(&format!("autostart = {}\n", config.autostart));
    out.push_str(&format!("debug = {}\n\n", config.debug));

    out.push_str("[macro]\n");
    out.push_str("# Danh sách từ gõ tắt: <từ viết tắt> = <cụm từ thay thế>\n");
    for (k, v) in macros.get_sorted_entries() {
        out.push_str(&format!("{} = {}\n", k, v));
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

    // Đồng bộ autostart vào Windows Registry nếu trạng thái khác biệt
    if crate::platform::is_windows_autostart_enabled() != config.autostart {
        crate::platform::set_windows_autostart(config.autostart);
    }

    Ok(())
}

pub use crate::platform::{is_windows_autostart_enabled, set_windows_autostart};

