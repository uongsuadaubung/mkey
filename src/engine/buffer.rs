use crate::vietnamese::state::SyllableState;
use super::{
    action::EngineAction,
    config::EngineConfig,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawKey {
    pub ch: char,
    pub is_upper: bool,
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

    /// Evaluates a sequence of RawKeys deterministically and returns:
    /// (resulting_state, rendered_string, emitted_length, effective_raw_keys, last_action)
    pub fn evaluate_keys(
        keys: &[RawKey],
        config: &EngineConfig,
    ) -> (SyllableState, String, usize, Vec<RawKey>, EngineAction) {
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
                        effective = s
                            .chars()
                            .map(|c| RawKey {
                                ch: c,
                                is_upper: c.is_uppercase(),
                            })
                            .collect();
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
        (state, rendered, current_len, effective, last_action)
    }

    /// Checks whether `key` marks a word boundary (e.g. CamelCase / PascalCase boundary).
    pub fn is_boundary(&self, key: &RawKey) -> bool {
        if self.raw_keys.is_empty() {
            return false;
        }

        key.is_upper && self.raw_keys.iter().any(|k| !k.is_upper && k.ch.is_alphabetic())
    }

    /// Feeds a new key into the state machine and returns the resulting EngineAction.
    pub fn feed_key(&mut self, key: RawKey, config: &EngineConfig) -> EngineAction {
        if self.is_boundary(&key) {
            self.clear();
        }

        self.raw_keys.push(key);

        let current_state = std::mem::take(&mut self.state);
        let (next_state, action) = current_state.feed_key(key, self.emitted_len, config);
        self.state = next_state;

        match &action {
            EngineAction::Replace { output, .. } => {
                self.emitted_len = output.encode_utf16().count();
            }
            EngineAction::Passthrough => {
                self.emitted_len += key.ch.len_utf16();
            }
            EngineAction::Consume => {}
        }

        if matches!(&action, EngineAction::Replace { .. }) {
            if let SyllableState::Passthrough(ref s) = self.state {
                self.raw_keys = s
                    .chars()
                    .map(|c| RawKey {
                        ch: c,
                        is_upper: c.is_uppercase(),
                    })
                    .collect();
                self.is_passthrough = true;
            }
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
            let (new_state, _new_rendered, new_len, effective, _) =
                Self::evaluate_keys(&self.raw_keys, config);
            self.raw_keys = effective;
            self.state = new_state;
            self.emitted_len = new_len;
            self.is_passthrough = matches!(self.state, SyllableState::Passthrough(_));
        }

        if self.raw_keys.is_empty() {
            self.clear();
        }

        true
    }
}

