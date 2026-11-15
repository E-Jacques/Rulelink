/// Errors raised by `checkmate-config`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The Identifier prefix is malformed.
    #[error(
        "invalid Identifier prefix `{0}`: use upper-case letters, digits and `_`, starting with a letter"
    )]
    InvalidIdPrefix(String),
}
