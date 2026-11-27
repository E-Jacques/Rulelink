//! `check`.

use std::fmt::Write as _;
use std::path::Path;

use clap::{Args, ValueEnum};
use rulelink_app::checks::Checked;
use rulelink_app::runtime::Outcome;
use rulelink_app::{Error, Finding, Severity};

use super::open;
use crate::presenter::Presenter;

/// Said before the Findings while the Source has no Rules.
const NO_RULES: &str = "The rules file has no Rules yet; the code is still scanned.";

/// The last line of a report: what a check does not prove.
const PROMISE: &str =
    "check verifies the link between a Rule and the code, not that the code is correct.";

/// Options of `check`.
#[derive(Debug, Args)]
pub struct CheckArgs {
    /// The report to print: lines for people, or JSON for programs.
    #[arg(long, value_enum, default_value_t = Format::Human)]
    format: Format,
}

/// The forms a report can take.
#[derive(Debug, Clone, Copy, ValueEnum)]
enum Format {
    Human,
    Json,
}

/// Prints the report; the outcome fails if it holds an error, whatever the format.
pub(super) fn run(args: &CheckArgs, out: &mut Presenter<'_>) -> Result<Outcome, Error> {
    let checked = rulelink_app::checks::run(&open(out)?)?;
    match args.format {
        Format::Human => print_human(&checked, out)?,
        Format::Json => print_json(&checked, out)?,
    }
    Ok(if checked.report.count(Severity::Error) > 0 {
        Outcome::ChecksFailed
    } else {
        Outcome::Success
    })
}

/// One line per Finding, the totals and what a check does not prove.
fn print_human(checked: &Checked, out: &mut Presenter<'_>) -> Result<(), Error> {
    let report = &checked.report;
    if report.no_rules {
        out.log(NO_RULES)?;
    }
    for finding in &report.findings {
        out.log(line(finding, &checked.source_path))?;
    }
    out.log(format_args!(
        "{}, {}",
        counted(report.count(Severity::Error), "error"),
        counted(report.count(Severity::Warning), "warning")
    ))?;
    out.log(PROMISE)
}

/// The published JSON report, and nothing else: programs parse the whole output.
fn print_json(checked: &Checked, out: &mut Presenter<'_>) -> Result<(), Error> {
    let json = serde_json::to_string_pretty(&checked.report_v1())
        .map_err(|e| Error::Internal(format!("cannot write the report: {e}")))?;
    out.log(escape_controls_in_json(&json))
}

/// Writes as `\u` escapes the control characters that JSON lets a string hold raw (DEL and
/// U+0080 to U+009F): a file name must not be able to drive the terminal. The parsed values
/// do not change. The only other raw control characters of `json` are its line breaks.
fn escape_controls_in_json(json: &str) -> String {
    let mut out = String::with_capacity(json.len());
    for c in json.chars() {
        if c.is_control() && c != '\n' {
            // Every control character is below U+00A0: four hexadecimal digits hold it.
            // Writing to a `String` cannot fail.
            let _ = write!(out, "\\u{:04x}", u32::from(c));
        } else {
            out.push(c);
        }
    }
    out
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
