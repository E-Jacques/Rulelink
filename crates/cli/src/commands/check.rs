//! `check`.

use std::path::Path;

use rulelink_app::runtime::Outcome;
use rulelink_app::{Error, Finding, PROMISE, Severity};

use super::project_root;

/// Said before the Findings while the Source has no Rules.
const NO_RULES: &str = "The rules file has no Rules yet; the code is still scanned.";

/// Prints the report; the outcome fails if it holds an error.
pub(super) fn run() -> Result<Outcome, Error> {
    let checked = rulelink_app::checks::run(&project_root()?)?;
    let report = &checked.report;
    if report.no_rules {
        println!("{NO_RULES}");
    }
    for finding in &report.findings {
        println!("{}", line(finding, &checked.source_path));
    }
    let (errors, warnings) = (
        report.count(Severity::Error),
        report.count(Severity::Warning),
    );
    println!(
        "{}, {}",
        counted(errors, "error"),
        counted(warnings, "warning")
    );
    println!("{PROMISE}");
    Ok(if errors > 0 {
        Outcome::ChecksFailed
    } else {
        Outcome::Success
    })
}

/// `error RL001: message (file:line)`. A Finding with no Annotation points at the rules file.
fn line(finding: &Finding, source_path: &Path) -> String {
    let location = finding.location.as_ref().map_or_else(
        || source_path.display().to_string(),
        |at| format!("{}:{}", at.path, at.line),
    );
    on_one_line(&format!(
        "{} {}: {} ({location})",
        finding.severity,
        finding.problem.code().id(),
        finding.message()
    ))
}

/// Escapes control characters: a hand-edited rules file or a file name must not be able to
/// add lines to the report.
fn on_one_line(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_control() {
            out.extend(c.escape_default());
        } else {
            out.push(c);
        }
    }
    out
}

/// `1 error`, `0 errors`.
fn counted(count: usize, noun: &str) -> String {
    let plural = if count == 1 { "" } else { "s" };
    format!("{count} {noun}{plural}")
}
