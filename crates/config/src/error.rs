/// Errors raised by `checkmate-config`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The Identifier prefix is malformed.
    #[error(
        "invalid Identifier prefix `{0}`: use upper-case letters, digits and `_`, starting with a letter"
    )]
    InvalidIdPrefix(String),
    /// `checkmate.toml` is not valid.
    #[error("invalid {file}: {reason}", file = crate::CONFIG_FILE)]
    Invalid {
        /// What is wrong with the file.
        reason: String,
    },
}
