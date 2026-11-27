//! The Findings about the links between Rules and code.

use rulelink_link::LinkIndex;
use rulelink_model::{Rule, RuleId, RuleSet, RuleStatus};

use crate::{Place, Problem, Raw};

/// What is wrong between `rules` and the Annotations of `links`.
pub(crate) fn findings<'a>(
    rules: &'a RuleSet,
    links: &'a LinkIndex,
) -> impl Iterator<Item = Raw> + 'a {
    citations(rules, links).chain(orphans(rules, links))
}

/// What is wrong at each location that cites an Identifier.
fn citations<'a>(rules: &'a RuleSet, links: &'a LinkIndex) -> impl Iterator<Item = Raw> + 'a {
    links.cited().flat_map(move |id| {
        let problem = problem_of_citing(rules, id);
        links.locations(id).into_iter().filter_map(move |at| {
            Some(Raw {
                rule: Some(id.clone()),
                problem: problem.clone()?,
                place: Place::Code(at),
            })
        })
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
        .map(|rule| Raw::in_rules_file(Some(rule.id.clone()), Problem::Orphan))
}
