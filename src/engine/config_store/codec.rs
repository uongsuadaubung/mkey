//! INI Parser and Serializer for EngineConfig and MacroTable

use crate::engine::config::EngineConfig;
use crate::engine::macro_table::MacroTable;

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
                (_, "switch_key_enabled") | (_, "use_switch_key") | (_, "switch_enabled") => {
                    config.switch_key_enabled = val_bool;
                }
                (_, "switch_key") | (_, "switch_with_ctrl_shift") => {
                    config.switch_key = crate::engine::config::Hotkey::from_config_str(val);
                    config.switch_with_ctrl_shift = config.switch_key.is_ctrl_shift();
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
    let mut out = String::from(
        "# Cấu hình MKey - Bộ gõ tiếng Việt hiện đại\n# Đường dẫn: ~/.config/mkey/config.ini\n\n",
    );

    out.push_str("[general]\n");
    out.push_str(&format!("method = {}\n", config.method));
    out.push_str(&format!("enabled = {}\n", config.enabled));
    out.push_str(&format!(
        "switch_key_enabled = {}\n",
        config.switch_key_enabled
    ));
    out.push_str(&format!(
        "switch_key = {}\n\n",
        config.switch_key.to_config_str()
    ));

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
