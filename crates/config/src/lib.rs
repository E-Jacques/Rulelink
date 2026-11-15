//! Project discovery and configuration (checkmate.toml).

mod error;

pub use error::Error;

/// Name of the configuration file at the project root.
pub const CONFIG_FILE: &str = "checkmate.toml";
/// Default location of the Source, relative to the project root (always `/`-separated).
pub const DEFAULT_SOURCE_PATH: &str = "rules/rules.json";
/// Default Identifier prefix.
pub const DEFAULT_ID_PREFIX: &str = "RULE";

/// An Identifier prefix: an upper-case letter followed by upper-case letters, digits or `_`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdPrefix(String);

impl IdPrefix {
    /// Validates a prefix.
    ///
    /// # Errors
    /// Returns [`Error::InvalidIdPrefix`] if `raw` does not match `[A-Z][A-Z0-9_]*`.
    pub fn parse(raw: &str) -> Result<Self, Error> {
        let mut chars = raw.chars();
        let valid = chars.next().is_some_and(|c| c.is_ascii_uppercase())
            && chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
        if valid {
            Ok(Self(raw.to_owned()))
        } else {
            Err(Error::InvalidIdPrefix(raw.to_owned()))
        }
    }

    /// The prefix as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for IdPrefix {
    fn default() -> Self {
        Self(DEFAULT_ID_PREFIX.to_owned())
    }
}

/// Template of the `checkmate.toml` written by `init`, embedded at compile time.
const INITIAL_TEMPLATE: &str = include_str!("../templates/checkmate.toml");

/// Renders the `checkmate.toml` written by `init`.
#[must_use]
pub fn render_initial(prefix: &IdPrefix) -> String {
    INITIAL_TEMPLATE
        .replace("{{source_path}}", DEFAULT_SOURCE_PATH)
        .replace("{{id_prefix}}", prefix.as_str())
}
