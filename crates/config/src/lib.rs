//! Project discovery and configuration (rulelink.toml).
//!
//! This crate reads the file and owns the settings that no adapter owns. The caller passes
//! the adapters, which parse their own sections, so this crate depends on none of them.

mod error;
mod properties;
mod raw;

use std::path::Path;

use rulelink_adapter_sdk::{
    AdapterConfig, ConfigMap, ParseError, ParseResult, Parsed, Problem, Property, RawTable,
    TableParser, combine,
};

pub use error::Error;
pub use properties::{CheckLevels, RulesPrefix, SourceKind, Version};
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

/// The adapters whose settings `rulelink.toml` may hold.
#[derive(Clone, Copy)]
pub struct AdapterCatalog<'a> {
    /// One for each kind `source.kind` accepts; each parses the rest of `[source]`.
    pub sources: &'a [&'a dyn AdapterConfig],
    /// The adapter that scans the code; it parses `[scan]`.
    pub scanner: &'a dyn AdapterConfig,
}

/// Reads `rulelink.toml` in `root` and lets each owner parse its settings.
///
/// Read the result by property type: [`Version`], [`SourceKind`], [`RulesPrefix`],
/// [`CheckLevels`], then the properties of the selected source adapter and of the scanner.
///
/// ```no_run
/// # fn demo(catalog: rulelink_config::AdapterCatalog<'_>) -> Result<(), Box<dyn std::error::Error>> {
/// let parsed = rulelink_config::load(std::path::Path::new("."), catalog)?;
/// let prefix = parsed.configuration.get::<rulelink_config::RulesPrefix>()?;
/// println!("{}", prefix.prefix().as_str());
/// # Ok(())
/// # }
/// ```
///
/// # Errors
/// [`Error::NotFound`], [`Error::Unreadable`], or [`Error::Invalid`] holding every problem
/// found. Invalid TOML and an unsupported `version` are reported alone.
pub fn load(root: &Path, catalog: AdapterCatalog<'_>) -> Result<Parsed<ConfigMap>, Error> {
    let path = root.join(CONFIG_FILE);
    let text = std::fs::read_to_string(&path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => Error::NotFound(path.clone()),
        _ => Error::Unreadable {
            path: path.clone(),
            message: e.to_string(),
        },
    })?;
    parse(&text, catalog).map_err(Error::Invalid)
}

fn parse(text: &str, catalog: AdapterCatalog<'_>) -> ParseResult<ConfigMap> {
    let mut root = raw::root(text).map_err(|e| ParseError::new(Problem::new("", e.to_string())))?;
    // Nothing else can be trusted in a file of another version.
    let version = TableParser::new(root.split_off(Version::KEY))
        .property::<Version>()
        .finish()?;
    let rest = TableParser::new(root)
        .section("source", |table| parse_source(table, catalog))
        .section("rules", |table| {
            TableParser::new(table).property::<RulesPrefix>().finish()
        })
        .section("scan", |table| catalog.scanner.parse(table))
        .section("check", |table| {
            TableParser::new(table).property::<CheckLevels>().finish()
        })
        .reserved("adapters")
        .finish();
    combine(Ok(version), rest)
}

/// `[source]`: `kind` first, as it chooses who parses the other keys. If it is refused, they
/// are left unchecked.
fn parse_source(mut table: RawTable, catalog: AdapterCatalog<'_>) -> ParseResult<ConfigMap> {
    let kind = TableParser::new(table.split_off("kind"))
        .property::<SourceKind>()
        .finish()?;
    let adapter = selected(&kind.configuration, catalog)?;
    combine(Ok(kind), adapter.parse(table))
}

fn selected<'a>(
    kind: &ConfigMap,
    catalog: AdapterCatalog<'a>,
) -> Result<&'a dyn AdapterConfig, ParseError> {
    let refused = |message: String| ParseError::new(Problem::new(SourceKind::KEY, message));
    let kind = kind
        .get::<SourceKind>()
        .map_err(|e| refused(e.to_string()))?
        .as_str();
    catalog
        .sources
        .iter()
        .copied()
        .find(|adapter| adapter.kind() == kind)
        .ok_or_else(|| {
            refused(format!(
                "unsupported `{}` \"{}\", {}",
                SourceKind::KEY,
                kind.escape_debug(),
                available(catalog.sources)
            ))
        })
}

fn available(sources: &[&dyn AdapterConfig]) -> String {
    let kinds: Vec<String> = sources
        .iter()
        .map(|adapter| format!("\"{}\"", adapter.kind()))
        .collect();
    match kinds.as_slice() {
        [only] => format!("only {only} is available"),
        _ => format!("expected one of: {}", kinds.join(", ")),
    }
}

#[cfg(test)]
mod tests;
