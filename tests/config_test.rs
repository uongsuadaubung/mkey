use mkey::{
    EngineConfig, InputMethod, MacroTable, UiTheme, get_config_path, parse_config_and_macros,
};

#[test]
fn test_default_config_has_macro_disabled() {
    let config = EngineConfig::default();
    assert!(!config.use_macro, "use_macro must be false by default");
    assert!(
        config.switch_with_ctrl_shift,
        "switch_with_ctrl_shift should default to true"
    );
    assert!(
        config.show_dialog_on_startup,
        "show_dialog_on_startup should default to true"
    );
}

#[test]
fn test_parse_config_and_macros() {
    let ini = r#"
# Sample config file
[general]
method = vni
enabled = false
switch_key = ctrl_shift

[spelling]
check_spelling = true
restore_on_wrong = false
auto_uppercase_first = true

[features]
use_macro = true
bracket_w = false
quick_start_consonant = true
quick_end_consonant = true
remember_history = true

[sound]
enabled = true
switch = NovelKeys Cream
volume = 85

[system]
language = en
show_dialog_on_startup = false
debug = true
theme = dark

[macro]
ko = không
dc = được
vn = Việt Nam
"#;

    let (config, macros) = parse_config_and_macros(ini);

    assert_eq!(config.method, InputMethod::Vni);
    assert!(!config.enabled);
    assert!(config.switch_with_ctrl_shift);
    assert!(!config.restore_on_wrong_spelling);
    assert!(config.auto_uppercase_first_char);
    assert!(config.use_macro);
    assert!(!config.bracket_w);
    assert!(config.remember_history_across_space);
    assert!(config.sound_enabled);
    assert_eq!(config.sound_profile, "NovelKeys Cream");
    assert_eq!(config.sound_volume, 85);
    assert_eq!(config.language, mkey::Language::English);
    assert!(!config.show_dialog_on_startup);
    assert!(config.debug);
    assert_eq!(config.theme, UiTheme::Dark);

    assert_eq!(macros.lookup("ko"), Some("không".to_string()));
    assert_eq!(macros.lookup("dc"), Some("được".to_string()));
    assert_eq!(macros.lookup("vn"), Some("Việt Nam".to_string()));
    assert_eq!(macros.len(), 3);
}

#[test]
fn test_config_store_roundtrip() {
    let config = EngineConfig {
        method: InputMethod::SimpleTelex1,
        language: mkey::Language::English,
        use_macro: false,
        show_dialog_on_startup: false,
        theme: UiTheme::Dark,
        sound_enabled: true,
        sound_profile: "Topre".to_string(),
        sound_volume: 80,
        ..Default::default()
    };

    let mut macros = MacroTable::new();
    macros.insert("test", "thử nghiệm");
    macros.insert("rust", "ngôn ngữ Rust");

    // Serialize to INI string and parse back
    let ini_str = mkey::serialize_config_and_macros(&config, &macros);
    let (loaded_config, loaded_macros) = parse_config_and_macros(&ini_str);

    assert_eq!(loaded_config.method, InputMethod::SimpleTelex1);
    assert_eq!(loaded_config.language, mkey::Language::English);
    assert!(!loaded_config.use_macro);
    assert!(!loaded_config.show_dialog_on_startup);
    assert_eq!(loaded_config.theme, UiTheme::Dark);
    assert!(loaded_config.sound_enabled);
    assert_eq!(loaded_config.sound_profile, "Topre");
    assert_eq!(loaded_config.sound_volume, 80);
    assert_eq!(loaded_macros.lookup("test"), Some("thử nghiệm".to_string()));
    assert_eq!(
        loaded_macros.lookup("rust"),
        Some("ngôn ngữ Rust".to_string())
    );

    // Verify config path helper is valid
    let path = get_config_path();
    assert!(
        path.to_string_lossy().contains(".config"),
        "Config path should be in .config"
    );
}

#[test]
fn test_inspect_sound_banks() {
    if let Some(switches_dir) = mkey::engine::config_store::get_switches_dir()
        && let Ok(entries) = std::fs::read_dir(&switches_dir)
    {
        for entry in entries.flatten() {
            if let Ok(ft) = entry.file_type()
                && ft.is_dir()
            {
                let switch_name = entry.file_name().to_string_lossy().to_string();
                let mut space_files = Vec::new();
                let mut backspace_files = Vec::new();
                let mut normal_files = Vec::new();
                if let Ok(sub) = std::fs::read_dir(entry.path()) {
                    for f in sub.flatten() {
                        let fname = f.file_name().to_string_lossy().to_string();
                        let stem = f
                            .path()
                            .file_stem()
                            .map(|s| s.to_string_lossy().to_ascii_lowercase())
                            .unwrap_or_default();
                        match mkey::platform::win32::match_file_tag(&stem) {
                            Some(mkey::platform::win32::KeyTag::Space) => space_files.push(fname),
                            Some(mkey::platform::win32::KeyTag::Backspace) => {
                                backspace_files.push(fname)
                            }
                            None => normal_files.push(fname),
                            _ => {}
                        }
                    }
                }
                eprintln!(
                    "Switch [{}] -> Space: {:?} | Backspace: {:?} | Normal: {:?}",
                    switch_name, space_files, backspace_files, normal_files
                );
                assert_eq!(
                    space_files,
                    vec!["space.wav"],
                    "Space should only contain space.wav for {}",
                    switch_name
                );
                assert_eq!(
                    backspace_files,
                    vec!["backspace.wav"],
                    "Backspace should only contain backspace.wav for {}",
                    switch_name
                );
            }
        }
    }
}
