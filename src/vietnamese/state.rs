use super::{
    charset::{BaseVowel, Diacritic, Tone, decompose_vowel, is_d_stroke},
    coda::CodaState,
    modifier::{KeyEffect, match_modifier_key},
    nucleus::{ModifierOutcome, NucleusState},
    onset::OnsetState,
    spelling::{
        can_vowels_accept_coda, is_valid_coda_pair, is_valid_coda_start,
        is_valid_onset_extension,
    },
    syllable::Syllable,
    VowelLetter,
};
use crate::engine::{action::EngineAction, buffer::RawKey, config::EngineConfig};
use std::fmt;

/// State pattern: Representation of Vietnamese Syllable Parser State
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SyllableState {
    #[default]
    Empty,
    Onset(OnsetState),
    Nucleus(NucleusState),
    Coda(CodaState),
    Passthrough(String),
}

impl fmt::Display for SyllableState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyllableState::Empty => Ok(()),
            SyllableState::Onset(onset) => write!(f, "{onset}"),
            SyllableState::Nucleus(nucleus) => write!(f, "{nucleus}"),
            SyllableState::Coda(coda) => write!(f, "{coda}"),
            SyllableState::Passthrough(raw) => write!(f, "{raw}"),
        }
    }
}

impl SyllableState {
    /// Renders current state to a displayed string
    pub fn render(&self) -> String {
        self.to_string()
    }

    /// Returns a compact summary string of the state for debugging
    pub fn summary(&self) -> String {
        match self {
            SyllableState::Empty => "Empty".to_string(),
            SyllableState::Onset(onset) => format!("Onset(\"{onset}\")"),
            SyllableState::Nucleus(nucleus) => format!("Nucleus(\"{nucleus}\")"),
            SyllableState::Coda(coda) => format!("Coda(\"{coda}\")"),
            SyllableState::Passthrough(raw) => format!("Passthrough(\"{raw}\")"),
        }
    }

    /// Converts this state into a Syllable for validation, if it is a structured syllable state
    pub fn to_syllable(&self) -> Option<Syllable> {
        match self {
            SyllableState::Empty => None,
            SyllableState::Onset(onset) => Some(Syllable {
                onset: onset.chars.clone(),
                d_stroke: onset.is_d_stroke,
                vowels: Vec::new(),
                tone: Tone::None,
                coda: Vec::new(),
            }),
            SyllableState::Nucleus(nucleus) => Some(nucleus.into()),
            SyllableState::Coda(coda) => Some(coda.into()),
            SyllableState::Passthrough(_) => None,
        }
    }

    /// Parses a string into the appropriate SyllableState (Onset, Nucleus, Coda, or Passthrough)
    pub fn parse_prefix(raw: &str, config: &EngineConfig) -> SyllableState {
        let mut state = SyllableState::Empty;
        let mut current_len = 0;
        for ch in raw.chars() {
            let key = RawKey {
                ch,
                is_upper: ch.is_uppercase(),
            };
            let (next_state, action) = state.feed_key(key, current_len, config);
            state = next_state;
            match action {
                EngineAction::Replace { output, .. } => {
                    current_len = output.chars().count();
                }
                EngineAction::Passthrough => {
                    current_len += 1;
                }
                EngineAction::Consume => {}
            }
        }
        state
    }

    /// Transitions backward when Backspace is pressed:
    /// Drops the last rendered character on screen and transitions to the previous state cleanly.
    pub fn handle_backspace(self, config: &EngineConfig) -> SyllableState {
        match self {
            SyllableState::Empty => SyllableState::Empty,
            SyllableState::Onset(mut onset) => {
                onset.chars.pop();
                if onset.chars.is_empty() {
                    SyllableState::Empty
                } else {
                    SyllableState::Onset(onset)
                }
            }
            SyllableState::Nucleus(mut nucleus) => {
                let tone_pos_before = if nucleus.tone != Tone::None {
                    Some(nucleus.find_tone_position())
                } else {
                    None
                };

                nucleus.vowels.pop();

                if nucleus.vowels.is_empty() {
                    if let Some(mut onset) = nucleus.onset {
                        // If onset absorbed glide "gi" or "qu", break it back down into onset + single vowel
                        if onset.chars.len() == 2 {
                            let c0 = onset.chars[0].0.to_ascii_lowercase();
                            let c1 = onset.chars[1].0.to_ascii_lowercase();
                            if (c0 == 'g' && c1 == 'i') || (c0 == 'q' && c1 == 'u') {
                                let glide = onset.chars.pop().unwrap();
                                let base = if glide.0.eq_ignore_ascii_case(&'i') {
                                    BaseVowel::I
                                } else {
                                    BaseVowel::U
                                };
                                nucleus.onset = Some(onset);
                                nucleus.vowels = vec![VowelLetter {
                                    base,
                                    diacritic: Diacritic::None,
                                    is_upper: glide.1,
                                }];
                                return SyllableState::Nucleus(nucleus);
                            }
                        }
                        SyllableState::Onset(onset)
                    } else {
                        SyllableState::Empty
                    }
                } else {
                    // If the popped vowel carried the tone mark, clear tone
                    if let Some(pos) = tone_pos_before
                        && pos >= nucleus.vowels.len()
                    {
                        nucleus.tone = Tone::None;
                    }
                    SyllableState::Nucleus(nucleus)
                }
            }
            SyllableState::Coda(mut coda) => {
                coda.coda.pop();
                if coda.coda.is_empty() {
                    SyllableState::Nucleus(coda.nucleus)
                } else {
                    SyllableState::Coda(coda)
                }
            }
            SyllableState::Passthrough(mut raw) => {
                raw.pop();
                if raw.is_empty() {
                    SyllableState::Empty
                } else {
                    Self::parse_prefix(&raw, config)
                }
            }
        }
    }

