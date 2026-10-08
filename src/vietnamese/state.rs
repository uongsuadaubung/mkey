use super::{
    charset::{compose_d, compose_vowel, decompose_vowel, is_d_stroke, BaseVowel, Diacritic, Tone},
    modifier::{match_modifier_key, KeyEffect},
    spelling::{
        can_vowels_accept_coda, is_valid_coda_pair, is_valid_coda_start, is_valid_onset_extension,
    },
    syllable::Syllable,
    VowelLetter,
};
use crate::engine::{
    action::EngineAction,
    buffer::RawKey,
    config::EngineConfig,
};

/// Onset (Phụ âm đầu)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OnsetState {
    pub chars: Vec<(char, bool)>, // (char, is_upper)
    pub is_d_stroke: bool,        // 'đ' / 'Đ'
}

impl OnsetState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for &(c, is_upper) in &self.chars {
            if c.to_ascii_lowercase() == 'd' {
                s.push(compose_d(self.is_d_stroke, is_upper));
            } else if is_upper {
                s.extend(c.to_uppercase());
            } else {
                s.extend(c.to_lowercase());
            }
        }
        s
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }
}

/// Nucleus (Nguyên âm & Dấu thanh)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NucleusState {
    pub onset: Option<OnsetState>,
    pub vowels: Vec<VowelLetter>,
    pub tone: Tone,
}

impl NucleusState {
    pub fn find_tone_position(&self) -> usize {
        let n = self.vowels.len();
        if n <= 1 {
            return 0;
        }

        if n == 2 {
            let v0 = &self.vowels[0];
            let v1 = &self.vowels[1];

            // If ươ (both have horn) -> tone on ơ (index 1)
            if v0.base == BaseVowel::U && v0.diacritic == Diacritic::Horn
                && v1.base == BaseVowel::O && v1.diacritic == Diacritic::Horn
            {
                return 1;
            }

            // In any diphthong, if second vowel has a diacritic (iê, yê, uô, uê, oă, uơ) -> index 1!
            if v1.diacritic != Diacritic::None {
                return 1;
            }

            // If first vowel has a diacritic (ưa, âu, ây, êu, ưu, ơi, ưi) -> index 0!
            if v0.diacritic != Diacritic::None {
                return 0;
            }

            // Neither vowel has a diacritic:
            // "oa", "oe", "uy": in modern orthography, tone is always placed on the main vowel (index 1: hòa, khóe, thủy)
            if (v0.base == BaseVowel::O && (v1.base == BaseVowel::A || v1.base == BaseVowel::E))
                || (v0.base == BaseVowel::U && v1.base == BaseVowel::Y)
            {
                return 1;
            }

            // "ua", "ia", "ya" -> tone on first vowel
            if v1.base == BaseVowel::A {
                return 0;
            }

            // "ai", "oi", "ui", "ay", "ao", "eo", "au", "iu" -> tone on first vowel
            return 0;
        }

        if n == 3 {
            // Triphthongs: tone on middle vowel (index 1)
            return 1;
        }

        0
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        if let Some(ref onset) = self.onset {
            s.push_str(&onset.to_string());
        }

        let tone_pos = self.find_tone_position();
        for (i, v) in self.vowels.iter().enumerate() {
            let tone_for_vowel = if i == tone_pos { self.tone } else { Tone::None };
            s.push(compose_vowel(v.base, v.diacritic, tone_for_vowel, v.is_upper));
        }

        s
    }

    /// Converts this Nucleus into a standard Syllable for validation/inspection
    pub fn to_syllable(&self) -> Syllable {
        Syllable {
            onset: self.onset.as_ref().map(|o| o.chars.clone()).unwrap_or_default(),
            d_stroke: self.onset.as_ref().map(|o| o.is_d_stroke).unwrap_or(false),
            vowels: self.vowels.clone(),
            tone: self.tone,
            coda: Vec::new(),
        }
    }

