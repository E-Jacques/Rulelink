/// Errors raised by `checkmate-import`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The requested capability is not implemented yet.
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
}
