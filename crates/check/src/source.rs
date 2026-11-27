//! The Findings about the Source itself.

use std::collections::{BTreeMap, HashSet};

use rulelink_model::{Rule, RuleId, RuleSet, RuleStatus};

use crate::{IdentifierAt, Problem, Raw};

/// What is wrong with the Source as read: `text` is the file `rules` was parsed from.
pub(crate) fn findings(rules: &RuleSet, text: &str) -> Vec<Raw> {
    let not_canonical =
        (!rules.is_canonical(text)).then(|| Raw::in_rules_file(None, Problem::NotCanonical));
    duplicates(rules)
        .chain(invalid_identifiers(rules))
        .chain(reused_retired(rules))
        .chain(unknown_related(rules))
        .chain(empty_fields(rules))
        .chain(not_canonical)
        .collect()
}

/// The Identifier of a Finding about `id`: none if there is nothing to name.
fn named(id: &RuleId) -> Option<RuleId> {
    (!is_blank(&id.0)).then(|| id.clone())
}

fn is_blank(text: &str) -> bool {
    text.trim().is_empty()
}

/// One per Identifier that several Rules have. A blank one is reported as empty instead.
fn duplicates(rules: &RuleSet) -> impl Iterator<Item = Raw> {
    let mut count: BTreeMap<&str, usize> = BTreeMap::new();
    for rule in rules.rules.iter().filter(|r| !is_blank(&r.id.0)) {
        *count.entry(&rule.id.0).or_default() += 1;
    }
    count
        .into_iter()
        .filter(|(_, rules)| *rules > 1)
        .map(|(id, rules)| {
            Raw::in_rules_file(Some(RuleId(id.to_owned())), Problem::DuplicateId { rules })
        })
}

/// Every place of the Source that holds a malformed Identifier. A blank `id` is a missing
/// required field, reported as empty instead.
fn invalid_identifiers(rules: &RuleSet) -> impl Iterator<Item = Raw> + '_ {
    let retired = rules.retired.iter().map(|id| (id, IdentifierAt::Retired));
    rules
        .rules
        .iter()
        .flat_map(identifiers_of)
        .chain(retired)
        .filter(|(id, _)| !id.is_well_formed())
        .map(|(id, at)| {
            let value = id.0.clone();
            Raw::in_rules_file(named(id), Problem::InvalidIdentifier { value, at })
        })
}

/// The Identifiers one Rule holds, with where it holds them.
fn identifiers_of(rule: &Rule) -> impl Iterator<Item = (&RuleId, IdentifierAt)> {
    let holder = named(&rule.id);
    let own = holder.is_some().then_some((&rule.id, IdentifierAt::Id));
    let related = rule.related.iter().map({
        let holder = holder.clone();
        move |id| (id, IdentifierAt::Related(holder.clone()))
    });
    let replacement = rule
        .superseded_by
        .iter()
        .map(move |id| (id, IdentifierAt::SupersededBy(holder.clone())));
    own.into_iter().chain(related).chain(replacement)
}

/// The Rules in use whose Identifier is retired.
fn reused_retired(rules: &RuleSet) -> impl Iterator<Item = Raw> + '_ {
    let retired: HashSet<&RuleId> = rules.retired.iter().collect();
    rules
        .rules
        .iter()
        .filter(|r| matches!(r.status, RuleStatus::Proposed | RuleStatus::Validated))
        .filter(move |r| !is_blank(&r.id.0) && retired.contains(&r.id))
        .map(|r| {
            let problem = Problem::ReusedRetiredId { status: r.status };
            Raw::in_rules_file(Some(r.id.clone()), problem)
        })
}

/// The `related` entries that no Rule has. A blank one is reported as malformed only.
fn unknown_related(rules: &RuleSet) -> impl Iterator<Item = Raw> + '_ {
    let known: HashSet<&RuleId> = rules.rules.iter().map(|r| &r.id).collect();
    rules
        .rules
        .iter()
        .flat_map(|rule| rule.related.iter().map(move |related| (rule, related)))
        .filter(move |(_, related)| !is_blank(&related.0) && !known.contains(related))
        .map(|(rule, related)| {
            let problem = Problem::UnknownRelated {
                related: related.clone(),
            };
            Raw::in_rules_file(named(&rule.id), problem)
        })
}

/// The required fields that hold nothing.
fn empty_fields(rules: &RuleSet) -> impl Iterator<Item = Raw> + '_ {
    rules.rules.iter().flat_map(|rule| {
        let kind = rule.origin.as_ref().map(|origin| origin.kind.as_str());
        [
            ("id", Some(rule.id.0.as_str())),
            ("title", Some(rule.title.as_str())),
            ("statement", Some(rule.statement.as_str())),
            ("origin.kind", kind),
        ]
        .into_iter()
        .filter(|(_, value)| value.is_some_and(is_blank))
        .map(|(field, _)| {
            let title = (!is_blank(&rule.title)).then(|| rule.title.clone());
            Raw::in_rules_file(named(&rule.id), Problem::EmptyField { field, title })
        })
    })
}
