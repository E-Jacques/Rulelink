use std::path::PathBuf;

/// Errors raised by `checkmate-adapter-json`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The Source file already exists.
    #[error("source file already exists")]
    AlreadyExists,
    /// The Source file does not exist.
    #[error("source file {} not found", .0.display())]
    NotFound(PathBuf),
    /// The Source file is not valid JSON of the expected shape.
    #[error("source file {} is invalid: {message}", .path.display())]
    Invalid {
        /// The Source file.
        path: PathBuf,
        /// Parser message, with line and column when known.
        message: String,
    },
    /// The Source file uses a `format` this version cannot handle.
    #[error("source file {} has unsupported format {found}, expected {expected}", .path.display())]
    UnsupportedFormat {
        /// The Source file.
        path: PathBuf,
        /// The `format` found in the file.
        found: u32,
        /// The `format` this version handles.
        expected: u32,
    },
    /// I/O failure.
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
