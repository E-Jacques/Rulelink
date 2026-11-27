//! Adapter contract: traits, protocol types and conformance kit.
//!
//! Each adapter parses and validates its own settings: it declares one [`Property`] for each,
//! runs them through a [`TableParser`] and exposes the result as an [`AdapterConfig`].

mod error;
mod map;
mod path;
mod report;
mod suggest;
mod table;
mod value;

pub use error::Error;
pub use map::{ConfigMap, Property, UndeclaredProperty};
pub use path::{OutsideProject, ProjectPath};
pub use report::{ParseError, ParseResult, Parsed, Problem, Warning};
pub use table::{TableParser, combine};
pub use value::{RawTable, RawValue, is_extension, required_text};

/// The settings of one adapter.
///
/// ```
/// use rulelink_adapter_sdk::{AdapterConfig, ConfigMap, ParseResult, RawTable, TableParser};
///
/// struct Memory;
///
/// impl AdapterConfig for Memory {
///     fn kind(&self) -> &'static str {
///         "memory"
///     }
///
///     fn parse(&self, table: RawTable) -> ParseResult<ConfigMap> {
///         TableParser::new(table).finish()
///     }
/// }
/// ```
pub trait AdapterConfig: Sync {
    /// The name a user selects the adapter by.
    fn kind(&self) -> &'static str;

    /// Parses and validates the adapter's section, refusing the names it does not know.
    ///
    /// # Errors
    /// [`ParseError`] holding every problem found.
    fn parse(&self, table: RawTable) -> ParseResult<ConfigMap>;
}
