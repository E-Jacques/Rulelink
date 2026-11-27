//! Link use cases: where the code cites a Rule.

use rulelink_link::Location;
use rulelink_model::{RuleId, RuleStatus};

use crate::{Error, Workspace};

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
/// [`Error::Input`] if the Source holds no Rule `id`, [`Error::Config`] if the exclusion
/// patterns cannot be combined, [`Error::Source`] for a missing, invalid or unsupported
/// Source, [`Error::Internal`] if a file cannot be read.
pub fn locate(project: &Workspace, id: &RuleId) -> Result<Located, Error> {
    let (_, rule_set) = crate::rules::read_source(project)?;
    let rule = rule_set
        .find(id)
        .ok_or_else(|| Error::Input(format!("{id} is not a Rule of this project")))?;
    let index = rulelink_adapter_scan_fs::scan(
        project.root(),
        project.scan_exclude()?,
        project.source_path()?,
    )?;
    Ok(Located {
        status: rule.status,
        locations: index.locations(id),
    })
}
