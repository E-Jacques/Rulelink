//! Project discovery and configuration (rulelink.toml).

mod error;
mod unknown;

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use rulelink_check::{Code, Level, Levels};
use serde::Deserialize;

pub use error::Error;
pub use rulelink_model::{DEFAULT_ID_PREFIX, IdPrefix};

/// Name of the configuration file at the project root.
pub const CONFIG_FILE: &str = "rulelink.toml";
/// Default location of the Source, relative to the project root (always `/`-separated).
pub const DEFAULT_SOURCE_PATH: &str = "rules/rules.json";
/// The only version of `rulelink.toml` this tool reads.
const SUPPORTED_VERSION: u32 = 1;
/// The only `source.kind` available today.
const SOURCE_KIND_JSON: &str = "json";
/// Starts a negated `.gitignore` pattern, which `scan.exclude` refuses.
const NEGATION: char = '!';
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
    /// The levels of the Findings that `[check.levels]` sets.
    pub check_levels: Levels,
    /// Names that have no effect, probably misspelled, in alphabetical order. A key of
    /// `[rules]` or `[scan]` is written `section.key`. `x-` names and `adapters` are not
    /// listed, and `[source]` is not looked into.
    pub unknown_keys: Vec<String>,
}

#[derive(Deserialize)]
struct RawConfig {
    version: Option<u32>,
    source: Option<RawSource>,
    rules: Option<RawRules>,
    scan: Option<RawScan>,
    check: Option<RawCheck>,
}

/// Unknown keys are refused: a setting such as a per-path override must not look applied.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCheck {
    levels: Option<BTreeMap<String, toml::Value>>,
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
        // The whole error, not only its message: it quotes the line at fault and its number.
        let raw: RawConfig = toml::from_str(text).map_err(|e| invalid(&e.to_string()))?;
        check_version(raw.version)?;
        let source_path = parse_source(raw.source)?;
        let prefix = raw
            .rules
            .and_then(|r| r.prefix)
            .ok_or_else(|| invalid("missing `rules.prefix`"))?;
        Ok(Self {
            id_prefix: IdPrefix::parse(&prefix)?,
            source_path,
            scan_exclude: parse_scan_exclude(raw.scan)?,
            check_levels: parse_check_levels(raw.check)?,
            unknown_keys: unknown::unknown_keys(text)?,
        })
    }
}

fn check_version(version: Option<u32>) -> Result<(), Error> {
    match version {
        Some(SUPPORTED_VERSION) => Ok(()),
        Some(other) => Err(invalid(&format!(
            "unsupported version {other}, expected {SUPPORTED_VERSION}"
        ))),
        None => Err(invalid("missing `version`")),
    }
}

/// Validates `[source]` and returns the location of the Source.
fn parse_source(source: Option<RawSource>) -> Result<PathBuf, Error> {
    let source = source.ok_or_else(|| invalid("missing `[source]` section"))?;
    match source.kind.as_deref() {
        Some(SOURCE_KIND_JSON) => {}
        Some(other) => {
            return Err(invalid(&format!(
                "unsupported `source.kind` \"{other}\", only \"{SOURCE_KIND_JSON}\" is available"
            )));
        }
        None => return Err(invalid("missing `source.kind`")),
    }
    relative_inside_project(
        source
            .path
            .as_deref()
            .ok_or_else(|| invalid("missing `source.path`"))?,
    )
}

/// Validates `[scan]` and returns the exclusion patterns (none when the section is absent).
fn parse_scan_exclude(scan: Option<RawScan>) -> Result<Vec<String>, Error> {
    let patterns: Vec<String> = scan
        .and_then(|s| s.exclude)
        .unwrap_or_default()
        .iter()
        .map(|p| p.trim().to_owned())
        .collect();
    if patterns.iter().any(String::is_empty) {
        return Err(invalid("`scan.exclude` must not contain an empty pattern"));
    }
    if let Some(negated) = patterns.iter().find(|p| p.starts_with(NEGATION)) {
        return Err(invalid(&format!(
            "`scan.exclude` pattern \"{negated}\" starts with `{NEGATION}`: list only what to skip"
        )));
    }
    Ok(patterns)
}

/// Validates `[check.levels]` (none set when the section is absent).
fn parse_check_levels(check: Option<RawCheck>) -> Result<Levels, Error> {
    let mut levels = Levels::default();
    for (name, value) in check.and_then(|c| c.levels).unwrap_or_default() {
        let refused = |why: &str| invalid(&format!("`check.levels.{name}`: {why}"));
        let code: Code = name
            .parse()
            .map_err(|e: rulelink_check::UnknownFinding| refused(&e.to_string()))?;
        let level: Level = value
            .as_str()
            .ok_or_else(|| {
                refused(&format!(
                    "expected `error`, `warning` or `off` in quotes, found {}",
                    value.type_str()
                ))
            })?
            .parse()
            .map_err(|e: rulelink_check::UnknownLevel| refused(&e.to_string()))?;
        levels.set(code, level);
    }
    Ok(levels)
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
        let padded = format!("{base}[scan]\nexclude = [\" vendor/ \"]\n");
        assert_eq!(
            Config::parse(&padded).expect("valid").scan_exclude,
            ["vendor/"]
        );
        let negated = format!("{base}[scan]\nexclude = [\"!src/\"]\n");
        assert!(matches!(
            Config::parse(&negated),
            Err(Error::Invalid { .. })
        ));
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
