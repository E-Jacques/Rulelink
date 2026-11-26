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

/// A value that is not a level.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown level `{value}`, expected `error`, `warning` or `off`")]
pub struct UnknownLevel {
    /// The refused value.
    pub value: String,
}
