use crate::engine::config::InputMethod;
use super::charset::{BaseVowel, Tone};

/// Abstract linguistic modifier action in the Vietnamese orthography.
///
/// Any Vietnamese typing method (Telex, Simple Telex, VNI, VIQR, etc.)
/// maps keyboard inputs to these orthographic actions:
/// 1. Tones (Thanh điệu: Sắc, Huyền, Hỏi, Ngã, Nặng, Bỏ dấu)
/// 2. Circumflex (Dấu mũ: â, ê, ô)
/// 3. Horn (Dấu móc: ư, ơ)
/// 4. Breve (Dấu trăng: ă)
/// 5. D-Stroke (Gạch ngang: đ)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEffect {
    /// Applies or toggles a tone mark (Acute, Grave, HookAbove, Tilde, DotBelow)
    Tone(Tone),
    /// Removes tone or diacritics (Telex 'z', VNI '0')
    RemoveTone,
    /// Adds or toggles a circumflex mark (â, ê, ô)
    /// - `Some(v)` targets a specific vowel (Telex aa, ee, oo)
    /// - `None` targets any eligible vowel in the syllable (VNI 6)
    Circumflex(Option<BaseVowel>),
    /// Adds or toggles horn mark on u/o (VNI 7)
    Horn,
    /// Adds or toggles breve mark on a (VNI 8)
    Breve,
    /// Adds or toggles horn on u/o or breve on a (Telex 'w')
    HornShortcut,
    /// Adds or toggles d-stroke: d -> đ (Telex dd, VNI d9)
    DStroke,
}

/// Matches a key character to its modifier effect based on the active input method
/// and syllable context (present vowels, onset 'd').
pub fn match_modifier_key(
    method: InputMethod,
    ch: char,
    has_vowel: impl Fn(BaseVowel) -> bool,
    has_onset_d: bool,
) -> Option<KeyEffect> {
    let ch_lower = ch.to_ascii_lowercase();

    if method.is_telex_family() {
        match ch_lower {
            's' => Some(KeyEffect::Tone(Tone::Acute)),
            'f' => Some(KeyEffect::Tone(Tone::Grave)),
            'r' => Some(KeyEffect::Tone(Tone::HookAbove)),
            'x' => Some(KeyEffect::Tone(Tone::Tilde)),
            'j' => Some(KeyEffect::Tone(Tone::DotBelow)),
            'z' => Some(KeyEffect::RemoveTone),
            'w' => Some(KeyEffect::HornShortcut),
            'a' if has_vowel(BaseVowel::A) => Some(KeyEffect::Circumflex(Some(BaseVowel::A))),
            'e' if has_vowel(BaseVowel::E) => Some(KeyEffect::Circumflex(Some(BaseVowel::E))),
            'o' if has_vowel(BaseVowel::O) => Some(KeyEffect::Circumflex(Some(BaseVowel::O))),
            'd' if has_onset_d => Some(KeyEffect::DStroke),
            _ => None,
        }
    } else if method.is_vni() {
        match ch_lower {
            '1' => Some(KeyEffect::Tone(Tone::Acute)),
            '2' => Some(KeyEffect::Tone(Tone::Grave)),
            '3' => Some(KeyEffect::Tone(Tone::HookAbove)),
            '4' => Some(KeyEffect::Tone(Tone::Tilde)),
            '5' => Some(KeyEffect::Tone(Tone::DotBelow)),
            '0' => Some(KeyEffect::RemoveTone),
            '6' => Some(KeyEffect::Circumflex(None)),
            '7' => Some(KeyEffect::Horn),
            '8' => Some(KeyEffect::Breve),
            '9' if has_onset_d => Some(KeyEffect::DStroke),
            _ => None,
        }
    } else {
        None
    }
}
