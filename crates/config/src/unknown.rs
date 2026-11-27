//! The names of `rulelink.toml` that nothing reads.

// Only names are read, as maps to `IgnoredAny`: reading the values would refuse some that
// nothing uses, such as an oversized integer under `x-`.
#![allow(clippy::zero_sized_map_values)] // a table is read as a map, not as a set

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::de::IgnoredAny;

use crate::{Error, KNOWN_KEYS, RULES_KEYS, SCAN_KEYS, invalid};

/// Set aside for external adapters: not read yet, and not a mistake.
const ADAPTERS_KEY: &str = "adapters";
/// Starts a name reserved for a team's own notes and other tools.
const EXTENSION_PREFIX: &str = "x-";

/// A name of `rulelink.toml` that has no effect.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnknownKey {
    /// The section that holds the key; `None` for a top-level section or key.
    pub section: Option<&'static str>,
    /// The name as the file spells it, without its section. It may hold any character.
    pub name: String,
}

/// The names of a table.
type Names = BTreeMap<String, IgnoredAny>;

/// The names in the sections whose keys are checked.
#[derive(Default, Deserialize)]
struct SectionNames {
    #[serde(default)]
    rules: Names,
    #[serde(default)]
    scan: Names,
}

/// The names of `text` that nothing reads. Parsed apart from the configuration itself, as
/// collecting them there (`serde(flatten)`) would lose the line numbers of its errors.
pub(crate) fn unknown_keys(text: &str) -> Result<Vec<UnknownKey>, Error> {
    let top: Names = toml::from_str(text).map_err(|e| invalid(&e.to_string()))?;
    // The configuration also accepts a list where it expects a section, which has no names
    // to read. A warning must not refuse what the commands accept: give up on the sections.
    let sections: SectionNames = toml::from_str(text).unwrap_or_default();
    let mut unknown: Vec<UnknownKey> = unread(&top, &KNOWN_KEYS)
        .filter(|name| *name != ADAPTERS_KEY)
        .map(|name| key(None, name))
        .collect();
    for (section, names, known) in [
        ("rules", &sections.rules, RULES_KEYS),
        ("scan", &sections.scan, SCAN_KEYS),
    ] {
        unknown.extend(unread(names, &known).map(|name| key(Some(section), name)));
    }
    unknown.sort();
    Ok(unknown)
}

fn key(section: Option<&'static str>, name: &str) -> UnknownKey {
    UnknownKey {
        section,
        name: name.to_owned(),
    }
}

/// The `names` that are neither `known` nor reserved by `x-`.
fn unread<'a>(names: &'a Names, known: &'a [&str]) -> impl Iterator<Item = &'a str> {
    names
        .keys()
        .map(String::as_str)
        .filter(|name| !known.contains(name) && !name.starts_with(EXTENSION_PREFIX))
}
