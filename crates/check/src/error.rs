use crate::Code;

/// A name that is not the name of a Finding.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown Finding `{value}`, expected one of: {}", names())]
pub struct UnknownFinding {
    /// The refused name.
    pub value: String,
}

fn names() -> String {
    Code::ALL.map(Code::name).join(", ")
}

/// An attempt to lower or turn off a Finding that is always an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error(
    "{} ({}) is an error in the rules file: its level cannot be lowered or turned off",
    .code.name(),
    .code.id()
)]
pub struct FixedLevel {
    /// The Finding whose level is fixed.
    pub code: Code,
}

/// A value that is not a level.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown level `{value}`, expected `error`, `warning` or `off`")]
pub struct UnknownLevel {
    /// The refused value.
    pub value: String,
}
