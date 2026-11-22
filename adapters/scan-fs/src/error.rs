use std::path::PathBuf;

/// Errors raised by `rulelink-adapter-scan-fs`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A configured exclusion is not a valid ignore pattern.
    #[error("invalid `scan.exclude` pattern \"{pattern}\": {message}")]
    InvalidExclude {
        /// The pattern as configured.
        pattern: String,
        /// Why it is refused.
        message: String,
    },
    /// The project cannot be walked or a file cannot be read.
    #[error("cannot scan {}: {message}", .path.display())]
    Unreadable {
        /// The directory or file at fault.
        path: PathBuf,
        /// Cause reported by the operating system.
        message: String,
    },
}
