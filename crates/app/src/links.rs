//! Link use cases: where the code cites a Rule.

use std::path::Path;

use rulelink_link::Location;
use rulelink_model::{RuleId, RuleStatus};

use crate::Error;

/// A Rule and the places that cite it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Located {
    /// Status of the Rule in the Source.
    pub status: RuleStatus,
    /// The Annotations that cite it, sorted by path then line.
    pub locations: Vec<Location>,
}

/// Finds the code locations that cite `id`.
///
/// Scans only if the Rule exists, so an unknown Identifier fails before any file is read.
///
/// # Errors
/// [`Error::Input`] if the Source holds no Rule `id`, [`Error::Config`] for a missing or
/// invalid `rulelink.toml` or an invalid `scan.exclude` pattern, [`Error::Source`] for a
/// missing, invalid or unsupported Source, [`Error::Internal`] if a file cannot be read.
pub fn locate(root: &Path, id: &RuleId) -> Result<Located, Error> {
    let (config, _, rule_set) = crate::rules::read_source(root)?;
    let rule = rule_set
        .find(id)
        .ok_or_else(|| Error::Input(format!("{id} is not a Rule of this project")))?;
    let index = rulelink_adapter_scan_fs::scan(root, &config.scan_exclude)?;
    Ok(Located {
        status: rule.status,
        locations: index.locations(id),
    })
}
