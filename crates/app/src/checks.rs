//! Check use case: the Findings of a project.

use std::path::{Path, PathBuf};

use rulelink_check::Report;

use crate::Error;

/// Result of [`run`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checked {
    /// The Source, as `rulelink.toml` names it (relative to the project root).
    pub source_path: PathBuf,
    /// The Findings.
    pub report: Report,
}

/// Reports the Findings of the project at `root`. Reads only.
///
/// # Errors
/// [`Error::Config`] for a missing or invalid `rulelink.toml` or an invalid `scan.exclude`
/// pattern, [`Error::Source`] for a missing, invalid or unsupported Source,
/// [`Error::Internal`] if a file cannot be read.
pub fn run(root: &Path) -> Result<Checked, Error> {
    let (config, _, rule_set) = crate::rules::read_source(root)?;
    let links = rulelink_adapter_scan_fs::scan(root, &config.scan_exclude)?;
    Ok(Checked {
        report: rulelink_check::check(&rule_set, &links, &config.check_levels),
        source_path: config.source_path,
    })
}
