use super::{action::EngineAction, config::EngineConfig};
use crate::vietnamese::state::SyllableState;

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
    /// Whether this buffer is in raw passthrough mode (e.g. restored from spelling correction)
    pub is_passthrough: bool,
}

impl TypingBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.raw_keys.clear();
        self.state = SyllableState::Empty;
        self.emitted_len = 0;
        self.is_passthrough = false;
    }

    pub fn restore_as_passthrough(&mut self, keys: Vec<RawKey>) {
        let text: String = keys.iter().map(|k| k.ch).collect();
        self.raw_keys = keys;
        self.emitted_len = text.encode_utf16().count();
        self.state = SyllableState::Passthrough(text);
        self.is_passthrough = true;
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

        for &key in keys {
            effective.push(key);
            let (next_state, action) = state.feed_key(key, current_len, config);
            state = next_state;
            match &action {
                EngineAction::Replace { output, .. } => {
                    current_len = output.encode_utf16().count();
                    // If an undo toggle transitioned into Passthrough (e.g. toanss -> toans, chuww -> chuw),
                    // synchronize effective keys to match the explicit cancelled output.
                    if let SyllableState::Passthrough(ref s) = state {
                        effective = s.chars().map(RawKey::from).collect();
                    }
                }
                EngineAction::Passthrough => {
                    current_len += key.ch.len_utf16();
                }
                EngineAction::Consume => {}
            }
            last_action = action;
        }

        let rendered = state.render();
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

        let prev_rendered = self.state.render();
        self.raw_keys.push(key);

        let current_state = std::mem::take(&mut self.state);
        let (next_state, action) = current_state.feed_key(key, self.emitted_len, config);
        self.state = next_state;

        // Common Prefix Optimization:
        // If the unchanged prefix of the word (e.g. onset 'g' in "go" -> "gõ") remains identical,
        // do not backspace and re-emit it. Only backspace the changed suffix ("o") and emit ("õ").
        // This eliminates duplicate characters ("ggõ") on browser address bars (Firefox, Chrome).
        let action = match action {
            EngineAction::Replace { backspaces, output } => {
                let prev_chars: Vec<char> = prev_rendered.chars().collect();
                let new_chars: Vec<char> = output.chars().collect();

                let mut common = 0;
                while common < prev_chars.len()
                    && common < new_chars.len()
                    && prev_chars[common] == new_chars[common]
                {
                    common += 1;
                }

                if common > 0 {
                    let opt_backspaces = prev_chars.len() - common;
                    let opt_output: String = new_chars[common..].iter().collect();
                    EngineAction::Replace {
                        backspaces: opt_backspaces,
                        output: opt_output,
                    }
                } else {
                    EngineAction::Replace { backspaces, output }
                }
            }
            other => other,
        };

        match &action {
            EngineAction::Replace { .. } => {
                self.emitted_len = self.state.render().encode_utf16().count();
            }
            EngineAction::Passthrough => {
                self.emitted_len += key.ch.len_utf16();
            }
            EngineAction::Consume => {}
        }

        if matches!(&action, EngineAction::Replace { .. })
            && let SyllableState::Passthrough(ref s) = self.state
        {
            self.raw_keys = s.chars().map(RawKey::from).collect();
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

        let target_len = self.emitted_len.saturating_sub(1);
        while self.emitted_len > target_len && !self.raw_keys.is_empty() {
            self.raw_keys.pop();
            let eval = Self::evaluate_keys(&self.raw_keys, config);
            self.raw_keys = eval.effective_raw_keys;
            self.state = eval.state;
            self.emitted_len = eval.emitted_len;
            self.is_passthrough = matches!(self.state, SyllableState::Passthrough(_));
        }

        if self.raw_keys.is_empty() {
            self.clear();
        }

        true
    }
}
