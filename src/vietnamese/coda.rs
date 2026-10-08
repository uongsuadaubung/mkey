//! Vietnamese Coda (Phụ âm cuối) State Representation

use super::charset::{Diacritic, Tone, compose_vowel};
use super::nucleus::NucleusState;
use super::syllable::Syllable;
use std::fmt;

/// Coda (Phụ âm cuối)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodaState {
    pub nucleus: NucleusState,
    pub coda: Vec<(char, bool)>,
}

impl CodaState {
    pub fn to_syllable(&self) -> Syllable {
        self.into()
    }
}

impl fmt::Display for CodaState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref onset) = self.nucleus.onset {
            write!(f, "{onset}")?;
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
            write!(
                f,
                "{}",
                compose_vowel(v.base, v.diacritic, tone_for_vowel, v.is_upper)
            )?;
        }

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

impl From<&CodaState> for Syllable {
    fn from(coda: &CodaState) -> Self {
        Syllable {
            onset: coda
                .nucleus
                .onset
                .as_ref()
                .map(|o| o.chars.clone())
                .unwrap_or_default(),
            d_stroke: coda
                .nucleus
                .onset
                .as_ref()
                .is_some_and(|o| o.is_d_stroke),
            vowels: coda.nucleus.vowels.clone(),
            tone: coda.nucleus.tone,
            coda: coda.coda.clone(),
        }
    }
}

