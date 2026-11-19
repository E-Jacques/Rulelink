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

/// The settings of `checkmate.toml` that commands rely on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Location of the Source, relative to the project root.
    pub source_path: String,
    /// Prefix of the Identifiers allocated by the tool.
    pub id_prefix: IdPrefix,
}

#[derive(serde::Deserialize)]
struct RawConfig {
    source: RawSource,
    #[serde(default)]
    rules: RawRules,
}

#[derive(serde::Deserialize)]
struct RawSource {
    path: String,
}

#[derive(serde::Deserialize, Default)]
struct RawRules {
    prefix: Option<String>,
}

impl Config {
    /// Parses the content of `checkmate.toml`.
    ///
    /// # Errors
    /// [`Error::InvalidConfig`] for malformed TOML or a missing `[source] path`,
    /// [`Error::InvalidIdPrefix`] for a malformed prefix.
    pub fn parse(content: &str) -> Result<Self, Error> {
        let raw: RawConfig = toml::from_str(content).map_err(|e| Error::InvalidConfig {
            file: CONFIG_FILE,
            reason: e.message().to_owned(),
        })?;
        let id_prefix = raw
            .rules
            .prefix
            .as_deref()
            .map_or_else(|| Ok(IdPrefix::default()), IdPrefix::parse)?;
        Ok(Self {
            source_path: raw.source.path,
            id_prefix,
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
        let prefix = IdPrefix::parse("BR").unwrap_or_else(|e| panic!("{e}"));
        let config = Config::parse(&render_initial(&prefix)).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(config.source_path, DEFAULT_SOURCE_PATH);
        assert_eq!(config.id_prefix, prefix);
    }

    #[test]
    fn prefix_defaults_when_absent() {
        let config =
            Config::parse("[source]\npath = \"r.json\"\n").unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(config.id_prefix, IdPrefix::default());
    }

    #[test]
    fn malformed_content_is_refused() {
        assert!(Config::parse("not toml = = =").is_err());
        assert!(Config::parse("version = 1\n").is_err());
        assert!(Config::parse("[source]\npath = \"r\"\n[rules]\nprefix = \"bad\"\n").is_err());
    }
}