    /// Process a new key and transit to the next state
    pub fn feed_key(
        self,
        key: RawKey,
        current_len: usize,
        config: &EngineConfig,
    ) -> (Self, EngineAction) {
        match self {
            SyllableState::Empty => Self::handle_empty(key, config),
            SyllableState::Onset(onset) => Self::handle_onset(onset, key, current_len, config),
            SyllableState::Nucleus(nucleus) => {
                Self::handle_nucleus(nucleus, key, current_len, config)
            }
            SyllableState::Coda(coda) => Self::handle_coda(coda, key, current_len, config),
            SyllableState::Passthrough(mut raw) => {
                raw.push(key.ch);
                (SyllableState::Passthrough(raw), EngineAction::Passthrough)
            }
        }
    }

    fn handle_empty(key: RawKey, config: &EngineConfig) -> (Self, EngineAction) {
        let ch_lower = key.ch.to_ascii_lowercase();

        // 1. Bracket W shortcuts: [ -> ư, ] -> ơ
        if config.bracket_w && config.method.has_bracket_shortcuts() {
            if key.ch == '[' {
                let nucleus = NucleusState {
                    onset: None,
                    vowels: vec![VowelLetter {
                        base: BaseVowel::U,
                        diacritic: Diacritic::Horn,
                        is_upper: key.is_upper,
                    }],
                    tone: Tone::None,
                };
                let output = nucleus.to_string();
                return (
                    SyllableState::Nucleus(nucleus),
                    EngineAction::Replace {
                        backspaces: 0,
                        output,
                    },
                );
            }
            if key.ch == ']' {
                let nucleus = NucleusState {
                    onset: None,
                    vowels: vec![VowelLetter {
                        base: BaseVowel::O,
                        diacritic: Diacritic::Horn,
                        is_upper: key.is_upper,
                    }],
                    tone: Tone::None,
                };
                let output = nucleus.to_string();
                return (
                    SyllableState::Nucleus(nucleus),
                    EngineAction::Replace {
                        backspaces: 0,
                        output,
                    },
                );
            }
        }

        // Standalone 'w' -> 'ư'
        if config.method.has_standalone_w() && ch_lower == 'w' {
            let nucleus = NucleusState {
                onset: None,
                vowels: vec![VowelLetter {
                    base: BaseVowel::U,
                    diacritic: Diacritic::Horn,
                    is_upper: key.is_upper,
                }],
                tone: Tone::None,
            };
            let output = nucleus.to_string();
            return (
                SyllableState::Nucleus(nucleus),
                EngineAction::Replace {
                    backspaces: 0,
                    output,
                },
            );
        }

        // If auto-uppercased first letter (key.is_upper is true, but key.ch is lowercase)
        if key.is_upper && key.ch.is_lowercase() && key.ch.is_alphabetic() {
            let upper_char = key.ch.to_uppercase().next().unwrap_or(key.ch);
            if let Some(base_vowel) = BaseVowel::from_char(key.ch) {
                let nucleus = NucleusState {
                    onset: None,
                    vowels: vec![VowelLetter {
                        base: base_vowel,
                        diacritic: Diacritic::None,
                        is_upper: true,
                    }],
                    tone: Tone::None,
                };
                return (
                    SyllableState::Nucleus(nucleus),
                    EngineAction::Replace {
                        backspaces: 0,
                        output: upper_char.to_string(),
                    },
                );
            } else {
                let onset = OnsetState {
                    chars: vec![(upper_char, true)],
                    is_d_stroke: false,
                };
                return (
                    SyllableState::Onset(onset),
                    EngineAction::Replace {
                        backspaces: 0,
                        output: upper_char.to_string(),
                    },
                );
            }
        }

        // If vowel
        if let Some((base_vowel, diacritic, tone)) = decompose_vowel(key.ch) {
            let nucleus = NucleusState {
                onset: None,
                vowels: vec![VowelLetter {
                    base: base_vowel,
                    diacritic,
                    is_upper: key.is_upper,
                }],
                tone,
            };
            return (SyllableState::Nucleus(nucleus), EngineAction::Passthrough);
        }

        // 'đ' or 'Đ'
        if is_d_stroke(key.ch) {
            let onset = OnsetState {
                chars: vec![('d', key.is_upper)],
                is_d_stroke: true,
            };
            return (SyllableState::Onset(onset), EngineAction::Passthrough);
        }

        // If consonant
        if key.ch.is_alphabetic() {
            let onset = OnsetState {
                chars: vec![(key.ch, key.is_upper)],
                is_d_stroke: false,
            };
            return (SyllableState::Onset(onset), EngineAction::Passthrough);
        }

        // Anything else -> Passthrough
        let mut raw = String::new();
        raw.push(key.ch);
        (SyllableState::Passthrough(raw), EngineAction::Passthrough)
    }

