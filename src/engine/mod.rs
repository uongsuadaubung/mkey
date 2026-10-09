pub mod action;
pub mod buffer;
pub mod config;
pub mod config_store;
pub mod history;
pub mod macro_table;

use crate::vietnamese::spelling::is_valid_vietnamese_syllable;
use action::EngineAction;
use buffer::{RawKey, TypingBuffer};
pub use config::EngineConfig;
use history::WordHistory;
use macro_table::MacroTable;
use std::sync::mpsc::{Sender, channel};

/// Non-blocking asynchronous log worker that writes debug logs to disk on a dedicated thread,
/// preventing any I/O disk stalls on the Windows low-level hook thread.
#[derive(Debug, Clone)]
pub struct AsyncLogWriter {
    sender: Option<Sender<String>>,
}

impl AsyncLogWriter {
    pub fn new(path: Option<String>) -> Self {
        if let Some(path) = path {
            let (tx, rx) = channel::<String>();
            let file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path);

            if let Ok(mut file) = file {
                let _ = std::thread::Builder::new()
                    .name("mkey-async-logger".to_string())
                    .spawn(move || {
                        use std::io::Write;
                        while let Ok(msg) = rx.recv() {
                            let _ = writeln!(file, "{}", msg);
                            let _ = file.flush();
                        }
                    });
                Self { sender: Some(tx) }
            } else {
                Self { sender: None }
            }
        } else {
            Self { sender: None }
        }
    }

    pub fn write_line(&self, line: String) {
        if let Some(ref sender) = self.sender {
            let _ = sender.send(line);
        }
    }

    pub fn is_active(&self) -> bool {
        self.sender.is_some()
    }
}

/// Clean, stateful, thread-safe Vietnamese Input Engine.
#[derive(Debug, Clone)]
pub struct VietnameseEngine {
    config: EngineConfig,
    pub buffer: TypingBuffer,
    pub macro_table: MacroTable,
    history: WordHistory,
    auto_uppercase_next: bool,
    pub debug_log: Vec<String>,
    log_writer: AsyncLogWriter,
}

impl Default for VietnameseEngine {
    fn default() -> Self {
        Self::new(EngineConfig::default())
    }
}

impl VietnameseEngine {
    pub fn new(config: EngineConfig) -> Self {
        let log_writer = AsyncLogWriter::new(config.debug_file_path.clone());
        Self {
            config,
            buffer: TypingBuffer::new(),
            macro_table: MacroTable::with_defaults(),
            history: WordHistory::new(),
            auto_uppercase_next: false,
            debug_log: Vec::new(),
            log_writer,
        }
    }

    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut EngineConfig {
        &mut self.config
    }

    /// Chuyển đổi qua lại giữa chế độ gõ Tiếng Việt và Tiếng Anh (trả về trạng thái mới: true là Tiếng Việt, false là Tiếng Anh)
    pub fn toggle_enabled(&mut self) -> bool {
        self.config.enabled = !self.config.enabled;
        self.reset();
        self.config.enabled
    }

    fn log_debug(&mut self, msg: String) {
        if self.config.debug {
            if !self.log_writer.is_active() && self.config.debug_file_path.is_some() {
                self.log_writer = AsyncLogWriter::new(self.config.debug_file_path.clone());
            }
            let timestamp = current_timestamp_str();
            let formatted = format!("[{}]{}", timestamp, msg);
            eprintln!("{}", formatted);

            // Ghi nhật ký bất đồng bộ ra background thread để triệt tiêu mọi I/O latency trên hook
            self.log_writer.write_line(formatted.clone());

            if self.debug_log.len() >= 500 {
                self.debug_log.remove(0);
            }
            self.debug_log.push(formatted);
        }
    }

    /// Lấy danh sách các bản ghi log gần nhất
    pub fn debug_log(&self) -> &[String] {
        &self.debug_log
    }

    /// Xuất toàn bộ log vết phím gần đây thành chuỗi
    pub fn dump_debug_log(&self) -> String {
        self.debug_log.join("\n")
    }

    /// Xóa sạch lịch sử debug log
    pub fn clear_debug_log(&mut self) {
        self.debug_log.clear();
    }

    /// Reset current word session and history (e.g. on mouse click or navigation keys)
    pub fn reset(&mut self) {
        if self.config.debug && !self.buffer.is_empty() {
            let prev_word: String = self.buffer.raw_keys.iter().map(|k| k.ch).collect();
            self.log_debug(format!(
                "[DBG][RESET] Session reset. Word in buffer was: {:?}",
                prev_word
            ));
        }
        self.buffer.clear();
        self.history.clear();
    }

