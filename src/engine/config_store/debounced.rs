//! Debounced background file saver for EngineConfig and MacroTable

use super::codec::serialize_config_and_macros;
use super::paths::get_config_path;
use crate::engine::config::EngineConfig;
use crate::engine::macro_table::MacroTable;
use std::fs;
use std::sync::{Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

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

struct DebounceState {
    pending: Option<(EngineConfig, MacroTable)>,
    last_request: Instant,
    shutdown: bool,
}

static DEBOUNCER: OnceLock<(Mutex<DebounceState>, Condvar)> = OnceLock::new();

fn get_debouncer() -> &'static (Mutex<DebounceState>, Condvar) {
    DEBOUNCER.get_or_init(|| {
        let pair = (
            Mutex::new(DebounceState {
                pending: None,
                last_request: Instant::now(),
                shutdown: false,
            }),
            Condvar::new(),
        );

        let _ = std::thread::Builder::new()
            .name("mkey-config-debouncer".to_string())
            .spawn(|| {
                let (lock, cvar) = get_debouncer();
                let debounce_duration = Duration::from_millis(500);

                loop {
                    let mut state = lock.lock().unwrap();

                    while state.pending.is_none() && !state.shutdown {
                        state = cvar.wait(state).unwrap();
                    }

                    if state.shutdown {
                        if let Some((cfg, mac)) = state.pending.take() {
                            let _ = save_config_and_macros(&cfg, &mac);
                        }
                        break;
                    }

                    let elapsed = state.last_request.elapsed();
                    if elapsed < debounce_duration {
                        let remaining = debounce_duration - elapsed;
                        let (new_state, _) = cvar.wait_timeout(state, remaining).unwrap();
                        state = new_state;
                        continue;
                    }

                    if let Some((cfg, mac)) = state.pending.take() {
                        drop(state);
                        if let Err(e) = save_config_and_macros(&cfg, &mac) {
                            eprintln!("[MKey] Lỗi lưu cấu hình: {e}");
                        }
                    }
                }
            });

        pair
    })
}

/// Requests an asynchronous, debounced save of EngineConfig and MacroTable to disk.
/// Multiple rapid calls (e.g. from hotkeys or sliders) are coalesced into a single write after 500ms of inactivity.
pub fn save_config_and_macros_debounced(config: &EngineConfig, macros: &MacroTable) {
    let (lock, cvar) = get_debouncer();
    let mut state = lock.lock().unwrap();
    state.pending = Some((config.clone(), macros.clone()));
    state.last_request = Instant::now();
    cvar.notify_one();
}

/// Immediately flushes any pending debounced config saves to disk.
pub fn flush_config_debounced() {
    if let Some((lock, _)) = DEBOUNCER.get() {
        let mut state = lock.lock().unwrap();
        if let Some((cfg, mac)) = state.pending.take() {
            let _ = save_config_and_macros(&cfg, &mac);
        }
    }
}