    fn handle_onset(
        mut onset: OnsetState,
        key: RawKey,
        current_len: usize,
        config: &EngineConfig,
    ) -> (Self, EngineAction) {
        let ch_lower = key.ch.to_ascii_lowercase();

        // 1. Bracket W shortcuts: [ -> ư, ] -> ơ
        if config.bracket_w && config.method.has_bracket_shortcuts() {
            if key.ch == '[' {
                let nucleus = NucleusState {
                    onset: Some(onset),
                    vowels: vec![VowelLetter {
                        base: BaseVowel::U,
                        diacritic: Diacritic::Horn,
                        is_upper: key.is_upper,
                    }],
                    tone: Tone::None,
                };
                let output = nucleus.to_string();
                return (
                    SyllableState::Nucleus(nucleus),
                    EngineAction::Replace {
                        backspaces: current_len,
                        output,
                    },
                );
            }
            if key.ch == ']' {
                let nucleus = NucleusState {
                    onset: Some(onset),
                    vowels: vec![VowelLetter {
                        base: BaseVowel::O,
                        diacritic: Diacritic::Horn,
                        is_upper: key.is_upper,
                    }],
                    tone: Tone::None,
                };
                let output = nucleus.to_string();
                return (
                    SyllableState::Nucleus(nucleus),
                    EngineAction::Replace {
                        backspaces: current_len,
                        output,
                    },
                );
            }
        }

        // 2. D-Stroke in Onset (Telex 'dd' or VNI 'd9' -> 'đ')
        let has_onset_d = !onset.chars.is_empty() && onset.chars[0].0.eq_ignore_ascii_case(&'d');
        if let Some(KeyEffect::DStroke) =
            match_modifier_key(config.method, key.ch, |_| false, has_onset_d)
        {
            if config.method.is_telex_family() && onset.is_d_stroke {
                // 3rd 'd' cancels 'đ' and restores double 'dd' into Passthrough!
                let first_d = if onset.chars[0].1 { 'D' } else { 'd' };
                let output = format!("{}{}", first_d, key.ch);
                return (
                    SyllableState::Passthrough(output.clone()),
                    EngineAction::Replace {
                        backspaces: current_len,
                        output,
                    },
                );
            } else {
                onset.is_d_stroke = !onset.is_d_stroke;
                let output = onset.to_string();
                return (
                    SyllableState::Onset(onset),
                    EngineAction::Replace {
                        backspaces: current_len,
                        output,
                    },
                );
            }
        }

        // 2. Incoming vowel -> Transition from Onset to Nucleus!
        if config.method.is_telex_family() && ch_lower == 'w' {
            // e.g. "tw" -> "tư"
            let nucleus = NucleusState {
                onset: Some(onset),
                vowels: vec![VowelLetter {
                    base: BaseVowel::U,
                    diacritic: Diacritic::Horn,
                    is_upper: key.is_upper,
                }],
                tone: Tone::None,
            };
            let output = nucleus.to_string();
            return (
                SyllableState::Nucleus(nucleus),
                EngineAction::Replace {
                    backspaces: current_len,
                    output,
                },
            );
        }

        if let Some((base_vowel, diacritic, tone)) = decompose_vowel(key.ch) {
            let prev_onset_str = onset.to_string();
            // If onset was capitalized (e.g. quick consonant "Qu", "Ph", "Gi") and this vowel is also uppercase,
            // promote entire onset to uppercase for ALL CAPS words (e.g. "QUA", "PHONG")
            if key.is_upper && !onset.chars.is_empty() && onset.chars[0].1 {
                for c in onset.chars.iter_mut() {
                    c.1 = true;
                }
            }

            let nucleus = NucleusState {
                onset: Some(onset),
                vowels: vec![VowelLetter {
                    base: base_vowel,
                    diacritic,
                    is_upper: key.is_upper,
                }],
                tone,
            };
            let output = nucleus.to_string();
            if output == format!("{}{}", prev_onset_str, key.ch) {
                return (SyllableState::Nucleus(nucleus), EngineAction::Passthrough);
            }
            return (
                SyllableState::Nucleus(nucleus),
                EngineAction::Replace {
                    backspaces: current_len,
                    output,
                },
            );
        }

        // 3. Additional consonant in Onset
        if key.ch.is_alphabetic() && is_valid_onset_extension(&onset.chars, key.ch) {
            onset.chars.push((key.ch, key.is_upper));
            return (SyllableState::Onset(onset), EngineAction::Passthrough);
        }

        // Invalid onset combo -> Passthrough
        let mut raw = onset.to_string();
        raw.push(key.ch);
        (SyllableState::Passthrough(raw), EngineAction::Passthrough)
    }

