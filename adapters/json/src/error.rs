use std::path::PathBuf;

/// Errors raised by `checkmate-adapter-json`.
#[derive(Debug, thiserror::Error)]
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
    /// The Source file exists but cannot be read.
    #[error("cannot read source file {}: {message}", .path.display())]
    Unreadable {
        /// The Source file.
        path: PathBuf,
        /// Cause reported by the operating system.
        message: String,
    },
    /// I/O failure.
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

impl Error {
    /// Classifies a failure to read the Source file at `path`.
    pub(crate) fn from_read(path: &std::path::Path, err: &std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Self::NotFound(path.to_owned()),
            std::io::ErrorKind::InvalidData => Self::invalid(path, "not valid UTF-8"),
            _ => Self::Unreadable {
                path: path.to_owned(),
                message: err.to_string(),
            },
        }
    }

    /// The content of the Source file at `path` is refused, for `message`.
    pub(crate) fn invalid(path: &std::path::Path, message: impl std::fmt::Display) -> Self {
        Self::Invalid {
            path: path.to_owned(),
            message: message.to_string(),
        }
    }
}
