/// The Identifier prefix is malformed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error(
    "invalid Identifier prefix `{0}`: use upper-case letters, digits and `_`, starting with a letter"
)]
pub struct InvalidIdPrefix(pub String);

/// A value given for a new Rule is refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DraftError {
    /// A required text value is empty once trimmed.
    #[error("{field} must not be empty or blank")]
    Empty {
        /// Name of the offending value.
        field: &'static str,
    },
    /// The title is longer than allowed.
    #[error("title is too long: {len} characters, at most {max} are allowed")]
    TitleTooLong {
        /// Length of the stored title, in characters.
        len: usize,
        /// Maximum length.
        max: usize,
    },
    /// A list value appears twice.
    #[error("{field} lists `{value}` more than once")]
    Duplicate {
        /// Name of the offending value.
        field: &'static str,
        /// The repeated value.
        value: String,
    },
}

/// Errors raised by `checkmate-model` outside input validation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The requested capability is not implemented yet.
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
    /// No more Identifiers can be allocated for the prefix.
    #[error("no Identifier is left for prefix `{0}`")]
    IdentifierSpaceExhausted(String),
}
