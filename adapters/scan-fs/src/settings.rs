//! The settings of the code scan: `[scan]`.

use ignore::overrides::OverrideBuilder;
use rulelink_adapter_sdk::{
    AdapterConfig, ConfigMap, ParseError, ParseResult, Parsed, Problem, Property, RawTable,
    RawValue, TableParser,
};

/// Starts a negated `.gitignore` pattern, which `scan.exclude` refuses.
const NEGATION: char = '!';

/// `scan.exclude`: `.gitignore`-style patterns, relative to the project root, of the files
/// the code scan skips. Optional: nothing is excluded by default.
///
/// Only parsing builds one, so each pattern is trimmed, not blank, not negated and valid.
///
/// ```
/// use rulelink_adapter_sdk::Property;
/// use rulelink_adapter_scan_fs::ScanExclude;
///
/// let nothing = ScanExclude::parse(None)?;
/// assert!(nothing.configuration.patterns().is_empty());
/// # Ok::<(), rulelink_adapter_sdk::ParseError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanExclude(Vec<String>);

impl ScanExclude {
    /// The patterns, in the order written.
    #[must_use]
    pub fn patterns(&self) -> &[String] {
        &self.0
    }
}

impl Property for ScanExclude {
    const KEY: &'static str = "scan.exclude";

    fn parse(raw: Option<&RawValue>) -> ParseResult<Self> {
        let items = match raw {
            None => &[][..],
            Some(RawValue::Array(items)) => items,
            Some(other) => {
                return Err(refused(format!(
                    "`{}` must be a list of patterns, found {}",
                    Self::KEY,
                    other.kind_name()
                )));
            }
        };
        let mut patterns = Vec::new();
        let mut errors = Vec::new();
        for item in items {
            match pattern(item) {
                Ok(pattern) => patterns.push(pattern),
                Err(message) => errors.push(message),
            }
        }
        errors.dedup();
        match ParseError::from_problems(errors.into_iter().map(problem)) {
            Some(error) => Err(error),
            None => Ok(Parsed::new(Self(patterns))),
        }
    }
}

/// One entry of the list, trimmed, or why it is refused.
fn pattern(item: &RawValue) -> Result<String, String> {
    let key = ScanExclude::KEY;
    let RawValue::String(raw) = item else {
        return Err(format!(
            "`{key}` must hold only text in quotes, found {}",
            item.kind_name()
        ));
    };
    let pattern = raw.trim();
    let shown = pattern.escape_debug();
    if pattern.is_empty() {
        Err(format!("`{key}` must not contain an empty pattern"))
    } else if pattern.starts_with(NEGATION) {
        Err(format!(
            "`{key}` pattern \"{shown}\" starts with `{NEGATION}`: list only what to skip"
        ))
    } else {
        match OverrideBuilder::new(".").add(&format!("{NEGATION}{pattern}")) {
            Ok(_) => Ok(pattern.to_owned()),
            Err(e) => Err(format!("`{key}` pattern \"{shown}\" is invalid: {e}")),
        }
    }
}

fn problem(message: String) -> Problem {
    Problem::new(ScanExclude::KEY, message)
}

fn refused(message: String) -> ParseError {
    ParseError::new(problem(message))
}

/// The settings of this adapter, for the list the configuration is loaded with.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScanFsConfig;

impl AdapterConfig for ScanFsConfig {
    fn kind(&self) -> &'static str {
        "fs"
    }

    fn parse(&self, table: RawTable) -> ParseResult<ConfigMap> {
        TableParser::new(table).property::<ScanExclude>().finish()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    fn list(patterns: &[&str]) -> RawValue {
        RawValue::Array(
            patterns
                .iter()
                .map(|p| RawValue::String((*p).to_owned()))
                .collect(),
        )
    }

    fn problems(raw: &RawValue) -> Vec<String> {
        let error = ScanExclude::parse(Some(raw)).expect_err("refused");
        error.problems().map(|p| p.message.clone()).collect()
    }

    #[test]
    fn every_property_is_declared() {
        let parsed = ScanFsConfig
            .parse(RawTable::new("scan", BTreeMap::new()))
            .expect("valid");
        assert!(parsed.configuration.get::<ScanExclude>().is_ok());
    }

    #[test]
    fn patterns_are_trimmed() {
        let parsed = ScanExclude::parse(Some(&list(&[" vendor/ ", "*.gen.rs"]))).expect("valid");
        assert_eq!(parsed.configuration.patterns(), ["vendor/", "*.gen.rs"]);
    }

    #[test]
    fn every_refused_pattern_is_reported() {
        let found = problems(&list(&[" ", "!src/", "[", "ok"]));
        assert_eq!(found.len(), 3, "{found:?}");
        assert!(found[0].contains("empty pattern"), "{found:?}");
        assert!(found[1].contains("\"!src/\" starts with `!`"), "{found:?}");
        assert!(found[2].contains("pattern \"[\" is invalid"), "{found:?}");
    }

    #[test]
    fn a_pattern_cannot_add_lines_to_a_message() {
        let found = problems(&list(&["!a\nb"]));
        assert!(found[0].contains("a\\nb"), "{found:?}");
    }

    #[test]
    fn another_type_is_refused() {
        assert!(problems(&RawValue::Boolean(true))[0].contains("list of patterns"));
        let numbers = RawValue::Array(vec![RawValue::Integer(1)]);
        assert!(problems(&numbers)[0].contains("only text"));
    }
}
