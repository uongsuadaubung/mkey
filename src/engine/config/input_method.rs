//! Vietnamese Input Method definitions and syntax helpers

use std::fmt;
use std::str::FromStr;

/// Vietnamese Input Method type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMethod {
    #[default]
    Telex,
    Vni,
    SimpleTelex1,
    SimpleTelex2,
}

impl InputMethod {
    /// String representation used for configuration storage
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Telex => "telex",
            Self::Vni => "vni",
            Self::SimpleTelex1 => "simple_telex1",
            Self::SimpleTelex2 => "simple_telex2",
        }
    }

    /// Whether this input method belongs to the Telex syntax family (using letter keys as accents)
    pub const fn is_telex_family(&self) -> bool {
        matches!(
            self,
            InputMethod::Telex | InputMethod::SimpleTelex1 | InputMethod::SimpleTelex2
        )
    }

    /// Whether this input method is VNI (using number keys 1-9 as accents)
    pub const fn is_vni(&self) -> bool {
        matches!(self, InputMethod::Vni)
    }

    /// Whether standalone 'w' at word start or empty state turns into 'ư'
    /// - Standard Telex: Yes ('w' -> 'ư')
    /// - Simple Telex 1: No ('w' stays literal 'w' for English words like "win", "web")
    pub const fn has_standalone_w(&self) -> bool {
        matches!(self, InputMethod::Telex | InputMethod::SimpleTelex2)
    }

    /// Whether square bracket keys '[' and ']' act as shortcuts for 'ư' and 'ơ'
    /// - Standard Telex: Yes ('[' -> 'ư', ']' -> 'ơ')
    /// - Simple Telex: No ('[' and ']' remain brackets for programming)
    pub const fn has_bracket_shortcuts(&self) -> bool {
        matches!(self, InputMethod::Telex)
    }
}

impl fmt::Display for InputMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for InputMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for InputMethod {
    fn from(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "vni" => Self::Vni,
            "simple_telex1" | "simpletelex1" | "simpletelex" => Self::SimpleTelex1,
            "simple_telex2" | "simpletelex2" => Self::SimpleTelex2,
            _ => Self::Telex,
        }
    }
}

impl FromStr for InputMethod {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl From<InputMethod> for usize {
    fn from(m: InputMethod) -> Self {
        match m {
            InputMethod::Telex => 0,
            InputMethod::Vni => 1,
            InputMethod::SimpleTelex1 | InputMethod::SimpleTelex2 => 2,
        }
    }
}

impl From<usize> for InputMethod {
    fn from(idx: usize) -> Self {
        match idx {
            1 => Self::Vni,
            2 => Self::SimpleTelex1,
            _ => Self::Telex,
        }
    }
}
