/// Errors raised by `checkmate-model`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The requested capability is not implemented yet.
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
    /// A rule does not satisfy the model.
    #[error("{0}")]
    InvalidRule(String),
}
