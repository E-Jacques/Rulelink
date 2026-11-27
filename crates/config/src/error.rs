use std::path::PathBuf;

use rulelink_adapter_sdk::ParseError;

/// Errors raised by `rulelink-config`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
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
    /// The file is not valid TOML, or a setting is refused.
    #[error("invalid {}:{}", crate::CONFIG_FILE, listed(.0))]
    Invalid(ParseError),
}

/// One problem stays on the line of the file name; several are already listed below it.
fn listed(error: &ParseError) -> String {
    let separator = if error.problems().nth(1).is_none() {
        " "
    } else {
        ""
    };
    format!("{separator}{error}")
}
