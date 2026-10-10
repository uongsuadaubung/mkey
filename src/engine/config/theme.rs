//! UI Theme mode definitions (Auto, Light, Dark)

use std::fmt;
use std::str::FromStr;

/// UI Theme mode (Auto, Light, Dark)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UiTheme {
    #[default]
    Auto,
    Light,
    Dark,
}

impl UiTheme {
    /// String representation used for configuration storage
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

impl fmt::Display for UiTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for UiTheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for UiTheme {
    fn from(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "light" => Self::Light,
            "dark" => Self::Dark,
            _ => Self::Auto,
        }
    }
}

impl FromStr for UiTheme {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl From<UiTheme> for usize {
    fn from(t: UiTheme) -> Self {
        match t {
            UiTheme::Auto => 0,
            UiTheme::Light => 1,
            UiTheme::Dark => 2,
        }
    }
}

impl From<usize> for UiTheme {
    fn from(idx: usize) -> Self {
        match idx {
            1 => Self::Light,
            2 => Self::Dark,
            _ => Self::Auto,
        }
    }
}
