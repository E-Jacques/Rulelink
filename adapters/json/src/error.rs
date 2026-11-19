/// Errors raised by `checkmate-adapter-json`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The Source file already exists.
    #[error("source file already exists")]
    AlreadyExists,
    /// The Source file does not exist.
    #[error("source file not found")]
    NotFound,
    /// The Source file is not valid JSON of the Rule model.
    #[error("source file is invalid: {0}")]
    Invalid(String),
    /// I/O failure.
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
