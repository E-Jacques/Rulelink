//! The names of `rulelink.toml` that nothing reads.

// Only names are read, as maps to `IgnoredAny`: reading the values would refuse some that
// nothing uses, such as an oversized integer under `x-`.
#![allow(clippy::zero_sized_map_values)] // a table is read as a map, not as a set

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::de::IgnoredAny;

use crate::{Error, invalid};

/// The top-level names of `rulelink.toml` this tool reads.
const KNOWN_KEYS: [&str; 5] = ["version", "source", "rules", "scan", "check"];
/// The keys read in `[rules]`.
const RULES_KEYS: [&str; 1] = ["prefix"];
/// The keys read in `[scan]`.
const SCAN_KEYS: [&str; 1] = ["exclude"];
/// Set aside for external adapters: not read yet, and not a mistake.
const ADAPTERS_KEY: &str = "adapters";
/// Starts a name reserved for a team's own notes and other tools.
const EXTENSION_PREFIX: &str = "x-";

/// The names of a table.
type Names = BTreeMap<String, IgnoredAny>;

/// The names in the sections whose keys are checked.
#[derive(Deserialize)]
struct SectionNames {
    #[serde(default)]
    rules: Names,
    #[serde(default)]
    scan: Names,
}

/// The names of `text` that nothing reads. Parsed apart from the configuration itself, as
/// collecting them there (`serde(flatten)`) would lose the line numbers of its errors.
pub(crate) fn unknown_keys(text: &str) -> Result<Vec<String>, Error> {
    let parse_failed = |e: toml::de::Error| invalid(&e.to_string());
    let top: Names = toml::from_str(text).map_err(parse_failed)?;
    let sections: SectionNames = toml::from_str(text).map_err(parse_failed)?;
    let mut unknown: Vec<String> = unread(&top, &KNOWN_KEYS)
        .filter(|name| *name != ADAPTERS_KEY)
        .map(str::to_owned)
        .collect();
    for (section, names, known) in [
        ("rules", &sections.rules, RULES_KEYS),
        ("scan", &sections.scan, SCAN_KEYS),
    ] {
        unknown.extend(unread(names, &known).map(|key| format!("{section}.{key}")));
    }
    unknown.sort();
    Ok(unknown)
}

/// The `names` that are neither `known` nor reserved by `x-`.
fn unread<'a>(names: &'a Names, known: &'a [&str]) -> impl Iterator<Item = &'a str> {
    names
        .keys()
        .map(String::as_str)
        .filter(|name| !known.contains(name) && !name.starts_with(EXTENSION_PREFIX))
}
