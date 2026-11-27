use std::collections::BTreeMap;
use std::str::FromStr;

use crate::{Code, UnknownLevel};

/// How much a Finding weighs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Fails `check`. Sorts before warnings.
    Error,
    /// Reported only.
    Warning,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Error => "error",
            Self::Warning => "warning",
        })
    }
}

/// What a team sets for a Finding: a severity, or nothing at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// Report it as an error.
    Error,
    /// Report it as a warning.
    Warning,
    /// Do not report it.
    Off,
}

impl Level {
    /// The severity to report with, `None` if the Finding is hidden.
    pub(crate) fn severity(self) -> Option<Severity> {
        match self {
            Self::Error => Some(Severity::Error),
            Self::Warning => Some(Severity::Warning),
            Self::Off => None,
        }
    }
}

impl FromStr for Level {
    type Err = UnknownLevel;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            "error" => Ok(Self::Error),
            "warning" => Ok(Self::Warning),
            "off" => Ok(Self::Off),
            _ => Err(UnknownLevel {
                value: raw.to_owned(),
            }),
        }
    }
}

/// The levels a team has set. A [`Code`] that was not set keeps its default.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Levels(BTreeMap<Code, Level>);

impl Levels {
    /// Sets the level of `code`.
    pub fn set(&mut self, code: Code, level: Level) {
        self.0.insert(code, level);
    }

    /// The level the team set for `code`, if any.
    pub(crate) fn get(&self, code: Code) -> Option<Level> {
        self.0.get(&code).copied()
    }
}
