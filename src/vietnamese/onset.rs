//! Vietnamese Onset (Phụ âm đầu) State Representation

use super::charset::compose_d;
use std::fmt;

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

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }
}

impl fmt::Display for OnsetState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &(c, is_upper) in &self.chars {
            if c.eq_ignore_ascii_case(&'d') {
                write!(f, "{}", compose_d(self.is_d_stroke, is_upper))?;
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
        Ok(())
    }
}

