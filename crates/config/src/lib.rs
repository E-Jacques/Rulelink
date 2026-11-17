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

/// The settings of `checkmate.toml` that commands need.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Prefix of the Identifiers allocated to new Rules.
    pub prefix: IdPrefix,
    /// Location of the Source, relative to the project root.
    pub source_path: String,
}

#[derive(serde::Deserialize)]
struct RawConfig {
    source: RawSource,
    rules: RawRules,
}

#[derive(serde::Deserialize)]
struct RawSource {
    path: String,
}

#[derive(serde::Deserialize)]
struct RawRules {
    prefix: String,
}

impl Config {
    /// Parses the content of a `checkmate.toml`.
    ///
    /// # Errors
    /// Returns [`Error::Invalid`] if the TOML is malformed or lacks a setting, and
    /// [`Error::InvalidIdPrefix`] if the prefix is not valid.
    pub fn parse(content: &str) -> Result<Self, Error> {
        let raw: RawConfig = toml::from_str(content).map_err(|e| Error::Invalid {
            reason: e.message().to_owned(),
        })?;
        Ok(Self {
            prefix: IdPrefix::parse(&raw.rules.prefix)?,
            source_path: raw.source.path,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_what_init_writes() {
        let text = render_initial(&IdPrefix::parse("BR").unwrap_or_default());
        let config = Config::parse(&text).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(config.prefix.as_str(), "BR");
        assert_eq!(config.source_path, DEFAULT_SOURCE_PATH);
    }

    #[test]
    fn rejects_missing_settings_and_bad_prefix() {
        assert!(matches!(
            Config::parse("version = 1\n"),
            Err(Error::Invalid { .. })
        ));
        let bad = render_initial(&IdPrefix::default()).replace("RULE", "bad one");
        assert!(matches!(
            Config::parse(&bad),
            Err(Error::InvalidIdPrefix(_))
        ));
    }
}
