/// Errors raised by `checkmate-model`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The requested capability is not implemented yet.
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
    /// A Rule does not satisfy the Rule model.
    #[error("invalid rule: {0}")]
    InvalidRule(String),
    /// No Identifier is left for a prefix.
    #[error("no identifier left for prefix `{0}`")]
    IdentifiersExhausted(String),
}
