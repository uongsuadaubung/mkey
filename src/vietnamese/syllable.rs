use super::charset::{BaseVowel, Diacritic, Tone, compose_d, compose_vowel};
use std::fmt;

/// Represents a single character in the syllable with its case preserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VowelLetter {
    pub base: BaseVowel,
    pub diacritic: Diacritic,
    pub is_upper: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Syllable {
    /// Phụ âm đầu (e.g. "t", "th", "ngh", "d", "đ")
    pub onset: Vec<(char, bool)>, // (char, is_upper)
    /// D có dấu gạch ngang (đ / Đ) hay không
    pub d_stroke: bool,
    /// Các nguyên âm chính (Nucleus: 1 to 3 vowels)
    pub vowels: Vec<VowelLetter>,
    /// Dấu thanh chung của âm tiết
    pub tone: Tone,
    /// Phụ âm cuối (e.g. "ng", "nh", "ch", "t", "c", "m", "n", "p")
    pub coda: Vec<(char, bool)>,
}

impl Syllable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.onset.is_empty() && self.vowels.is_empty() && self.coda.is_empty()
    }

    /// Determines which vowel index in `vowels` should receive the tone mark.
    /// In modern Vietnamese orthography (QĐ 1980 / SGK hiện đại):
    /// Tone mark is placed on the nucleus vowel (hòa, thúy, khỏe).
    pub fn find_tone_position(&self) -> usize {
        let n = self.vowels.len();
        if n <= 1 {
            return 0;
        }

        // 1. If there is a coda (e.g. "tiếng", "toán"), tone usually goes on the main/last vowel before coda.
        if !self.coda.is_empty() {
            // Exceptions: "qu", "gi" where 'u' or 'i' acts like semi-vowel
            if n == 2 {
                return 1;
            }
            if n == 3 {
                return 1; // e.g. "nguyên" -> 'ê'
            }
            return n - 1;
        }

        // 2. Open syllable (no coda, e.g. "họa", "thúy", "tôi", "mía")
        if n == 2 {
            let v0 = self.vowels[0].base;
            let v1 = self.vowels[1].base;

            // "oa", "oe", "uy": tone on first vowel (hòa, khóe, thủy, thúy)
            if (v0 == BaseVowel::O && (v1 == BaseVowel::A || v1 == BaseVowel::E))
                || (v0 == BaseVowel::U && v1 == BaseVowel::Y)
            {
                return 0;
            }

            // "ua", "ưa", "ia", "ya" -> tone on first vowel (múa, cứa, mía)
            if v1 == BaseVowel::A {
                return 0;
            }

            // Default: tone on first vowel for open diphthongs (tối, nói, bùi...)
            return 0;
        }

        if n == 3 {
            // e.g. "ngoại", "khuỷu"
            return 1;
        }

        0
    }
}

impl fmt::Display for Syllable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 1. Onset
        for &(c, is_upper) in &self.onset {
            if c.eq_ignore_ascii_case(&'d') {
                write!(f, "{}", compose_d(self.d_stroke, is_upper))?;
            } else if is_upper {
                for u in c.to_uppercase() {
                    write!(f, "{u}")?;
                }
            } else {
                for l in c.to_lowercase() {
                    write!(f, "{l}")?;
                }
            }
        }

        // 2. Nucleus (Vowels)
        let tone_pos = self.find_tone_position();
        for (i, v) in self.vowels.iter().enumerate() {
            let tone_for_vowel = if i == tone_pos { self.tone } else { Tone::None };
            write!(
                f,
                "{}",
                compose_vowel(v.base, v.diacritic, tone_for_vowel, v.is_upper)
            )?;
        }

        // 3. Coda
        for &(c, is_upper) in &self.coda {
            if is_upper {
                for u in c.to_uppercase() {
                    write!(f, "{u}")?;
                }
            } else {
                for l in c.to_lowercase() {
                    write!(f, "{l}")?;
                }
            }
        }

        Ok(())
    }
}