    fn handle_nucleus(
        mut nucleus: NucleusState,
        key: RawKey,
        current_len: usize,
        config: &EngineConfig,
    ) -> (Self, EngineAction) {
        let is_telex = config.method.is_telex_family();

        // 1. Bracket W shortcuts: [[ -> [, ]] -> ]
        if config.bracket_w
            && config.method.has_bracket_shortcuts()
            && nucleus.onset.is_none()
            && nucleus.vowels.len() == 1
        {
            if key.ch == '['
                && nucleus.vowels[0].base == BaseVowel::U
                && nucleus.vowels[0].diacritic == Diacritic::Horn
            {
                return (
                    SyllableState::Passthrough("[".to_string()),
                    EngineAction::Replace {
                        backspaces: current_len,
                        output: "[".to_string(),
                    },
                );
            }
            if key.ch == ']'
                && nucleus.vowels[0].base == BaseVowel::O
                && nucleus.vowels[0].diacritic == Diacritic::Horn
            {
                return (
                    SyllableState::Passthrough("]".to_string()),
                    EngineAction::Replace {
                        backspaces: current_len,
                        output: "]".to_string(),
                    },
                );
            }
        }

        // 2. Unified Modifiers (Tone, Circumflex, Horn, Breve, D-stroke, Undo Toggle)
        let has_onset_d = nucleus
            .onset
            .as_ref()
            .map(|o| !o.chars.is_empty() && o.chars[0].0.eq_ignore_ascii_case(&'d'))
            .unwrap_or(false);
        if let Some(effect) = match_modifier_key(
            config.method,
            key.ch,
            |b| nucleus.vowels.iter().any(|v| v.base == b),
            has_onset_d,
        ) {
            match nucleus.apply_modifier(effect, key, None, config) {
                ModifierOutcome::Applied => {
                    let output = nucleus.to_string();
                    return (
                        SyllableState::Nucleus(nucleus),
                        EngineAction::Replace {
                            backspaces: current_len,
                            output,
                        },
                    );
                }
                ModifierOutcome::Undone(raw) => {
                    return (
                        SyllableState::Passthrough(raw.clone()),
                        EngineAction::Replace {
                            backspaces: current_len,
                            output: raw,
                        },
                    );
                }
                ModifierOutcome::NotApplied => {}
            }
        }

        // 5. Additional vowel in Nucleus (up to 3 vowels)
        if let Some((base_vowel, mut diacritic, tone)) = decompose_vowel(key.ch) {
            // Glide absorption:
            // When onset is 'g' and single vowel is 'i' (e.g. "gi"),
            // or onset is 'q' and single vowel is 'u' (e.g. "qu"),
            // the arrival of another vowel indicates 'i' or 'u' was the consonant glide!
            // Absorb 'i' / 'u' into onset (making onset "gi" or "qu"), leaving vowels fresh for the incoming vowel!
            if nucleus.vowels.len() == 1
                && let Some(ref mut onset) = nucleus.onset
                && onset.chars.len() == 1
            {
                let c0 = onset.chars[0].0.to_ascii_lowercase();
                if c0 == 'g'
                    && nucleus.vowels[0].base == BaseVowel::I
                    && nucleus.vowels[0].diacritic == Diacritic::None
                {
                    onset.chars.push(('i', nucleus.vowels[0].is_upper));
                    nucleus.vowels.clear();
                } else if c0 == 'q'
                    && nucleus.vowels[0].base == BaseVowel::U
                    && nucleus.vowels[0].diacritic == Diacritic::None
                {
                    onset.chars.push(('u', nucleus.vowels[0].is_upper));
                    nucleus.vowels.clear();
                }
            }

            if nucleus.vowels.len() < 3 {
                // Telex automatic coupling: 'ư' + 'o' -> 'ươ'
                // In Vietnamese Telex typing, typing 'o' after 'ư' automatically couples into 'ươ'
                // e.g. "đư" + 'o' -> "đươ", "bư" + 'o' -> "bươ", "tư" + 'o' -> "tươ"
                let prev_rendered = nucleus.to_string();
                if is_telex
                    && base_vowel == BaseVowel::O
                    && diacritic == Diacritic::None
                    && nucleus.vowels.len() == 1
                    && nucleus.vowels[0].base == BaseVowel::U
                    && nucleus.vowels[0].diacritic == Diacritic::Horn
                {
                    diacritic = Diacritic::Horn;
                }

                nucleus.vowels.push(VowelLetter {
                    base: base_vowel,
                    diacritic,
                    is_upper: key.is_upper,
                });
                if tone != Tone::None && nucleus.tone == Tone::None {
                    nucleus.tone = tone;
                }

                let output = nucleus.to_string();
                if output == format!("{}{}", prev_rendered, key.ch) {
                    return (SyllableState::Nucleus(nucleus), EngineAction::Passthrough);
                } else {
                    return (
                        SyllableState::Nucleus(nucleus),
                        EngineAction::Replace {
                            backspaces: current_len,
                            output,
                        },
                    );
                }
            }
        }

        // 6. Consonant -> Transition from Nucleus to Coda!
        let can_accept_coda = can_vowels_accept_coda(&nucleus.vowels);
        if can_accept_coda && is_valid_coda_start(key.ch) {
            let coda_chars = vec![(key.ch, key.is_upper)];
            let prev_rendered = nucleus.to_string();
            let coda = CodaState {
                nucleus,
                coda: coda_chars,
            };
            let output = coda.to_string();

            if output == format!("{}{}", prev_rendered, key.ch) {
                return (SyllableState::Coda(coda), EngineAction::Passthrough);
            }

            return (
                SyllableState::Coda(coda),
                EngineAction::Replace {
                    backspaces: current_len,
                    output,
                },
            );
        }

        // Non-coda consonant or non-alphabetic -> Passthrough
        let mut raw = nucleus.to_string();
        raw.push(key.ch);
        (SyllableState::Passthrough(raw), EngineAction::Passthrough)
    }

