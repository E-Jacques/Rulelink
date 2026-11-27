//! Check use case: the Findings of a project.

use std::path::PathBuf;

use rulelink_check::Report;

use crate::{Error, Workspace};

/// Result of [`run`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checked {
    /// The Source, as `rulelink.toml` names it (relative to the project root).
    pub source_path: PathBuf,
    /// The Findings.
    pub report: Report,
}

/// Reports the Findings of the project. Reads only.
///
/// # Errors
/// [`Error::Config`] if the exclusion patterns cannot be combined, [`Error::Source`] for a
/// missing, invalid or unsupported Source, [`Error::Internal`] if a file cannot be read.
pub fn run(project: &Workspace) -> Result<Checked, Error> {
    let (_, rule_set) = crate::rules::read_source(project)?;
    let source_path = project.source_path()?;
    let links =
        rulelink_adapter_scan_fs::scan(project.root(), project.scan_exclude()?, source_path)?;
    Ok(Checked {
        report: rulelink_check::check(&rule_set, &links, project.check_levels()?),
        source_path: source_path.to_path_buf(),
    })
}