    /// Free mark 'd' stroke toggle: toggles onset 'd' <-> 'đ'
    pub fn toggle_d_stroke(&mut self) -> bool {
        if let Some(ref mut onset) = self.onset {
            if !onset.chars.is_empty() && onset.chars[0].0.to_ascii_lowercase() == 'd' {
                onset.is_d_stroke = !onset.is_d_stroke;
                return true;
            }
        }
        false
    }

    /// Applies horn (or breve for 'a') to eligible vowels according to Vietnamese phonotactics:
    /// - Single: a->ă, o->ơ, u->ư
    /// - Diphthong "uo" -> "ươ" (both get horn: lươn, được, nước)
    /// - Diphthong "ua" -> "ưa" (only 'u' gets horn: mưa, chưa, xưa)
    /// - Diphthong "oa" -> "oă" (only 'a' gets breve: hoặc, khoăn)
    /// - Diphthongs "ui", "oi" -> "ưi", "ơi" (first vowel gets horn: gửi, chơi)
    /// - Diphthong "iu" -> "ưu" (second vowel gets horn: cừu, lựu)
    /// - Triphthong "uoi", "uou" -> "ươi", "ươu" (u and o get horn)
    pub fn apply_horn(&mut self) -> bool {
        match self.vowels.len() {
            0 => false,
            1 => {
                let v = &mut self.vowels[0];
                match v.base {
                    BaseVowel::A if v.diacritic == Diacritic::None => {
                        v.diacritic = Diacritic::Breve;
                        true
                    }
                    BaseVowel::O if v.diacritic == Diacritic::None => {
                        v.diacritic = Diacritic::Horn;
                        true
                    }
                    BaseVowel::U if v.diacritic == Diacritic::None => {
                        v.diacritic = Diacritic::Horn;
                        true
                    }
                    _ => false,
                }
            }
            2 => {
                let (b0, d0) = (self.vowels[0].base, self.vowels[0].diacritic);
                let (b1, d1) = (self.vowels[1].base, self.vowels[1].diacritic);
                match (b0, b1) {
                    // "uo" -> both get horn: "ươ"
                    (BaseVowel::U, BaseVowel::O) if d0 == Diacritic::None || d1 == Diacritic::None => {
                        self.vowels[0].diacritic = Diacritic::Horn;
                        self.vowels[1].diacritic = Diacritic::Horn;
                        true
                    }
                    // "ua" -> only 'u' gets horn: "ưa" (mưa, chưa)
                    (BaseVowel::U, BaseVowel::A) if d0 == Diacritic::None => {
                        self.vowels[0].diacritic = Diacritic::Horn;
                        true
                    }
                    // "oa" -> only 'a' gets breve: "oă" (hoặc, khoăn)
                    (BaseVowel::O, BaseVowel::A) if d1 == Diacritic::None => {
                        self.vowels[1].diacritic = Diacritic::Breve;
                        true
                    }
                    // "ui" -> only 'u' gets horn: "ưi" (gửi, ngửi)
                    (BaseVowel::U, BaseVowel::I) if d0 == Diacritic::None => {
                        self.vowels[0].diacritic = Diacritic::Horn;
                        true
                    }
                    // "oi" -> only 'o' gets horn: "ơi" (chơi, nơi)
                    (BaseVowel::O, BaseVowel::I) if d0 == Diacritic::None => {
                        self.vowels[0].diacritic = Diacritic::Horn;
                        true
                    }
                    // "iu" -> only 'u' gets horn: "ưu" (cừu, lựu)
                    (BaseVowel::I, BaseVowel::U) if d1 == Diacritic::None => {
                        self.vowels[1].diacritic = Diacritic::Horn;
                        true
                    }
                    _ => false,
                }
            }
            3 => {
                // Triphthong e.g. "uoi" -> "ươi", "uou" -> "ươu"
                if self.vowels[0].base == BaseVowel::U && self.vowels[1].base == BaseVowel::O {
                    self.vowels[0].diacritic = Diacritic::Horn;
                    self.vowels[1].diacritic = Diacritic::Horn;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Reverts horn (or breve for 'a') back to plain vowels (undo toggle)
    pub fn revert_horn(&mut self) -> bool {
        let mut reverted = false;
        for v in self.vowels.iter_mut() {
            if matches!(v.diacritic, Diacritic::Horn | Diacritic::Breve) {
                v.diacritic = Diacritic::None;
                reverted = true;
            }
        }
        reverted
    }

    /// Toggle circumflex (aa->â, ee->ê, oo->ô) with reverse scanning across vowels
    pub fn toggle_circumflex(&mut self, base: BaseVowel) -> bool {
        if let Some(v) = self.vowels.iter_mut().rev().find(|v| v.base == base) {
            v.diacritic = if v.diacritic == Diacritic::Circumflex {
                Diacritic::None
            } else {
                Diacritic::Circumflex
            };
            true
        } else {
            false
        }
    }

    /// Unified modifier application (Tone, Circumflex, Horn, Breve, D-stroke, Undo Toggle).
    /// Used by both NucleusState and CodaState across all input methods (Telex, Simple Telex, VNI).
    pub fn apply_modifier(
        &mut self,
        effect: KeyEffect,
        key: RawKey,
        coda: Option<&[(char, bool)]>,
        config: &EngineConfig,
    ) -> ModifierOutcome {
        let make_raw = |n: &NucleusState| -> String {
            let mut s = n.to_string();
            if let Some(coda_chars) = coda {
                for &(c, is_upper) in coda_chars {
                    if is_upper {
                        s.extend(c.to_uppercase());
                    } else {
                        s.extend(c.to_lowercase());
                    }
                }
            }
            s.push(key.ch);
            s
        };

        match effect {
            KeyEffect::Circumflex(target) => {
                if let Some(coda_chars) = coda {
                    let coda_str: String = coda_chars.iter().map(|&(c, _)| c.to_ascii_lowercase()).collect();
                    // In Vietnamese, multi-vowel diphthongs like "ie" -> "iê", "uo" -> "uô" always allow circumflex
                    // even before tone is typed (e.g. "tiep" + 'e' -> "tiêp" + 's' -> "tiếp").
                    // For single vowels with stop codas (c, ch, p, t) and no tone, incoming vowels are English
                    // continuations (e.g. "data", "delete", "compete"), so avoid applying circumflex.
                    if self.vowels.len() <= 1 && matches!(coda_str.as_str(), "c" | "ch" | "p" | "t") && self.tone == Tone::None {
                        return ModifierOutcome::NotApplied;
                    }
                }

                match target {
                    Some(target_base) => {
                        // Double vowel circumflex (Telex: aa -> â, ee -> ê, oo -> ô)
                        if let Some(pos) = self.vowels.iter().rposition(|v| v.base == target_base && v.diacritic == Diacritic::Circumflex) {
                            let prev_v = self.vowels.remove(pos);
                            self.vowels.insert(pos, VowelLetter {
                                base: target_base,
                                diacritic: Diacritic::None,
                                is_upper: prev_v.is_upper,
                            });
                            ModifierOutcome::Undone(make_raw(self))
                        } else if self.toggle_circumflex(target_base) {
                            ModifierOutcome::Applied
                        } else {
                            ModifierOutcome::NotApplied
                        }
                    }
                    None => {
                        // Generic circumflex (VNI: 6)
                        if self.toggle_circumflex(BaseVowel::A)
                            || self.toggle_circumflex(BaseVowel::E)
                            || self.toggle_circumflex(BaseVowel::O)
                        {
                            ModifierOutcome::Applied
                        } else {
                            ModifierOutcome::NotApplied
                        }
                    }
                }
            }

            KeyEffect::DStroke => {
                if let Some(ref mut onset) = self.onset {
                    if !onset.chars.is_empty() && onset.chars[0].0.to_ascii_lowercase() == 'd' {
                        if config.method.is_telex_family() && onset.is_d_stroke {
                            // Telex undo toggle: 3rd 'd' cancels 'đ' into 'dd'
                            onset.is_d_stroke = false;
                            ModifierOutcome::Undone(make_raw(self))
                        } else {
                            onset.is_d_stroke = !onset.is_d_stroke;
                            ModifierOutcome::Applied
                        }
                    } else {
                        ModifierOutcome::NotApplied
                    }
                } else {
                    ModifierOutcome::NotApplied
                }
            }

            KeyEffect::HornShortcut => {
                // Standalone 'w' undo: ww -> w
                if coda.is_none()
                    && self.onset.is_none()
                    && self.vowels.len() == 1
                    && self.vowels[0].base == BaseVowel::U
                    && self.vowels[0].diacritic == Diacritic::Horn
                {
                    let raw_w = if key.is_upper { "W" } else { "w" }.to_string();
                    return ModifierOutcome::Undone(raw_w);
                }

                if self.apply_horn() {
                    ModifierOutcome::Applied
                } else if self.revert_horn() {
                    ModifierOutcome::Undone(make_raw(self))
                } else {
                    ModifierOutcome::NotApplied
                }
            }

            KeyEffect::Horn => {
                if self.apply_horn() {
                    ModifierOutcome::Applied
                } else {
                    ModifierOutcome::NotApplied
                }
            }

            KeyEffect::Breve => {
                if let Some(v) = self.vowels.iter_mut().rev().find(|v| v.base == BaseVowel::A) {
                    v.diacritic = if v.diacritic == Diacritic::Breve {
                        Diacritic::None
                    } else {
                        Diacritic::Breve
                    };
                    ModifierOutcome::Applied
                } else {
                    ModifierOutcome::NotApplied
                }
            }

            KeyEffect::Tone(tone) => {
                if self.tone == tone {
                    if config.method.is_telex_family() {
                        self.tone = Tone::None;
                        ModifierOutcome::Undone(make_raw(self))
                    } else {
                        self.tone = Tone::None;
                        ModifierOutcome::Applied
                    }
                } else {
                    self.tone = tone;
                    if let Some(coda_chars) = coda {
                        let coda_str: String = coda_chars.iter().map(|&(c, _)| c.to_ascii_lowercase()).collect();
                        if matches!(coda_str.as_str(), "c" | "ch" | "p" | "t")
                            && !matches!(self.tone, Tone::Acute | Tone::DotBelow)
                        {
                            self.tone = Tone::None;
                            return ModifierOutcome::NotApplied;
                        }
                    }
                    ModifierOutcome::Applied
                }
            }

            KeyEffect::RemoveTone => {
                if self.tone != Tone::None {
                    self.tone = Tone::None;
                    ModifierOutcome::Applied
                } else if config.method.is_telex_family() {
                    let mut changed = false;
                    for v in self.vowels.iter_mut() {
                        if v.diacritic != Diacritic::None {
                            v.diacritic = Diacritic::None;
                            changed = true;
                        }
                    }
                    if let Some(ref mut onset) = self.onset {
                        if onset.is_d_stroke {
                            onset.is_d_stroke = false;
                            changed = true;
                        }
                    }
                    if changed {
                        ModifierOutcome::Applied
                    } else {
                        ModifierOutcome::NotApplied
                    }
                } else {
                    ModifierOutcome::NotApplied
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifierOutcome {
    Applied,
    Undone(String),
    NotApplied,
}

/// Coda (Phụ âm cuối)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodaState {
    pub nucleus: NucleusState,
    pub coda: Vec<(char, bool)>,
}

impl CodaState {
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        if let Some(ref onset) = self.nucleus.onset {
            s.push_str(&onset.to_string());
        }

        // When there is a coda, tone is placed according to coda rules:
        // Single vowel: index 0.
        // Triphthong with final diacritic (e.g. uyên, uyết): index 2.
        // Otherwise (diphthongs or general triphthongs): index 1.
        let n = self.nucleus.vowels.len();
        let tone_pos = if n <= 1 {
            0
        } else if n == 3 && self.nucleus.vowels[2].diacritic != Diacritic::None {
            2
        } else {
            1
        };

        for (i, v) in self.nucleus.vowels.iter().enumerate() {
            let tone_for_vowel = if i == tone_pos {
                self.nucleus.tone
            } else {
                Tone::None
            };
            s.push(compose_vowel(v.base, v.diacritic, tone_for_vowel, v.is_upper));
        }

        for &(c, is_upper) in &self.coda {
            if is_upper {
                s.extend(c.to_uppercase());
            } else {
                s.extend(c.to_lowercase());
            }
        }

        s
    }

    pub fn to_syllable(&self) -> Syllable {
        Syllable {
            onset: self.nucleus.onset.as_ref().map(|o| o.chars.clone()).unwrap_or_default(),
            d_stroke: self.nucleus.onset.as_ref().map(|o| o.is_d_stroke).unwrap_or(false),
            vowels: self.nucleus.vowels.clone(),
            tone: self.nucleus.tone,
            coda: self.coda.clone(),
        }
    }
}

/// State pattern: Representation of Vietnamese Syllable Parser State
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyllableState {
    Empty,
    Onset(OnsetState),
    Nucleus(NucleusState),
    Coda(CodaState),
    Passthrough(String),
}

impl Default for SyllableState {
    fn default() -> Self {
        SyllableState::Empty
    }
}

impl SyllableState {
    /// Renders current state to a displayed string
    pub fn render(&self) -> String {
        match self {
            SyllableState::Empty => String::new(),
            SyllableState::Onset(onset) => onset.to_string(),
            SyllableState::Nucleus(nucleus) => nucleus.to_string(),
            SyllableState::Coda(coda) => coda.to_string(),
            SyllableState::Passthrough(raw) => raw.clone(),
        }
    }

    /// Returns a compact summary string of the state for debugging
    pub fn summary(&self) -> String {
        match self {
            SyllableState::Empty => "Empty".to_string(),
            SyllableState::Onset(onset) => format!("Onset(\"{}\")", onset.to_string()),
            SyllableState::Nucleus(nucleus) => format!("Nucleus(\"{}\")", nucleus.to_string()),
            SyllableState::Coda(coda) => format!("Coda(\"{}\")", coda.to_string()),
            SyllableState::Passthrough(raw) => format!("Passthrough(\"{}\")", raw),
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
            SyllableState::Nucleus(nucleus) => Some(nucleus.to_syllable()),
            SyllableState::Coda(coda) => Some(coda.to_syllable()),
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
                                let base = if glide.0.to_ascii_lowercase() == 'i' {
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
                    if let Some(pos) = tone_pos_before {
                        if pos >= nucleus.vowels.len() {
                            nucleus.tone = Tone::None;
                        }
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
            SyllableState::Nucleus(nucleus) => Self::handle_nucleus(nucleus, key, current_len, config),
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

        // 2. Quick start consonants: f -> ph, j -> gi, w -> qu
        if config.quick_start_consonant {
            match ch_lower {
                'f' => {
                    let onset = OnsetState {
                        chars: vec![('p', key.is_upper), ('h', false)],
                        is_d_stroke: false,
                    };
                    let output = onset.to_string();
                    return (
                        SyllableState::Onset(onset),
                        EngineAction::Replace {
                            backspaces: 0,
                            output,
                        },
                    );
                }
                'j' => {
                    let onset = OnsetState {
                        chars: vec![('g', key.is_upper), ('i', false)],
                        is_d_stroke: false,
                    };
                    let output = onset.to_string();
                    return (
                        SyllableState::Onset(onset),
                        EngineAction::Replace {
                            backspaces: 0,
                            output,
                        },
                    );
                }
                'w' => {
                    let onset = OnsetState {
                        chars: vec![('q', key.is_upper), ('u', false)],
                        is_d_stroke: false,
                    };
                    let output = onset.to_string();
                    return (
                        SyllableState::Onset(onset),
                        EngineAction::Replace {
                            backspaces: 0,
                            output,
                        },
                    );
                }
                _ => {}
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
        let has_onset_d = !onset.chars.is_empty() && onset.chars[0].0.to_ascii_lowercase() == 'd';
        if let Some(KeyEffect::DStroke) = match_modifier_key(config.method, key.ch, |_| false, has_onset_d) {
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
        let ch_lower = key.ch.to_ascii_lowercase();
        let is_telex = config.method.is_telex_family();
 
        // 1. Bracket W shortcuts: [[ -> [, ]] -> ]
        if config.bracket_w && config.method.has_bracket_shortcuts() && nucleus.onset.is_none() && nucleus.vowels.len() == 1 {
            if key.ch == '[' && nucleus.vowels[0].base == BaseVowel::U && nucleus.vowels[0].diacritic == Diacritic::Horn {
                return (
                    SyllableState::Passthrough("[".to_string()),
                    EngineAction::Replace {
                        backspaces: current_len,
                        output: "[".to_string(),
                    },
                );
            }
            if key.ch == ']' && nucleus.vowels[0].base == BaseVowel::O && nucleus.vowels[0].diacritic == Diacritic::Horn {
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
        let has_onset_d = nucleus.onset.as_ref().map(|o| !o.chars.is_empty() && o.chars[0].0.to_ascii_lowercase() == 'd').unwrap_or(false);
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
            if nucleus.vowels.len() == 1 {
                if let Some(ref mut onset) = nucleus.onset {
                    if onset.chars.len() == 1 {
                        let c0 = onset.chars[0].0.to_ascii_lowercase();
                        if c0 == 'g' && nucleus.vowels[0].base == BaseVowel::I && nucleus.vowels[0].diacritic == Diacritic::None {
                            onset.chars.push(('i', nucleus.vowels[0].is_upper));
                            nucleus.vowels.clear();
                        } else if c0 == 'q' && nucleus.vowels[0].base == BaseVowel::U && nucleus.vowels[0].diacritic == Diacritic::None {
                            onset.chars.push(('u', nucleus.vowels[0].is_upper));
                            nucleus.vowels.clear();
                        }
                    }
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
        if can_accept_coda && is_valid_coda_start(key.ch, config.quick_end_consonant) {
            let mut coda_chars = vec![(key.ch, key.is_upper)];
            let mut did_quick_end = false;

            if config.quick_end_consonant {
                match ch_lower {
                    'g' => {
                        coda_chars = vec![('n', key.is_upper), ('g', key.is_upper)];
                        did_quick_end = true;
                    }
                    'h' => {
                        coda_chars = vec![('n', key.is_upper), ('h', key.is_upper)];
                        did_quick_end = true;
                    }
                    'k' => {
                        coda_chars = vec![('c', key.is_upper), ('h', key.is_upper)];
                        did_quick_end = true;
                    }
                    _ => {}
                }
            }

            let prev_rendered = nucleus.to_string();
            let coda = CodaState {
                nucleus,
                coda: coda_chars,
            };
            let output = coda.to_string();

            if !did_quick_end && output == format!("{}{}", prev_rendered, key.ch) {
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
        let has_onset_d = coda.nucleus.onset.as_ref().map(|o| !o.chars.is_empty() && o.chars[0].0.to_ascii_lowercase() == 'd').unwrap_or(false);
        if let Some(effect) = match_modifier_key(
            config.method,
            key.ch,
            |b| coda.nucleus.vowels.iter().any(|v| v.base == b),
            has_onset_d,
        ) {
            match coda.nucleus.apply_modifier(effect, key, Some(&coda.coda), config) {
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

        // 2. Quick end consonant undo: if user types the same quick key again, revert to single consonant
        // e.g. "lo" + 'g' -> "long", then typing 'g' again -> "log"
        if config.quick_end_consonant {
            let ch_l = key.ch.to_ascii_lowercase();
            if (ch_l == 'g' && coda.coda.len() == 2 && coda.coda[0].0.to_ascii_lowercase() == 'n' && coda.coda[1].0.to_ascii_lowercase() == 'g')
                || (ch_l == 'h' && coda.coda.len() == 2 && coda.coda[0].0.to_ascii_lowercase() == 'n' && coda.coda[1].0.to_ascii_lowercase() == 'h')
                || (ch_l == 'k' && coda.coda.len() == 2 && coda.coda[0].0.to_ascii_lowercase() == 'c' && coda.coda[1].0.to_ascii_lowercase() == 'h')
            {
                coda.coda = vec![(key.ch, key.is_upper)];
                let output = coda.to_string();
                return (
                    SyllableState::Coda(coda),
                    EngineAction::Replace {
                        backspaces: current_len,
                        output,
                    },
                );
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
