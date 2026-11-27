//! From TOML text to the neutral settings the properties read.

use std::collections::BTreeMap;

use rulelink_adapter_sdk::{RawTable, RawValue};
use toml::de::{DeTable, DeValue};

/// The top of the file.
///
/// Parsed without converting numbers first, so that one too large to hold does not refuse
/// the whole file: it only fails where a property reads it.
///
/// # Errors
/// The parser's own error, which quotes the line at fault and its number.
pub(crate) fn root(text: &str) -> Result<RawTable, toml::de::Error> {
    let table = DeTable::parse(text)?.into_inner();
    Ok(RawTable::new("", entries(table)))
}

fn entries(table: DeTable<'_>) -> BTreeMap<String, RawValue> {
    table
        .into_iter()
        .map(|(name, value)| (name.into_inner().into_owned(), convert(value.into_inner())))
        .collect()
}

fn convert(value: DeValue<'_>) -> RawValue {
    match value {
        DeValue::String(text) => RawValue::String(text.into_owned()),
        DeValue::Integer(number) => {
            let digits = number.as_str().replace('_', "");
            i64::from_str_radix(&digits, number.radix())
                .map_or(RawValue::LargeInteger(digits), RawValue::Integer)
        }
        DeValue::Float(number) => {
            let digits = number.as_str().replace('_', "");
            // The parser already checked the syntax, which Rust reads the same way.
            RawValue::Float(digits.parse().unwrap_or(f64::NAN))
        }
        DeValue::Boolean(boolean) => RawValue::Boolean(boolean),
        DeValue::Datetime(datetime) => RawValue::Datetime(datetime.to_string()),
        DeValue::Array(array) => RawValue::Array(
            array
                .iter()
                .map(|item| convert(item.get_ref().clone()))
                .collect(),
        ),
        DeValue::Table(table) => RawValue::Table(entries(table)),
    }
}
