/// Errors raised by `checkmate-adapter-json`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The Source file already exists.
    #[error("source file already exists")]
    AlreadyExists,
    /// I/O failure.
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
