use std::collections::BTreeMap;

use crate::suggest::closest;
use crate::{ParseError, Problem};

/// Starts a name reserved for a team's own notes and other tools.
const EXTENSION_PREFIX: &str = "x-";

/// Whether `name` is reserved for notes and other tools, and so never read nor refused.
///
/// ```
/// use rulelink_adapter_sdk::is_extension;
///
/// assert!(is_extension("x-owner"));
/// assert!(!is_extension("owner"));
/// ```
#[must_use]
pub fn is_extension(name: &str) -> bool {
    name.starts_with(EXTENSION_PREFIX)
}

/// A setting as written, before any property has read it.
///
/// The SDK has its own type so that no file format is part of the adapter contract.
///
/// ```
/// use rulelink_adapter_sdk::RawValue;
///
/// assert_eq!(RawValue::Boolean(true).kind_name(), "a boolean");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum RawValue {
    /// Text.
    String(String),
    /// A whole number.
    Integer(i64),
    /// A whole number too large to hold, as written. It fails only where a property reads it.
    LargeInteger(String),
    /// A number with a fractional part.
    Float(f64),
    /// `true` or `false`.
    Boolean(bool),
    /// A date or a time, as written.
    Datetime(String),
    /// A list.
    Array(Vec<RawValue>),
    /// A section.
    Table(BTreeMap<String, RawValue>),
}

impl RawValue {
    /// What the value is, in words for a message.
    #[must_use]
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::String(_) => "text",
            Self::Integer(_) | Self::LargeInteger(_) | Self::Float(_) => "a number",
            Self::Boolean(_) => "a boolean",
            Self::Datetime(_) => "a date",
            Self::Array(_) => "a list",
            Self::Table(_) => "a section",
        }
    }
}

/// The text of a required setting, for a property that reads one.
///
/// ```
/// use rulelink_adapter_sdk::{RawValue, required_text};
///
/// let raw = RawValue::String("json".to_owned());
/// assert_eq!(required_text("source.kind", Some(&raw)), Ok("json"));
/// assert!(required_text("source.kind", None).is_err());
/// ```
///
/// # Errors
/// [`ParseError`] if `raw` is absent or is not text.
pub fn required_text<'a>(key: &str, raw: Option<&'a RawValue>) -> Result<&'a str, ParseError> {
    match raw {
        Some(RawValue::String(text)) => Ok(text),
        Some(other) => Err(ParseError::new(Problem::new(
            key,
            format!(
                "`{key}` must be text in quotes, found {}",
                other.kind_name()
            ),
        ))),
        None => Err(ParseError::new(Problem::new(
            key,
            format!("missing `{key}`"),
        ))),
    }
}

/// The settings of one section, which a [`TableParser`](crate::TableParser) reads.
///
/// ```
/// use std::collections::BTreeMap;
/// use rulelink_adapter_sdk::{RawTable, RawValue};
///
/// let entries = BTreeMap::from([("kind".to_owned(), RawValue::Boolean(true))]);
/// let mut source = RawTable::new("source", entries);
/// let kind = source.split_off("kind");
/// assert_ne!(kind, source);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RawTable {
    /// The dotted name of the section; empty for the top of the file.
    pub(crate) path: String,
    pub(crate) entries: BTreeMap<String, RawValue>,
    /// The names split off to be parsed apart: still known when suggesting a spelling.
    pub(crate) split: Vec<&'static str>,
}

impl RawTable {
    /// The section `path` (empty for the top of the file) holding `entries`.
    #[must_use]
    pub fn new(path: impl Into<String>, entries: BTreeMap<String, RawValue>) -> Self {
        Self {
            path: path.into(),
            entries,
            split: Vec::new(),
        }
    }

    /// Moves the setting `name`, if present, into its own table of the same section, so that
    /// it can be parsed before the rest.
    #[must_use = "the setting is no longer in the original table"]
    pub fn split_off(&mut self, name: &'static str) -> Self {
        self.split.push(name);
        let entries = self.entries.remove_entry(name).into_iter().collect();
        Self::new(self.path.clone(), entries)
    }

    /// The problem with the first name that looks like a misspelling of `meant`, to show with
    /// "missing" when `meant` is required before the other names are checked.
    #[must_use]
    pub fn misspelling_of(&self, meant: &'static str) -> Option<Problem> {
        self.entries
            .keys()
            .filter(|name| name.as_str() != meant && !is_extension(name))
            .find(|name| closest(name, &[meant]).is_some())
            .map(|name| self.unknown(name, &[meant]))
    }

    /// Why `name`, which nothing reads, is refused, suggesting the closest of `known`.
    pub(crate) fn unknown(&self, name: &str, known: &[&'static str]) -> Problem {
        let key = self.written(name);
        let advice = closest(name, known).map_or_else(
            || format!("remove it, or start its name with `{EXTENSION_PREFIX}` to keep it"),
            |known| format!("did you mean `{}`?", self.written(known)),
        );
        let message = format!("unknown section or key `{key}`; {advice}");
        Problem::new(key, message)
    }

    /// `name` as a user would write it from the top of the file. The name is escaped, so that
    /// it cannot add lines to a message, and quoted if its dot could pass for a section.
    pub(crate) fn written(&self, name: &str) -> String {
        let escaped = name.escape_debug();
        if !self.path.is_empty() {
            format!("{}.{escaped}", self.path)
        } else if name.contains('.') {
            format!("\"{escaped}\"")
        } else {
            escaped.to_string()
        }
    }
}
