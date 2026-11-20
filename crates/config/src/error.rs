use std::path::PathBuf;

/// Errors raised by `rulelink-config`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The Identifier prefix is malformed.
    #[error(transparent)]
    InvalidIdPrefix(#[from] rulelink_model::InvalidIdPrefix),
    /// There is no `rulelink.toml` in the project root.
    #[error("{} not found: run `rulelink init` first", .0.display())]
    NotFound(PathBuf),
    /// The file exists but cannot be read.
    #[error("cannot read {}: {message}", .path.display())]
    Unreadable {
        /// The configuration file.
        path: PathBuf,
        /// Cause reported by the operating system.
        message: String,
    },
    /// The file is not valid, or misses a required value.
    #[error("invalid {file}: {message}")]
    Invalid {
        /// File name.
        file: &'static str,
        /// What is wrong and how to fix it.
        message: String,
    },
}
