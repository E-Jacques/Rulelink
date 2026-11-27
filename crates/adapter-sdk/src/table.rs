use crate::{
    ConfigMap, ParseError, ParseResult, Parsed, Problem, Property, RawTable, RawValue, is_extension,
};

/// Joins two parses: every problem of both, or one map holding the settings of both.
///
/// ```
/// use std::collections::BTreeMap;
/// use rulelink_adapter_sdk::{RawTable, TableParser, combine};
///
/// let empty = || TableParser::new(RawTable::new("", BTreeMap::new())).finish();
/// assert!(combine(empty(), empty()).is_ok());
/// ```
///
/// # Errors
/// The problems of `first` followed by those of `second`.
pub fn combine(
    first: ParseResult<ConfigMap>,
    second: ParseResult<ConfigMap>,
) -> ParseResult<ConfigMap> {
    match (first, second) {
        (Ok(mut first), Ok(second)) => {
            first.configuration.merge(second.configuration);
            first.warnings.extend(second.warnings);
            Ok(first)
        }
        (Err(mut first), Err(second)) => {
            first.append(second);
            Err(first)
        }
        (Err(error), Ok(_)) | (Ok(_), Err(error)) => Err(error),
    }
}

/// Parses one section: runs every declared property, even after one fails, then refuses the
/// names nothing claimed. Names starting with `x-` are accepted.
///
/// ```
/// # use std::collections::BTreeMap;
/// # use rulelink_adapter_sdk::{ParseResult, Parsed, Property, RawTable, RawValue, TableParser};
/// # #[derive(Debug)] struct Path;
/// # impl Property for Path {
/// #     const KEY: &'static str = "source.path";
/// #     fn parse(_: Option<&RawValue>) -> ParseResult<Self> { Ok(Parsed::new(Path)) }
/// # }
/// let entries = BTreeMap::from([("pat".to_owned(), RawValue::Boolean(true))]);
/// let error = TableParser::new(RawTable::new("source", entries))
///     .property::<Path>()
///     .finish()
///     .expect_err("unknown name");
/// assert_eq!(
///     error.to_string(),
///     "unknown section or key `source.pat`; did you mean `source.path`?"
/// );
/// ```
#[must_use]
pub struct TableParser {
    table: RawTable,
    known: Vec<&'static str>,
    result: ParseResult<ConfigMap>,
}

impl TableParser {
    /// A parser for `table`.
    pub fn new(table: RawTable) -> Self {
        Self {
            known: table.split.clone(),
            table,
            result: Ok(Parsed::new(ConfigMap::new())),
        }
    }

    /// Declares the property `P`, named by the last part of its key. The rest of the key
    /// must name this section.
    ///
    /// # Panics
    /// In a debug build, if `P::KEY` belongs to another section: it would silently read a
    /// setting its messages do not name. A test that parses the section catches it.
    pub fn property<P: Property>(mut self) -> Self {
        let name = P::KEY.rsplit('.').next().unwrap_or(P::KEY);
        debug_assert_eq!(
            P::KEY,
            self.table.written(name),
            "the property is declared in the wrong section"
        );
        let raw = self.claim(name);
        let parsed = P::parse(raw.as_ref()).map(|parsed| {
            let mut map = ConfigMap::new();
            map.insert(parsed.configuration);
            Parsed {
                configuration: map,
                warnings: parsed.warnings,
            }
        });
        self.add(parsed)
    }

    /// Declares the sub-section `name`, read by `parse`. An absent one is parsed as empty, so
    /// that its properties decide what is required.
    pub fn section(
        mut self,
        name: &'static str,
        parse: impl FnOnce(RawTable) -> ParseResult<ConfigMap>,
    ) -> Self {
        let path = self.table.written(name);
        let parsed = match self.claim(name) {
            None => parse(RawTable::new(path, std::collections::BTreeMap::new())),
            Some(RawValue::Table(entries)) => parse(RawTable::new(path, entries)),
            Some(other) => {
                let message = format!("`{path}` must be a section, found {}", other.kind_name());
                Err(ParseError::new(Problem::new(path, message)))
            }
        };
        self.add(parsed)
    }

    /// Accepts `name` and whatever it holds, without reading it.
    pub fn reserved(mut self, name: &'static str) -> Self {
        self.claim(name);
        self
    }

    /// The settings, if nothing was refused.
    ///
    /// # Errors
    /// Every problem of the declared settings, then one for each unknown name.
    pub fn finish(mut self) -> ParseResult<ConfigMap> {
        let unknown: Vec<Problem> = self
            .table
            .entries
            .keys()
            .filter(|name| !is_extension(name))
            .map(|name| self.table.unknown(name, &self.known))
            .collect();
        for problem in unknown {
            self = self.add(Err(ParseError::new(problem)));
        }
        self.result
    }

    fn claim(&mut self, name: &'static str) -> Option<RawValue> {
        self.known.push(name);
        self.table.entries.remove(name)
    }

    fn add(mut self, parsed: ParseResult<ConfigMap>) -> Self {
        self.result = combine(self.result, parsed);
        self
    }
}