    /// Feed a character into the engine with hardware Shift & CapsLock state.
    pub fn on_key(&mut self, ch: char, is_shift: bool, is_caps: bool) -> EngineAction {
        let prev_state_summary = self.buffer.state.summary();
        let mut is_upper = is_shift ^ is_caps;

        // Auto-uppercase first letter after period or newline
        if self.config.auto_uppercase_first_char
            && self.auto_uppercase_next
            && ch.is_alphabetic()
            && self.buffer.is_empty()
        {
            is_upper = true;
            self.auto_uppercase_next = false;
        }

        // Word break characters: space, enter, punctuation
        if ch.is_whitespace() || is_word_break(ch, self.config.bracket_w) {
            return self.handle_word_break(ch);
        }

        if !self.config.enabled {
            if self.config.use_macro && self.config.use_macro_in_english_mode {
                if ch.is_alphanumeric() {
                    self.buffer.raw_keys.push(RawKey { ch, is_upper });
                    self.buffer.emitted_len += 1;
                } else {
                    self.buffer.clear();
                }
            } else {
                self.buffer.clear();
            }
            return EngineAction::Passthrough;
        }

        let raw_key = RawKey { ch, is_upper };

        // If the incoming key begins a new CamelCase boundary, save preceding word into history
        if self.buffer.is_boundary(&raw_key) {
            let current_raw = self.buffer.raw_keys.clone();
            let rendered = self.buffer.state.render();

            let raw_str: String = current_raw.iter().map(|k| k.ch).collect();
            let should_restore = self.config.restore_on_wrong_spelling
                && rendered != raw_str
                && match self.buffer.state.to_syllable() {
                    Some(syllable) => !is_valid_vietnamese_syllable(&syllable),
                    None => !rendered.is_ascii(),
                };

            if should_restore {
                let backspaces = self.buffer.emitted_len;
                let mut output = raw_str.clone();
                output.push(ch);
                self.history.commit_word(current_raw, raw_str, false);
                self.buffer.clear();
                self.buffer.raw_keys.push(raw_key);
                let (next_state, _) = crate::vietnamese::SyllableState::Empty.feed_key(
                    raw_key,
                    0,
                    &self.config,
                );
                self.buffer.state = next_state;
                self.buffer.emitted_len = ch.len_utf16();
                return EngineAction::Replace {
                    backspaces,
                    output,
                };
            }

            self.history.commit_word(current_raw, rendered, false);
        }

        let action = self.buffer.feed_key(raw_key, &self.config);
        let next_state_summary = self.buffer.state.summary();
        let current_word: String = self.buffer.raw_keys.iter().map(|k| k.ch).collect();

        self.log_debug(format!(
            "[DBG][KEY] {:?} (shift={}, caps={}, upper={}) | State: {} -> {} | Word: {:?} (screen_len={}) | Action: {:?}",
            ch,
            is_shift as u8,
            is_caps as u8,
            is_upper as u8,
            prev_state_summary,
            next_state_summary,
            current_word,
            self.buffer.emitted_len,
            action
        ));

        action
    }

    fn handle_word_break(&mut self, ch: char) -> EngineAction {
        // Update auto-uppercase trigger
        if self.config.auto_uppercase_first_char && (ch == '.' || ch == '\n' || ch == '\r') {
            self.auto_uppercase_next = true;
        }

        // Check Macro expansion before clearing buffer
        if self.config.use_macro && !self.buffer.is_empty() {
            let current_word: String = self.buffer.raw_keys.iter().map(|k| k.ch).collect();
            let rendered = self.buffer.state.render();

            let is_space = ch == ' ';
            let allow_start = is_space;
            let allow_end = is_space;

            let expanded = self
                .macro_table
                .expand_word(&rendered, allow_start, allow_end)
                .or_else(|| {
                    self.macro_table
                        .expand_word(&current_word, allow_start, allow_end)
                });

            if let Some(expanded) = expanded {
                let backspaces = self.buffer.emitted_len;
                let mut output = expanded;
                output.push(ch);

                self.log_debug(format!(
                    "[DBG][MACRO] Key: {:?} | Expanded {:?} -> {:?} | Replace(bs={}, output={:?})",
                    ch, current_word, output, backspaces, output
                ));

                self.buffer.clear();
                if ch == ' ' {
                    self.history.commit_word(Vec::new(), output.clone(), false);
                    self.history.add_space();
                } else {
                    self.history.clear();
                }

                return EngineAction::Replace { backspaces, output };
            }
        }

        // Restore raw keys if the word is an invalid Vietnamese syllable
        if self.config.restore_on_wrong_spelling && !self.buffer.is_empty() {
            let raw_word: String = self.buffer.raw_keys.iter().map(|k| k.ch).collect();
            let rendered = self.buffer.state.render();

            let is_invalid = if rendered != raw_word {
                match self.buffer.state.to_syllable() {
                    Some(syllable) => !is_valid_vietnamese_syllable(&syllable),
                    None => !rendered.is_ascii(),
                }
            } else {
                false
            };

            if is_invalid {
                let backspaces = self.buffer.emitted_len;
                let mut output = raw_word.clone();
                output.push(ch);

                self.log_debug(format!(
                    "[DBG][RESTORE] Invalid word {:?} (screen: {:?}) -> Restoring to {:?}",
                    raw_word, rendered, output
                ));

                if ch == ' ' {
                    self.history
                        .commit_word(self.buffer.raw_keys.clone(), raw_word, true);
                    self.history.add_space();
                } else {
                    self.history.clear();
                }
                self.buffer.clear();

                return EngineAction::Replace { backspaces, output };
            }
        }

        // Commit word to history
        if !self.buffer.is_empty() {
            let current_raw = self.buffer.raw_keys.clone();
            let rendered = self.buffer.state.render();
            self.log_debug(format!(
                "[DBG][BREAK] Key: {:?} | Committed word: {:?} | Screen: {:?}",
                ch,
                current_raw.iter().map(|k| k.ch).collect::<String>(),
                rendered
            ));
            if ch == ' ' {
                self.history.commit_word(current_raw, rendered, false);
                self.history.add_space();
            } else {
                self.history.clear();
            }
        } else if ch == ' ' {
            self.history.add_space();
        } else {
            self.history.clear();
        }

        self.buffer.clear();
        EngineAction::Passthrough
    }

