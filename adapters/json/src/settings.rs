//! The settings of a JSON Source: `[source]`, apart from `kind`.

use std::path::Path;

use rulelink_adapter_sdk::{
    AdapterConfig, ConfigMap, ParseError, ParseResult, Parsed, Problem, ProjectPath, Property,
    RawTable, RawValue, TableParser, required_text,
};

/// The value of `source.kind` that selects this adapter.
const KIND: &str = "json";

/// `source.path`: the JSON file holding the Rules, inside the project. Required.
///
/// ```
/// use rulelink_adapter_json::SourcePath;
/// use rulelink_adapter_sdk::{Property, RawValue};
///
/// let raw = RawValue::String("rules/rules.json".to_owned());
/// let parsed = SourcePath::parse(Some(&raw))?;
/// assert!(parsed.configuration.as_path().ends_with("rules.json"));
/// # Ok::<(), rulelink_adapter_sdk::ParseError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcePath(ProjectPath);

impl SourcePath {
    /// The path, relative to the project root.
    #[must_use]
    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }
}

impl Property for SourcePath {
    const KEY: &'static str = "source.path";

    fn parse(raw: Option<&RawValue>) -> ParseResult<Self> {
        let text = required_text(Self::KEY, raw)?;
        let path = ProjectPath::parse(text).map_err(|why| {
            let message = format!("`{}` \"{}\" {why}", Self::KEY, text.escape_debug());
            ParseError::new(Problem::new(Self::KEY, message))
        })?;
        Ok(Parsed::new(Self(path)))
    }
}

/// The settings of this adapter, for the list the configuration is loaded with.
#[derive(Debug, Clone, Copy, Default)]
pub struct JsonConfig;

impl AdapterConfig for JsonConfig {
    fn kind(&self) -> &'static str {
        KIND
    }

    fn parse(&self, table: RawTable) -> ParseResult<ConfigMap> {
        TableParser::new(table).property::<SourcePath>().finish()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    fn source(path: &str) -> RawTable {
        let path = RawValue::String(path.to_owned());
        RawTable::new("source", BTreeMap::from([("path".to_owned(), path)]))
    }

    #[test]
    fn every_property_is_declared() {
        let parsed = JsonConfig.parse(source("a.json")).expect("valid");
        assert!(parsed.configuration.get::<SourcePath>().is_ok());
    }

    #[test]
    fn a_path_leaving_the_project_is_refused_by_name() {
        let error = JsonConfig.parse(source("../x.json")).expect_err("refused");
        assert_eq!(
            error.to_string(),
            "`source.path` \"../x.json\" must be a relative path inside the project, without `..`"
        );
    }

    #[test]
    fn the_path_is_required() {
        let error = JsonConfig
            .parse(RawTable::new("source", BTreeMap::new()))
            .expect_err("refused");
        assert_eq!(error.to_string(), "missing `source.path`");
    }
}
