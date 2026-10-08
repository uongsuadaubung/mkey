//! MKey Vietnamese Typing Engine in Rust
//!
//! A clean, memory-safe, modular rewrite of Vietnamese input engine.
//! Designed to eliminate global state, buffer corruption, and case-mismatch bugs.
#![allow(non_snake_case)]

pub mod engine;
pub mod platform;
#[cfg(windows)]
pub mod ui;
pub mod vietnamese;

pub use engine::{
    VietnameseEngine,
    action::EngineAction,
    config::{EngineConfig, InputMethod, UiTheme},
    config_store::{
        get_config_path, load_config_and_macros, parse_config_and_macros, save_config_and_macros,
        serialize_config_and_macros, set_windows_autostart,
    },
    macro_table::MacroTable,
};
