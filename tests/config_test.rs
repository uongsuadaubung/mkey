use mkey::{
    get_config_path, parse_config_and_macros,
    EngineConfig, InputMethod, MacroTable,
};

#[test]
fn test_default_config_has_macro_disabled() {
    let config = EngineConfig::default();
    assert!(!config.use_macro, "use_macro must be false by default");
    assert!(config.switch_with_ctrl_shift, "switch_with_ctrl_shift should default to true");
    assert!(!config.autostart, "autostart should default to false");
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

[system]
autostart = true
debug = true

[macro]
ko = không
dc = được
vn = Việt Nam
"#;

    let (config, macros) = parse_config_and_macros(ini);

    assert_eq!(config.method, InputMethod::Vni);
    assert!(!config.enabled);
    assert!(config.switch_with_ctrl_shift);
    assert!(config.check_spelling);
    assert!(!config.restore_on_wrong_spelling);
    assert!(config.auto_uppercase_first_char);
    assert!(config.use_macro);
    assert!(!config.bracket_w);
    assert!(config.quick_start_consonant);
    assert!(config.quick_end_consonant);
    assert!(config.remember_history_across_space);
    assert!(config.autostart);
    assert!(config.debug);

    assert_eq!(macros.lookup("ko"), Some("không".to_string()));
    assert_eq!(macros.lookup("dc"), Some("được".to_string()));
    assert_eq!(macros.lookup("vn"), Some("Việt Nam".to_string()));
    assert_eq!(macros.len(), 3);
}

#[test]
fn test_config_store_roundtrip() {
    let mut config = EngineConfig::default();
    config.method = InputMethod::SimpleTelex1;
    config.use_macro = false;
    config.quick_start_consonant = true;
    config.autostart = false;

    let mut macros = MacroTable::new();
    macros.insert("test", "thử nghiệm");
    macros.insert("rust", "ngôn ngữ Rust");

    // Serialize to INI string and parse back
    let ini_str = mkey::serialize_config_and_macros(&config, &macros);
    let (loaded_config, loaded_macros) = parse_config_and_macros(&ini_str);

    assert_eq!(loaded_config.method, InputMethod::SimpleTelex1);
    assert!(!loaded_config.use_macro);
    assert!(loaded_config.quick_start_consonant);
    assert_eq!(loaded_macros.lookup("test"), Some("thử nghiệm".to_string()));
    assert_eq!(loaded_macros.lookup("rust"), Some("ngôn ngữ Rust".to_string()));

    // Verify config path helper is valid
    let path = get_config_path();
    assert!(path.to_string_lossy().contains(".config"), "Config path should be in .config");
}

#[test]
fn test_windows_autostart_registry_toggle() {
    #[cfg(target_os = "windows")]
    {
        use mkey::platform::{is_windows_autostart_enabled, set_windows_autostart};

        // 1. Enable autostart
        set_windows_autostart(true);
        assert!(is_windows_autostart_enabled(), "Registry must reflect autostart enabled");

        // 2. Disable autostart
        set_windows_autostart(false);
        assert!(!is_windows_autostart_enabled(), "Registry must reflect autostart disabled");
    }
}
