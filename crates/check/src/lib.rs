//! Checks: the Findings about the Source, and about the links between Rules and code.
//!
//! Pure: [`check`] reads a [`Source`] and a [`LinkIndex`] and does no I/O. It verifies that a
//! link exists, not that the code is correct.

mod error;
mod finding;
mod level;
mod links;
mod report;
mod source;

use std::cmp::Ordering;

use rulelink_link::LinkIndex;
use rulelink_model::{RuleId, RuleSet};

pub use error::{FixedLevel, UnknownFinding, UnknownLevel};
pub use finding::{Code, Finding, IdentifierAt, Place, Problem};
pub use level::{Level, Levels, Severity};
pub use report::CheckReportV1;

/// The Source as it was read.
#[derive(Debug, Clone, Copy)]
pub enum Source<'a> {
    /// A Source that could be read.
    Rules {
        /// Its content.
        rules: &'a RuleSet,
        /// The file `rules` was read from, as written.
        text: &'a str,
    },
    /// A file that is JSON, but not of the shape of a Source.
    WrongShape {
        /// What does not fit.
        detail: &'a str,
        /// The line at fault, from 1, when it is known.
        line: Option<u32>,
    },
}

/// How far [`check`] could compare the Rules with the code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeCheck {
    /// The code was checked.
    Run,
    /// The code was checked, but the Source has no Rules yet.
    RunWithoutRules,
    /// The code was not checked: the Source has errors.
    Stopped,
}

/// The result of [`check`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// Errors first, then by code, Identifier, path and line.
    pub findings: Vec<Finding>,
    /// Whether the code was compared with the Rules.
    pub code_check: CodeCheck,
}

impl Report {
    /// How many Findings have `severity`.
    #[must_use]
    pub fn count(&self, severity: Severity) -> usize {
        self.findings
            .iter()
            .filter(|f| f.severity == severity)
            .count()
    }
}

/// Checks the Source, then compares every Rule with every Annotation of `links`.
///
/// An error in the Source stops there: Rules that cannot be trusted are not compared with the
/// code. Otherwise there is one Finding per location that cites an Identifier wrongly, and
/// one per `validated` Rule that nothing cites.
///
/// ```
/// use rulelink_check::{Code, Levels, Source, check};
/// use rulelink_link::LinkIndex;
/// use rulelink_model::RuleSet;
///
/// let rules = RuleSet::empty();
/// let text = rules.to_canonical();
/// let mut links = LinkIndex::new();
/// links.add_file("src/billing.rs", "// @rule RULE-099\n");
///
/// let source = Source::Rules { rules: &rules, text: &text };
/// let report = check(source, &links, &Levels::default());
///
/// assert_eq!(report.findings[0].problem.code(), Code::UnknownReference);
/// assert_eq!(report.findings[0].message(), "RULE-099 is not a Rule of this project");
/// ```
#[must_use]
pub fn check(source: Source<'_>, links: &LinkIndex, levels: &Levels) -> Report {
    let (mut raw, rules) = match source {
        Source::Rules { rules, text } => (source::findings(rules, text), Some(rules)),
        Source::WrongShape { detail, line } => (vec![Raw::wrong_shape(detail, line)], None),
    };
    let trusted = rules.filter(|_| !raw.iter().any(|r| r.problem.code().stops_code_check()));
    let code_check = match trusted {
        None => CodeCheck::Stopped,
        Some(rules) if rules.rules.is_empty() => CodeCheck::RunWithoutRules,
        Some(_) => CodeCheck::Run,
    };
    if let Some(rules) = trusted {
        raw.extend(links::findings(rules, links));
    }
    Report {
        findings: weighed(raw, levels, code_check),
        code_check,
    }
}

/// The Findings to report, each with its severity, in report order and without repeats.
fn weighed(raw: Vec<Raw>, levels: &Levels, code_check: CodeCheck) -> Vec<Finding> {
    let mut findings: Vec<Finding> = raw
        .into_iter()
        .filter_map(|raw| {
            let code = raw.problem.code();
            let level = levels
                .get(code)
                .unwrap_or_else(|| default_level(code, code_check));
            Some(Finding {
                severity: level.severity()?,
                rule: raw.rule,
                problem: raw.problem,
                place: raw.place,
            })
        })
        .collect();
    // Stable: Findings that are equal here keep the order of the Source.
    findings.sort_by(report_order);
    findings
}

fn report_order(a: &Finding, b: &Finding) -> Ordering {
    (a.severity, a.problem.code())
        .cmp(&(b.severity, b.problem.code()))
        .then_with(|| identifier_order(a.rule.as_ref(), b.rule.as_ref()))
        .then_with(|| a.place.cmp(&b.place))
}

/// Findings that name no Identifier come first.
fn identifier_order(a: Option<&RuleId>, b: Option<&RuleId>) -> Ordering {
    match (a, b) {
        (Some(a), Some(b)) => a
            .natural_cmp(b)
            // `RULE-5` and `RULE-05` are equal above: without this, their order would follow
            // the index's hash map and change from run to run.
            .then_with(|| a.0.cmp(&b.0)),
        _ => a.is_some().cmp(&b.is_some()),
    }
}

/// The level of a Finding the team has not set.
fn default_level(code: Code, code_check: CodeCheck) -> Level {
    match code {
        // Annotations written before the first Rule must not fail a build.
        Code::UnknownReference if code_check == CodeCheck::RunWithoutRules => Level::Warning,
        Code::UnknownReference
        | Code::RetiredReference
        | Code::DuplicateId
        | Code::InvalidShape
        | Code::InvalidIdentifier
        | Code::ReusedRetiredId
        | Code::EmptyField => Level::Error,
        // Citations of a replaced Rule are moved step by step.
        Code::ObsoleteReference
        | Code::OrphanRule
        | Code::UnvalidatedReference
        // Neither changes which Rule a citation means.
        | Code::UnknownRelated
        | Code::NotCanonical => Level::Warning,
    }
}

/// A Finding before the team's levels apply.
struct Raw {
    rule: Option<RuleId>,
    problem: Problem,
    place: Place,
}

impl Raw {
    fn in_rules_file(rule: Option<RuleId>, problem: Problem) -> Self {
        Self {
            rule,
            problem,
            place: Place::RulesFile { line: None },
        }
    }

    fn wrong_shape(detail: &str, line: Option<u32>) -> Self {
        let problem = Problem::WrongShape {
            detail: detail.to_owned(),
        };
        Self {
            rule: None,
            problem,
            place: Place::RulesFile { line },
        }
    }
}
