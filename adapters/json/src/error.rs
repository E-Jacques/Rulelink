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
    /// The file is not a valid Source.
    #[error("not a valid rules file: {0}")]
    Invalid(String),
    /// The Source uses a format this version does not understand.
    #[error("unsupported rules file format {0}")]
    UnsupportedFormat(u32),
}
