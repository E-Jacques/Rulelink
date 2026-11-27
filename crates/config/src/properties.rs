//! The settings that do not belong to an adapter.

use rulelink_adapter_sdk::{
    ParseError, ParseResult, Parsed, Problem, Property, RawValue, is_extension, required_text,
};
use rulelink_check::{Code, Level, Levels};
use rulelink_model::IdPrefix;

/// The only version of `rulelink.toml` this tool reads.
const SUPPORTED_VERSION: i64 = 1;

/// `version`: the version of the file format. Required.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version(());

impl Property for Version {
    const KEY: &'static str = "version";

    fn parse(raw: Option<&RawValue>) -> ParseResult<Self> {
        let message = match raw {
            Some(RawValue::Integer(SUPPORTED_VERSION)) => return Ok(Parsed::new(Self(()))),
            Some(RawValue::Integer(other)) => unsupported(other),
            Some(RawValue::LargeInteger(other)) => unsupported(other),
            Some(other) => format!(
                "`{}` must be the number {SUPPORTED_VERSION}, found {}",
                Self::KEY,
                other.kind_name()
            ),
            None => format!("missing `{}`", Self::KEY),
        };
        Err(refused(Self::KEY, message))
    }
}

fn unsupported(version: impl std::fmt::Display) -> String {
    format!("unsupported version {version}, expected {SUPPORTED_VERSION}")
}

/// `source.kind`: the adapter that reads the Source. Required. Whether the kind exists is
/// decided by the [`AdapterCatalog`](crate::AdapterCatalog).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceKind(String);

impl SourceKind {
    /// The kind as written.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Property for SourceKind {
    const KEY: &'static str = "source.kind";

    fn parse(raw: Option<&RawValue>) -> ParseResult<Self> {
        let kind = required_text(Self::KEY, raw)?;
        Ok(Parsed::new(Self(kind.to_owned())))
    }
}

/// `rules.prefix`: the prefix of the Identifiers allocated by the tool. Required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulesPrefix(IdPrefix);

impl RulesPrefix {
    /// The prefix.
    #[must_use]
    pub fn prefix(&self) -> &IdPrefix {
        &self.0
    }
}

impl Property for RulesPrefix {
    const KEY: &'static str = "rules.prefix";

    fn parse(raw: Option<&RawValue>) -> ParseResult<Self> {
        let text = required_text(Self::KEY, raw)?;
        let prefix = IdPrefix::parse(text)
            .map_err(|why| refused(Self::KEY, format!("`{}`: {why}", Self::KEY)))?;
        Ok(Parsed::new(Self(prefix)))
    }
}

/// `check.levels`: the levels of the Findings that the team changes. Optional: none by
/// default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckLevels(Levels);

impl CheckLevels {
    /// The levels that were set.
    #[must_use]
    pub fn levels(&self) -> &Levels {
        &self.0
    }
}

impl Property for CheckLevels {
    const KEY: &'static str = "check.levels";

    fn parse(raw: Option<&RawValue>) -> ParseResult<Self> {
        let mut levels = Levels::default();
        let mut problems = Vec::new();
        match raw {
            None => {}
            Some(RawValue::Table(entries)) => {
                for (name, value) in entries.iter().filter(|(name, _)| !is_extension(name)) {
                    if let Err(why) = set_level(&mut levels, name, value) {
                        let key = format!("{}.{}", Self::KEY, name.escape_debug());
                        let message = format!("`{key}`: {why}");
                        problems.push(Problem::new(key, message));
                    }
                }
            }
            Some(other) => {
                let message = format!(
                    "`{}` must be a section, found {}",
                    Self::KEY,
                    other.kind_name()
                );
                problems.push(Problem::new(Self::KEY, message));
            }
        }
        match ParseError::from_problems(problems) {
            Some(error) => Err(error),
            None => Ok(Parsed::new(Self(levels))),
        }
    }
}

/// Applies one line of `[check.levels]` to `levels`, or says why it is refused.
fn set_level(levels: &mut Levels, name: &str, value: &RawValue) -> Result<(), String> {
    let code: Code = name
        .parse()
        .map_err(|e: rulelink_check::UnknownFinding| e.to_string())?;
    let RawValue::String(text) = value else {
        return Err(format!(
            "expected `error`, `warning` or `off` in quotes, found {}",
            value.kind_name()
        ));
    };
    let level: Level = text
        .parse()
        .map_err(|e: rulelink_check::UnknownLevel| e.to_string())?;
    levels
        .set(code, level)
        .map_err(|e: rulelink_check::FixedLevel| e.to_string())
}

fn refused(key: &str, message: String) -> ParseError {
    ParseError::new(Problem::new(key, message))
}
