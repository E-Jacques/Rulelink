//! The published form of a [`Report`], for programs to read.
//!
//! These types are the contract of `schemas/check-report.schema.json`, apart from [`Finding`]
//! so that the engine can change without changing what is published. Within one version, codes
//! and fields may be added; removing, renaming or retyping a field needs a new version
//! (`docs/adr/0006-json-reports-open-within-a-version.md`).

use std::path::{Component, Path};

use serde::Serialize;

use crate::{Finding, Place, Report, Severity};

/// The value of `schema` in [`CheckReportV1`].
const SCHEMA_V1: &str = "rulelink/check-report/v1";

/// What the published schema says of the report. The doc comment of [`CheckReportV1`] is for
/// Rust callers.
#[cfg(feature = "schema")]
const SCHEMA_DESCRIPTION: &str = "The Findings printed by `rulelink check --format json`. \
It shows that links exist between Rules and code, not that the code is correct. Later releases \
may add fields and Finding codes: ignore what you do not know.";

/// The Findings of one check, as `rulelink check --format json` prints them.
///
/// ```
/// use std::path::Path;
///
/// use rulelink_check::{CheckReportV1, Levels, Source, check};
/// use rulelink_link::LinkIndex;
/// use rulelink_model::RuleSet;
///
/// let rules = RuleSet::empty();
/// let text = rules.to_canonical();
/// let source = Source::Rules { rules: &rules, text: &text };
/// let report = check(source, &LinkIndex::new(), &Levels::default());
///
/// let published = CheckReportV1::new(&report, Path::new("rules/rules.json"));
///
/// assert_eq!(
///     serde_json::to_string(&published).expect("serialisable"),
///     r#"{"schema":"rulelink/check-report/v1","findings":[],"totals":{"errors":0,"warnings":0}}"#
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schema",
    schemars(title = "CheckReport", description = SCHEMA_DESCRIPTION)
)]
pub struct CheckReportV1 {
    /// The version of this report.
    #[cfg_attr(feature = "schema", schemars(extend("const" = SCHEMA_V1)))]
    schema: &'static str,
    /// Errors first, then by code, Identifier, file and line.
    findings: Vec<FindingV1>,
    /// How many Findings are reported.
    totals: TotalsV1,
}

impl CheckReportV1 {
    /// The published form of `report`. `source_path` is the rules file, relative to the
    /// project root: it is the file of a Finding that no line of code is at fault for.
    /// It is not checked here: pass a path without a root or `..`, as the configuration
    /// guarantees.
    #[must_use]
    pub fn new(report: &Report, source_path: &Path) -> Self {
        let source_file = slashed(source_path);
        Self {
            schema: SCHEMA_V1,
            findings: report
                .findings
                .iter()
                .map(|finding| FindingV1::new(finding, &source_file))
                .collect(),
            totals: TotalsV1 {
                errors: report.count(Severity::Error),
                warnings: report.count(Severity::Warning),
            },
        }
    }
}

/// One Finding. A field that does not apply is left out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "Finding"))]
struct FindingV1 {
    /// The code, such as `RL001`. Rely on `severity` and `message` for an unknown one.
    #[cfg_attr(feature = "schema", schemars(regex(pattern = r"^RL[0-9]{3}$")))]
    code: &'static str,
    /// The name that sets the level of the Finding, such as `unknown-reference`.
    #[cfg_attr(
        feature = "schema",
        schemars(regex(pattern = r"^[a-z0-9]+(-[a-z0-9]+)*$"))
    )]
    name: &'static str,
    /// An error makes the check fail.
    severity: Severity,
    /// One sentence saying what is wrong.
    message: String,
    /// The Identifier the Finding is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "String"))]
    rule: Option<String>,
    /// The file at fault, from the project root and `/`-separated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "String"))]
    file: Option<String>,
    /// The line at fault in `file`, from 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "u32", range(min = 1)))]
    line: Option<u32>,
}

impl FindingV1 {
    fn new(finding: &Finding, source_file: &str) -> Self {
        let code = finding.problem.code();
        let (file, line) = match &finding.place {
            Place::Code(at) => (at.path.clone(), Some(at.line)),
            Place::RulesFile { line } => (source_file.to_owned(), *line),
        };
        Self {
            code: code.id(),
            name: code.name(),
            severity: finding.severity,
            message: finding.message(),
            rule: finding.rule.as_ref().map(|id| id.0.clone()),
            file: Some(file),
            line,
        }
    }
}

/// How many Findings are reported, by severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "Totals"))]
struct TotalsV1 {
    /// The number of errors.
    errors: usize,
    /// The number of warnings.
    warnings: usize,
}

/// A path inside the project, written like the paths of the scanned files: `/`-separated,
/// without `.` segments.
fn slashed(path: &Path) -> String {
    path.components()
        .filter(|part| !matches!(part, Component::CurDir))
        .map(|part| part.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
