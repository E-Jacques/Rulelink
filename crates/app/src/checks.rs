//! Check use case: the Findings of a project.

use std::path::PathBuf;

use rulelink_adapter_json::{Inspected, WrongShape};
use rulelink_check::{CheckReportV1, Report, Source};

use crate::{Error, Workspace};

/// Result of [`run`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checked {
    /// The Source, as `rulelink.toml` names it (relative to the project root).
    pub source_path: PathBuf,
    /// The Findings.
    pub report: Report,
}

impl Checked {
    /// The report in the form published for programs to read.
    #[must_use]
    pub fn report_v1(&self) -> CheckReportV1 {
        CheckReportV1::new(&self.report, &self.source_path)
    }
}

/// Reports the Findings of the project. Reads only.
///
/// A Source of the wrong shape is a Finding, not an error.
///
/// # Errors
/// [`Error::Config`] if the exclusion patterns cannot be combined, [`Error::Source`] for a
/// Source that is missing, unreadable, not JSON or of an unsupported format,
/// [`Error::Internal`] if a file cannot be read.
pub fn run(project: &Workspace) -> Result<Checked, Error> {
    let source_path = project.source_path()?;
    let inspected = rulelink_adapter_json::inspect_source(&project.root().join(source_path))?;
    let links =
        rulelink_adapter_scan_fs::scan(project.root(), project.scan_exclude()?, source_path)?;
    let source = match &inspected {
        Inspected::Rules { rules, text } => Source::Rules { rules, text },
        Inspected::WrongShape(WrongShape { detail, line }) => Source::WrongShape {
            detail,
            line: *line,
        },
    };
    Ok(Checked {
        report: rulelink_check::check(source, &links, project.check_levels()?),
        source_path: source_path.to_path_buf(),
    })
}