    fn handle_coda(
        mut coda: CodaState,
        key: RawKey,
        current_len: usize,
        config: &EngineConfig,
    ) -> (Self, EngineAction) {
        // 1. Unified Modifiers across Coda (Telex & VNI)
        let has_onset_d = coda
            .nucleus
            .onset
            .as_ref()
            .map(|o| !o.chars.is_empty() && o.chars[0].0.eq_ignore_ascii_case(&'d'))
            .unwrap_or(false);
        if let Some(effect) = match_modifier_key(
            config.method,
            key.ch,
            |b| coda.nucleus.vowels.iter().any(|v| v.base == b),
            has_onset_d,
        ) {
            match coda
                .nucleus
                .apply_modifier(effect, key, Some(&coda.coda), config)
            {
                ModifierOutcome::Applied => {
                    let output = coda.to_string();
                    return (
                        SyllableState::Coda(coda),
                        EngineAction::Replace {
                            backspaces: current_len,
                            output,
                        },
                    );
                }
                ModifierOutcome::Undone(raw) => {
                    return (
                        SyllableState::Passthrough(raw.clone()),
                        EngineAction::Replace {
                            backspaces: current_len,
                            output: raw,
                        },
                    );
                }
                ModifierOutcome::NotApplied => {}
            }
        }

        // 3. Additional consonant in Coda (only "ng", "nh", "ch" are valid 2-letter codas in Vietnamese)
        if coda.coda.len() == 1 && is_valid_coda_pair(coda.coda[0].0, key.ch) {
            coda.coda.push((key.ch, key.is_upper));
            return (SyllableState::Coda(coda), EngineAction::Passthrough);
        }

        // Otherwise -> Passthrough
        let mut raw = coda.to_string();
        raw.push(key.ch);
        (SyllableState::Passthrough(raw), EngineAction::Passthrough)
    }
}
