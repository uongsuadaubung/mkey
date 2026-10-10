use super::{action::EngineAction, config::EngineConfig};
use crate::vietnamese::{InlineList, state::SyllableState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawKey {
    pub ch: char,
    pub is_upper: bool,
}

impl From<char> for RawKey {
    #[inline]
    fn from(ch: char) -> Self {
        Self {
            ch,
            is_upper: ch.is_uppercase(),
        }
    }
}

/// Result of evaluating a sequence of raw keys in the typing buffer
#[derive(Debug, Clone)]
pub struct EvaluationResult {
    pub state: SyllableState,
    pub rendered: String,
    pub emitted_len: usize,
    pub effective_raw_keys: Vec<RawKey>,
    pub last_action: EngineAction,
}

/// TypingBuffer managing word state with raw_keys as the Single Source of Truth.
/// SyllableState is deterministically projected from raw_keys.
#[derive(Debug, Clone, Default)]
pub struct TypingBuffer {
    /// History of raw keys entered in this word session (Single Source of Truth)
    pub raw_keys: Vec<RawKey>,
    /// Projected state in the state machine (Empty, Onset, Nucleus, Coda, Passthrough)
    pub state: SyllableState,
    /// Length in chars of the rendered text currently emitted to the screen
    pub emitted_len: usize,
    /// Cached rendered string of the current state, avoiding re-renders and heap allocations
    pub last_rendered: String,
    /// Whether this buffer is in raw passthrough mode (e.g. restored from spelling correction)
    pub is_passthrough: bool,
    /// Whether this buffer was restored from history across a space via backspace
    pub is_restored_across_space: bool,
}

impl TypingBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn rendered(&self) -> &str {
        &self.last_rendered
    }

    pub fn clear(&mut self) {
        self.raw_keys.clear();
        self.state = SyllableState::Empty;
        self.emitted_len = 0;
        self.last_rendered.clear();
        self.is_passthrough = false;
        self.is_restored_across_space = false;
    }

    pub fn restore_as_passthrough(&mut self, keys: Vec<RawKey>) {
        let text: String = keys.iter().map(|k| k.ch).collect();
        self.emitted_len = text.encode_utf16().count();
        self.state = SyllableState::Passthrough(InlineList::from(text.as_str()));
        self.last_rendered = text;
        self.raw_keys = keys;
        self.is_passthrough = true;
        self.is_restored_across_space = true;
    }

    pub fn is_empty(&self) -> bool {
        self.raw_keys.is_empty()
    }

    /// Evaluates a sequence of RawKeys deterministically and returns an `EvaluationResult`.
    pub fn evaluate_keys(keys: &[RawKey], config: &EngineConfig) -> EvaluationResult {
        let mut state = SyllableState::Empty;
        let mut current_len = 0;
        let mut effective = Vec::with_capacity(keys.len());
        let mut last_action = EngineAction::Passthrough;
        let mut last_rendered = String::new();

        for (idx, &key) in keys.iter().enumerate() {
            effective.push(key);
            let was_nucleus_or_coda =
                matches!(state, SyllableState::Nucleus(_) | SyllableState::Coda(_));
            let prev_rendered = last_rendered.clone();
            let prev_rendered_len = last_rendered.chars().count();

            let (mut next_state, mut action) = state.feed_key(key, current_len, config);

            // Smart English Word Bypass / Instant Restore on Wrong Spelling:
            // If the state machine just transitioned from a structured Vietnamese state into Passthrough,
            // and the rendered text on screen had modifications (diacritics/tones),
            // rollback immediately to raw keys!
            if config.restore_on_wrong_spelling
                && was_nucleus_or_coda
                && matches!(next_state, SyllableState::Passthrough(_))
                && matches!(action, EngineAction::Passthrough)
            {
                let raw_str: String = keys[..=idx].iter().map(|k| k.ch).collect();
                let prev_raw_slice: String = keys[..idx].iter().map(|k| k.ch).collect();
                if prev_rendered != prev_raw_slice {
                    next_state = SyllableState::Passthrough(InlineList::from(raw_str.as_str()));
                    action = EngineAction::Replace {
                        backspaces: prev_rendered_len,
                        output: raw_str,
                    };
                }
            }

            state = next_state;
            match &action {
                EngineAction::Replace { output, .. } => {
                    current_len = output.encode_utf16().count();
                    last_rendered = output.clone();
                    // If an undo toggle transitioned into Passthrough (e.g. toanss -> toans, chuww -> chuw),
                    // synchronize effective keys to match the explicit cancelled output.
                    if let SyllableState::Passthrough(raw) = state {
                        effective = raw.iter().map(|&c| RawKey::from(c)).collect();
                    }
                }
                EngineAction::Passthrough => {
                    current_len += key.ch.len_utf16();
                    last_rendered.push(key.ch);
                }
                EngineAction::Consume => {}
            }
            last_action = action;
        }

        let rendered =
            if matches!(state, SyllableState::Passthrough(_)) && !last_rendered.is_empty() {
                last_rendered
            } else {
                state.render()
            };

        EvaluationResult {
            state,
            rendered,
            emitted_len: current_len,
            effective_raw_keys: effective,
            last_action,
        }
    }

    /// Checks whether `key` marks a word boundary (e.g. CamelCase / PascalCase boundary).
    pub fn is_boundary(&self, key: &RawKey) -> bool {
        if self.raw_keys.is_empty() {
            return false;
        }

        key.is_upper
            && self
                .raw_keys
                .iter()
                .any(|k| !k.is_upper && k.ch.is_alphabetic())
    }

    /// Feeds a new key into the state machine and returns the resulting EngineAction.
    pub fn feed_key(&mut self, key: RawKey, config: &EngineConfig) -> EngineAction {
        if self.is_boundary(&key) {
            self.clear();
        }

        // If this word was restored across space from history, test if the incoming key
        // is legitimately modifying / extending this word (e.g. adding tone 's' to "chao" -> "cháo").
        // If the key would degrade the state into Passthrough (or if it's already in passthrough),
        // the user is typing a NEW WORD! We clear the restored word so the new word can type cleanly.
        if self.is_restored_across_space {
            self.is_restored_across_space = false;
            let (test_state, _) = self.state.feed_key(key, self.emitted_len, config);
            if matches!(test_state, SyllableState::Passthrough(_)) || self.is_passthrough {
                self.clear();
            }
        }

        self.raw_keys.push(key);

        let was_nucleus_or_coda = matches!(
            self.state,
            SyllableState::Nucleus(_) | SyllableState::Coda(_)
        );
        let prev_rendered = self.last_rendered.clone();
        let prev_rendered_len = self.last_rendered.chars().count();

        let current_state = std::mem::take(&mut self.state);
        let (mut next_state, mut action) = current_state.feed_key(key, self.emitted_len, config);

        // Smart English Word Bypass / Instant Restore on Wrong Spelling:
        // If the state machine just transitioned from a structured Vietnamese state into Passthrough,
        // and the rendered text on screen had modifications (diacritics/tones),
        // rollback immediately to raw keys!
        if config.restore_on_wrong_spelling
            && was_nucleus_or_coda
            && matches!(next_state, SyllableState::Passthrough(_))
            && matches!(action, EngineAction::Passthrough)
        {
            let raw_str: String = self.raw_keys.iter().map(|k| k.ch).collect();
            let prev_raw_slice: String = self.raw_keys[..self.raw_keys.len() - 1]
                .iter()
                .map(|k| k.ch)
                .collect();
            if prev_rendered != prev_raw_slice {
                next_state = SyllableState::Passthrough(InlineList::from(raw_str.as_str()));
                action = EngineAction::Replace {
                    backspaces: prev_rendered_len,
                    output: raw_str,
                };
            }
        }

        self.state = next_state;

        // Common Prefix Optimization:
        // If the unchanged prefix of the word (e.g. onset 'g' in "go" -> "gõ") remains identical,
        // do not backspace and re-emit it. Only backspace the changed suffix ("o") and emit ("õ").
        // This eliminates duplicate characters ("ggõ") on browser address bars (Firefox, Chrome).
        let (action, new_emitted_len) = match action {
            EngineAction::Replace { backspaces, output } => {
                let full_output_utf16 = output.encode_utf16().count();
                let mut prev_iter = self.last_rendered.chars();
                let mut new_iter = output.chars();
                let mut common = 0;
                while let (Some(c1), Some(c2)) = (prev_iter.next(), new_iter.next()) {
                    if c1 == c2 {
                        common += 1;
                    } else {
                        break;
                    }
                }

                let prev_chars_count = self.last_rendered.chars().count();
                self.last_rendered.clear();
                self.last_rendered.push_str(&output);

                let final_action = if common > 0 {
                    let opt_backspaces = prev_chars_count - common;
                    let byte_offset = output
                        .char_indices()
                        .nth(common)
                        .map(|(i, _)| i)
                        .unwrap_or(output.len());
                    EngineAction::Replace {
                        backspaces: opt_backspaces,
                        output: output[byte_offset..].to_string(),
                    }
                } else {
                    EngineAction::Replace { backspaces, output }
                };

                (final_action, full_output_utf16)
            }
            EngineAction::Passthrough => {
                self.last_rendered.push(key.ch);
                (
                    EngineAction::Passthrough,
                    self.emitted_len + key.ch.len_utf16(),
                )
            }
            EngineAction::Consume => (EngineAction::Consume, self.emitted_len),
        };

        self.emitted_len = new_emitted_len;

        if let SyllableState::Passthrough(raw) = self.state {
            if matches!(&action, EngineAction::Replace { .. }) {
                self.raw_keys = raw.iter().map(|&c| RawKey::from(c)).collect();
            }
            self.is_passthrough = true;
        }

        action
    }

    /// Handles a backspace event: pops raw_keys until rendered length drops by 1,
    /// re-projecting the state deterministically from raw_keys.
    pub fn handle_backspace(&mut self, config: &EngineConfig) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.is_passthrough {
            self.raw_keys.pop();
            self.last_rendered.pop();
            self.emitted_len = self.last_rendered.encode_utf16().count();

            if self.raw_keys.is_empty() {
                self.clear();
            } else {
                // Check if the remaining raw keys can legitimately recover a structured Vietnamese syllable
                // that EXACTLY matches the text on screen (e.g. "tjee" backspaced to "t" matches Onset('t')).
                let eval = Self::evaluate_keys(&self.raw_keys, config);
                if eval.rendered == self.last_rendered {
                    self.raw_keys = eval.effective_raw_keys;
                    self.state = eval.state;
                    self.emitted_len = eval.emitted_len;
                    self.last_rendered = eval.rendered;
                    self.is_passthrough = matches!(self.state, SyllableState::Passthrough(_));
                } else {
                    let raw_str: String = self.raw_keys.iter().map(|k| k.ch).collect();
                    self.state = SyllableState::Passthrough(InlineList::from(raw_str.as_str()));
                }
            }
            return true;
        }

        let target_len = self.emitted_len.saturating_sub(1);
        while self.emitted_len > target_len && !self.raw_keys.is_empty() {
            self.raw_keys.pop();
            let eval = Self::evaluate_keys(&self.raw_keys, config);
            self.raw_keys = eval.effective_raw_keys;
            self.state = eval.state;
            self.emitted_len = eval.emitted_len;
            self.last_rendered = eval.rendered;
            self.is_passthrough = matches!(self.state, SyllableState::Passthrough(_));
        }

        if self.raw_keys.is_empty() {
            self.clear();
        }

        true
    }
}
