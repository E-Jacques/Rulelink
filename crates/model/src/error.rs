/// Errors raised by `checkmate-model`.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The requested capability is not implemented yet.
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
    /// The Identifier prefix is malformed.
    #[error(
        "invalid Identifier prefix `{0}`: use upper-case letters, digits and `_`, starting with a letter"
    )]
    InvalidIdPrefix(String),
    /// A required text value is empty once trimmed.
    #[error("{field} must not be empty or blank")]
    Empty {
        /// The offending option, as typed on the command line.
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
        /// The offending option, as typed on the command line.
        field: &'static str,
        /// The repeated value.
        value: String,
    },
    /// No more Identifiers can be allocated for the prefix.
    #[error("no Identifier is left for prefix `{0}`")]
    IdentifierSpaceExhausted(String),
}
