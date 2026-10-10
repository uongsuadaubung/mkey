//! Persistent configuration store and debounced I/O for MKey

pub mod codec;
pub mod debounced;
pub mod paths;

pub use codec::*;
pub use debounced::*;
pub use paths::*;

use crate::engine::config::EngineConfig;
use crate::engine::macro_table::MacroTable;
use std::fs;

/// Loads EngineConfig and MacroTable from disk, falling back to defaults if not found.
/// Automatically sets the UI localization language based on the loaded configuration.
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
