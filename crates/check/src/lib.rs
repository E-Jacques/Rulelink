//! Checks: the Findings about the links between Rules and code.
//!
//! Pure: [`check`] reads a [`RuleSet`] and a [`LinkIndex`] and does no I/O. It verifies that a
//! link exists, not that the code is correct.

mod error;
mod finding;
mod level;

use rulelink_link::{LinkIndex, Location};
use rulelink_model::{Rule, RuleId, RuleSet, RuleStatus};

pub use error::{UnknownFinding, UnknownLevel};
pub use finding::{Code, Finding, Problem};
pub use level::{Level, Levels, Severity};

/// What a report says about the limits of a check.
pub const PROMISE: &str =
    "check verifies the link between a Rule and the code, not that the code is correct.";

/// The result of [`check`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// Errors first, then by code, Identifier, path and line.
    pub findings: Vec<Finding>,
    /// Whether the Source has no Rules yet.
    pub no_rules: bool,
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

/// Compares every Rule of `rules` with every Annotation of `links`.
///
/// There is one Finding per location that cites an Identifier wrongly, and one per
/// `validated` Rule that nothing cites.
///
/// ```
/// use rulelink_check::{Code, Levels, check};
/// use rulelink_link::LinkIndex;
/// use rulelink_model::RuleSet;
///
/// let mut links = LinkIndex::new();
/// links.add_file("src/billing.rs", "// @rule RULE-099\n");
///
/// let report = check(&RuleSet::empty(), &links, &Levels::default());
///
/// assert_eq!(report.findings[0].problem.code(), Code::UnknownReference);
/// assert_eq!(report.findings[0].message(), "RULE-099 is not a Rule of this project");
/// ```
#[must_use]
pub fn check(rules: &RuleSet, links: &LinkIndex, levels: &Levels) -> Report {
    let no_rules = rules.rules.is_empty();
    let mut findings: Vec<Finding> = citations(rules, links)
        .chain(orphans(rules, links))
        .filter_map(|(rule, problem, location)| {
            let severity = levels.of(problem.code(), no_rules).severity()?;
            Some(Finding {
                severity,
                rule,
                problem,
                location,
            })
        })
        .collect();
    findings.sort_by(|a, b| {
        (a.severity, a.problem.code())
            .cmp(&(b.severity, b.problem.code()))
            .then_with(|| a.rule.natural_cmp(&b.rule))
            // `RULE-5` and `RULE-05` are equal above: without this, their order would follow
            // the index's hash map and change from run to run.
            .then_with(|| a.rule.0.cmp(&b.rule.0))
            .then_with(|| a.location.cmp(&b.location))
    });
    Report { findings, no_rules }
}

/// A Finding before the team's levels apply.
type Raw = (RuleId, Problem, Option<Location>);

/// What is wrong at each location that cites an Identifier.
fn citations<'a>(rules: &'a RuleSet, links: &'a LinkIndex) -> impl Iterator<Item = Raw> + 'a {
    links.cited().flat_map(move |id| {
        let problem = problem_of_citing(rules, id);
        links
            .locations(id)
            .into_iter()
            .filter_map(move |at| Some((id.clone(), problem.clone()?, Some(at))))
    })
}

/// What is wrong with citing `id`, if anything. A Rule in the Source decides by its status;
/// `retired` only speaks for an Identifier that has no Rule.
fn problem_of_citing(rules: &RuleSet, id: &RuleId) -> Option<Problem> {
    let Some(rule) = rules.find(id) else {
        return Some(if rules.retired.contains(id) {
            Problem::Retired
        } else {
            Problem::Unknown
        });
    };
    match rule.status {
        RuleStatus::Validated => None,
        RuleStatus::Proposed => Some(Problem::Unvalidated),
        RuleStatus::Rejected => Some(Problem::Rejected),
        RuleStatus::Obsolete => Some(Problem::Obsolete {
            superseded_by: rule.superseded_by.clone(),
        }),
    }
}

/// The `validated` Rules that nothing cites.
fn orphans<'a>(rules: &'a RuleSet, links: &'a LinkIndex) -> impl Iterator<Item = Raw> + 'a {
    let uncited =
        |rule: &&Rule| rule.status == RuleStatus::Validated && links.locations(&rule.id).is_empty();
    rules
        .rules
        .iter()
        .filter(uncited)
        .map(|rule| (rule.id.clone(), Problem::Orphan, None))
}