    /// Feed a backspace event
    pub fn on_backspace(&mut self) -> EngineAction {
        if !self.config.enabled {
            if self.config.use_macro && self.config.use_macro_in_english_mode && !self.buffer.is_empty() {
                self.buffer.raw_keys.pop();
                if self.buffer.emitted_len > 0 {
                    self.buffer.emitted_len -= 1;
                }
            } else {
                self.buffer.clear();
                self.history.clear();
            }
            return EngineAction::Passthrough;
        }

        let _prev_empty = self.buffer.is_empty();
        let prev_word: String = self.buffer.raw_keys.iter().map(|k| k.ch).collect();

        if self.buffer.is_empty() {
            let mut restored_word = None;
            if self.config.remember_history_across_space
                && let Some(restored) = self.history.pop_backspace()
                && let Some((raw_keys, is_raw_restored)) = restored
            {
                restored_word = Some(raw_keys.iter().map(|k| k.ch).collect::<String>());
                if is_raw_restored {
                    self.buffer.restore_as_passthrough(raw_keys);
                } else {
                    // Replay previous word into buffer
                    for k in raw_keys {
                        self.buffer.feed_key(k, &self.config);
                    }
                }
            }
            self.log_debug(format!(
                "[DBG][BS] Buffer was empty. History restored across space: {:?}",
                restored_word
            ));
            return EngineAction::Passthrough;
        }

        self.buffer.handle_backspace(&self.config);
        let now_word: String = self.buffer.raw_keys.iter().map(|k| k.ch).collect();
        self.log_debug(format!(
            "[DBG][BS] Backspaced. Word: {:?} -> {:?} | State: {} (screen_len={})",
            prev_word,
            now_word,
            self.buffer.state.summary(),
            self.buffer.emitted_len
        ));

        EngineAction::Passthrough
    }
}

fn is_word_break(c: char, bracket_w: bool) -> bool {
    if bracket_w && (c == '[' || c == ']') {
        return false;
    }
    matches!(
        c,
        '.' | ','
            | ';'
            | ':'
            | '!'
            | '?'
            | '"'
            | '\''
            | '('
            | ')'
            | '['
            | ']'
            | '{'
            | '}'
            | '/'
            | '\\'
            | '+'
            | '='
            | '*'
            | '<'
            | '>'
            | '@'
            | '#'
            | '$'
            | '%'
            | '^'
            | '&'
            | '_'
            | '`'
            | '~'
            | '|'
    )
}

/// Helper trả về chuỗi thời gian hiện tại chính xác đến mili-giây
pub fn current_timestamp_str() -> String {
    #[cfg(target_os = "windows")]
    {
        #[repr(C)]
        #[allow(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]
        struct SYSTEMTIME {
            wYear: u16,
            wMonth: u16,
            wDayOfWeek: u16,
            wDay: u16,
            wHour: u16,
            wMinute: u16,
            wSecond: u16,
            wMilliseconds: u16,
        }

        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn GetLocalTime(lpSystemTime: *mut SYSTEMTIME);
        }

        let mut st = std::mem::MaybeUninit::<SYSTEMTIME>::zeroed();
        unsafe {
            GetLocalTime(st.as_mut_ptr());
            let st = st.assume_init();
            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
                st.wYear, st.wMonth, st.wDay, st.wHour, st.wMinute, st.wSecond, st.wMilliseconds
            )
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        format!("{}.{:03}", now.as_secs(), now.subsec_millis())
    }
}
