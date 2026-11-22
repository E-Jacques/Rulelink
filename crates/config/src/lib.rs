//! Project discovery and configuration (rulelink.toml).

mod error;

use std::path::{Component, Path, PathBuf};

use serde::Deserialize;

pub use error::Error;
pub use rulelink_model::{DEFAULT_ID_PREFIX, IdPrefix};

/// Name of the configuration file at the project root.
pub const CONFIG_FILE: &str = "rulelink.toml";
/// Default location of the Source, relative to the project root (always `/`-separated).
pub const DEFAULT_SOURCE_PATH: &str = "rules/rules.json";
/// Template of the `rulelink.toml` written by `init`, embedded at compile time.
const INITIAL_TEMPLATE: &str = include_str!("../templates/rulelink.toml");

/// Renders the `rulelink.toml` written by `init`.
#[must_use]
pub fn render_initial(prefix: &IdPrefix) -> String {
    INITIAL_TEMPLATE
        .replace("{{source_path}}", DEFAULT_SOURCE_PATH)
        .replace("{{id_prefix}}", prefix.as_str())
}

/// The configuration the commands need, parsed and validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Prefix of the Identifiers allocated by the tool.
    pub id_prefix: IdPrefix,
    /// Location of the Source, relative to the project root.
    pub source_path: PathBuf,
    /// `.gitignore`-style patterns of files the code scan skips (`[scan] exclude`).
    pub scan_exclude: Vec<String>,
}

#[derive(Deserialize)]
struct RawConfig {
    version: Option<u32>,
    source: Option<RawSource>,
    rules: Option<RawRules>,
    scan: Option<RawScan>,
}

#[derive(Deserialize)]
struct RawScan {
    exclude: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct RawSource {
    kind: Option<String>,
    path: Option<String>,
}

#[derive(Deserialize)]
struct RawRules {
    prefix: Option<String>,
}

impl Config {
    /// Reads and validates `rulelink.toml` in `root`.
    ///
    /// Strict: an unknown `source.kind`, a missing section or a `source.path` that is not
    /// relative or leaves the project is an error. Nothing is defaulted silently.
    ///
    /// # Errors
    /// [`Error::NotFound`], [`Error::Unreadable`] or [`Error::Invalid`] (plus
    /// [`Error::InvalidIdPrefix`] for a malformed prefix).
    pub fn load(root: &Path) -> Result<Self, Error> {
        let path = root.join(CONFIG_FILE);
        let text = std::fs::read_to_string(&path).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => Error::NotFound(path.clone()),
            _ => Error::Unreadable {
                path: path.clone(),
                message: e.to_string(),
            },
        })?;
        Self::parse(&text)
    }

    fn parse(text: &str) -> Result<Self, Error> {
        let raw: RawConfig = toml::from_str(text).map_err(|e| invalid(e.message()))?;
        match raw.version {
            Some(1) => {}
            Some(other) => {
                return Err(invalid(&format!("unsupported version {other}, expected 1")));
            }
            None => return Err(invalid("missing `version`")),
        }
        let source = raw
            .source
            .ok_or_else(|| invalid("missing `[source]` section"))?;
        match source.kind.as_deref() {
            Some("json") => {}
            Some(other) => {
                return Err(invalid(&format!(
                    "unsupported `source.kind` \"{other}\", only \"json\" is available"
                )));
            }
            None => return Err(invalid("missing `source.kind`")),
        }
        let source_path = relative_inside_project(
            source
                .path
                .as_deref()
                .ok_or_else(|| invalid("missing `source.path`"))?,
        )?;
        let prefix = raw
            .rules
            .and_then(|r| r.prefix)
            .ok_or_else(|| invalid("missing `rules.prefix`"))?;
        let scan_exclude = raw.scan.and_then(|s| s.exclude).unwrap_or_default();
        if scan_exclude.iter().any(|p| p.trim().is_empty()) {
            return Err(invalid("`scan.exclude` must not contain an empty pattern"));
        }
        Ok(Self {
            id_prefix: IdPrefix::parse(&prefix)?,
            source_path,
            scan_exclude,
        })
    }
}

fn invalid(message: &str) -> Error {
    Error::Invalid {
        file: CONFIG_FILE,
        message: message.to_owned(),
    }
}

fn relative_inside_project(raw: &str) -> Result<PathBuf, Error> {
    let path = PathBuf::from(raw);
    let plain = !raw.is_empty()
        && path
            .components()
            .all(|c| matches!(c, Component::Normal(_) | Component::CurDir));
    if plain {
        Ok(path)
    } else {
        Err(invalid(&format!(
            "`source.path` \"{raw}\" must be a relative path inside the project, without `..`"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(path: &str, kind: &str, prefix: &str) -> String {
        format!(
            "version = 1\n[source]\nkind = \"{kind}\"\npath = \"{path}\"\n[rules]\nprefix = \"{prefix}\"\n"
        )
    }

    #[test]
    fn rendered_initial_configuration_loads_back() {
        let prefix = IdPrefix::parse("BR").unwrap_or_else(|e| panic!("{e}"));
        let loaded = Config::parse(&render_initial(&prefix)).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(loaded.id_prefix, prefix);
        assert_eq!(loaded.source_path, PathBuf::from(DEFAULT_SOURCE_PATH));
    }

    #[test]
    fn unknown_source_kind_is_refused() {
        let err = Config::parse(&config("a.json", "csv", "RULE")).map(|_| ());
        assert!(matches!(err, Err(Error::Invalid { ref message, .. }) if message.contains("csv")));
    }

    #[test]
    fn paths_leaving_the_project_are_refused() {
        for path in ["../x.json", "a/../../x.json", "/etc/x.json", ""] {
            assert!(
                Config::parse(&config(path, "json", "RULE")).is_err(),
                "{path}"
            );
        }
        assert!(Config::parse(&config("./a/b.json", "json", "RULE")).is_ok());
    }

    #[test]
    fn missing_sections_and_bad_prefix_are_refused() {
        assert!(Config::parse("").is_err());
        assert!(Config::parse("version = 1\n").is_err());
        assert!(Config::parse("version = 2\n").is_err());
        assert!(Config::parse("version = 1\n[source]\nkind = \"json\"\npath = \"a\"\n").is_err());
        assert!(matches!(
            Config::parse(&config("a.json", "json", "bad")),
            Err(Error::InvalidIdPrefix(_))
        ));
        assert!(Config::parse("not toml {").is_err());
    }

    #[test]
    fn scan_exclusions_are_optional_and_never_blank() {
        let base = config("a.json", "json", "RULE");
        assert_eq!(Config::parse(&base).expect("valid").scan_exclude.len(), 0);
        let with = format!("{base}[scan]\nexclude = [\"vendor/\", \"*.gen.rs\"]\n");
        assert_eq!(
            Config::parse(&with).expect("valid").scan_exclude,
            ["vendor/", "*.gen.rs"]
        );
        let blank = format!("{base}[scan]\nexclude = [\" \"]\n");
        assert!(matches!(Config::parse(&blank), Err(Error::Invalid { .. })));
    }

    #[test]
    fn missing_file_points_to_init() {
        let dir = std::env::temp_dir().join("rulelink-config-missing-test");
        let err = Config::load(&dir)
            .expect_err("no config in an empty directory")
            .to_string();
        assert!(err.contains("rulelink init"), "{err}");
    }
}
