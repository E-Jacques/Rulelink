use std::cmp::Ordering;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Error, IdPrefix, LifecycleError, Rule, RuleDraft, RuleId, RuleStatus};

/// The `format` value of the current Source layout.
pub const FORMAT_VERSION: u32 = 1;

/// The content of a Source: its rules and the bookkeeping that keeps Identifiers unique.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct RuleSet {
    /// Source format version.
    pub format: u32,
    /// Highest number allocated per Identifier prefix.
    #[serde(default)]
    pub sequences: BTreeMap<String, u64>,
    /// Retired Identifiers, never allocated again.
    #[serde(default)]
    pub retired: Vec<RuleId>,
    /// The rules.
    #[serde(default)]
    pub rules: Vec<Rule>,
}

impl RuleSet {
    /// An empty Source of the current format.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            format: FORMAT_VERSION,
            sequences: BTreeMap::new(),
            retired: Vec::new(),
            rules: Vec::new(),
        }
    }

    /// Adds `draft` as a `proposed` Rule under the next free Identifier of `prefix`.
    ///
    /// The number follows the highest of: the recorded sequence, the Rules present and the
    /// retired Identifiers. A Rule removed from the Source by hand therefore never has its
    /// Identifier reused, and `sequences` never decreases. Nothing changes on error.
    ///
    /// # Errors
    /// [`Error::IdentifierSpaceExhausted`] if the number would overflow.
    pub fn add(&mut self, prefix: &IdPrefix, draft: RuleDraft) -> Result<RuleId, Error> {
        let recorded = self.sequences.get(prefix.as_str()).copied().unwrap_or(0);
        let highest = self
            .rules
            .iter()
            .map(|r| &r.id)
            .chain(&self.retired)
            .filter_map(|id| prefix.number_of(id))
            .fold(recorded, u64::max);
        let next = highest
            .checked_add(1)
            .ok_or_else(|| Error::IdentifierSpaceExhausted(prefix.as_str().to_owned()))?;
        let id = prefix.identifier(next);
        self.sequences.insert(prefix.as_str().to_owned(), next);
        self.rules.push(draft.into_proposed(id.clone()));
        Ok(id)
    }

    /// `proposed` → `validated`.
    ///
    /// # Errors
    /// [`LifecycleError`] if the Rule does not exist or is not `proposed`. Nothing changes.
    pub fn approve(&mut self, id: &RuleId) -> Result<(), LifecycleError> {
        let at = self.position(id, "approve", RuleStatus::Proposed)?;
        self.rules[at].status = RuleStatus::Validated;
        Ok(())
    }

    /// `proposed` → `rejected`; the Identifier is retired.
    ///
    /// # Errors
    /// [`LifecycleError`] if the Rule does not exist or is not `proposed`. Nothing changes.
    pub fn reject(&mut self, id: &RuleId) -> Result<(), LifecycleError> {
        let at = self.position(id, "reject", RuleStatus::Proposed)?;
        self.rules[at].status = RuleStatus::Rejected;
        self.retire(id);
        Ok(())
    }

    /// `validated` → `obsolete`, optionally recording the Rule that replaces it; the
    /// Identifier is retired.
    ///
    /// # Errors
    /// [`LifecycleError`] if the Rule does not exist or is not `validated`, or if the
    /// replacement is the Rule itself or does not exist. Nothing changes.
    pub fn obsolete(
        &mut self,
        id: &RuleId,
        superseded_by: Option<&RuleId>,
    ) -> Result<(), LifecycleError> {
        let at = self.position(id, "obsolete", RuleStatus::Validated)?;
        if let Some(replacement) = superseded_by {
            if replacement == id {
                return Err(LifecycleError::SelfReplacement(id.clone()));
            }
            if self.find(replacement).is_none() {
                return Err(LifecycleError::ReplacementNotFound(replacement.clone()));
            }
        }
        let rule = &mut self.rules[at];
        rule.status = RuleStatus::Obsolete;
        rule.superseded_by = superseded_by.cloned();
        self.retire(id);
        Ok(())
    }

    fn find(&self, id: &RuleId) -> Option<&Rule> {
        self.rules.iter().find(|r| &r.id == id)
    }

    /// Index of the Rule `id` if it is in the status `required`; the check every
    /// transition shares.
    fn position(
        &self,
        id: &RuleId,
        action: &'static str,
        required: RuleStatus,
    ) -> Result<usize, LifecycleError> {
        let at = self
            .rules
            .iter()
            .position(|r| &r.id == id)
            .ok_or_else(|| LifecycleError::RuleNotFound(id.clone()))?;
        let status = self.rules[at].status;
        if status == required {
            Ok(at)
        } else {
            Err(LifecycleError::TransitionRefused {
                id: id.clone(),
                status,
                required,
                action,
            })
        }
    }

    fn retire(&mut self, id: &RuleId) {
        if !self.retired.contains(id) {
            self.retired.push(id.clone());
        }
    }

    /// Serialises in Canonical form: rules in natural Identifier order, keys in schema
    /// order, one property or array element per line, LF endings, trailing newline.
    ///
    /// # Panics
    /// Never in practice: a `RuleSet` only holds string-keyed maps and plain values.
    #[must_use]
    pub fn to_canonical(&self) -> String {
        let mut sorted = self.clone();
        sorted.rules.sort_by(|a, b| natural_cmp(&a.id.0, &b.id.0));
        sorted.retired.sort_by(|a, b| natural_cmp(&a.0, &b.0));
        let mut out = serde_json::to_string_pretty(&sorted)
            .expect("a RuleSet always serialises: string keys, no custom serialisers");
        out.push('\n');
        out
    }
}

/// Compares strings so that digit runs compare by value (`RULE-2` < `RULE-10`).
fn natural_cmp(a: &str, b: &str) -> Ordering {
    let (mut a, mut b) = (a.chars().peekable(), b.chars().peekable());
    loop {
        match (a.peek().copied(), b.peek().copied()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(x), Some(y)) if x.is_ascii_digit() && y.is_ascii_digit() => {
                let (na, nb) = (take_digits(&mut a), take_digits(&mut b));
                let (ta, tb) = (na.trim_start_matches('0'), nb.trim_start_matches('0'));
                let ord = ta.len().cmp(&tb.len()).then_with(|| ta.cmp(tb));
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            (Some(x), Some(y)) => {
                if x != y {
                    return x.cmp(&y);
                }
                a.next();
                b.next();
            }
        }
    }
}

fn take_digits(it: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
    let mut s = String::new();
    while let Some(c) = it.next_if(char::is_ascii_digit) {
        s.push(c);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draft() -> RuleDraft {
        RuleDraft::new("T", "S", &[], None, &[], &[]).unwrap_or_else(|e| panic!("{e}"))
    }

    fn add(set: &mut RuleSet) -> String {
        set.add(&IdPrefix::default(), draft())
            .unwrap_or_else(|e| panic!("{e}"))
            .0
    }

    #[test]
    fn first_identifier_is_one_and_the_sequence_follows() {
        let mut set = RuleSet::empty();
        assert_eq!(add(&mut set), "RULE-001");
        assert_eq!(add(&mut set), "RULE-002");
        assert_eq!(set.sequences.get("RULE"), Some(&2));
    }

    #[test]
    fn retired_identifiers_are_skipped() {
        let mut set = RuleSet::empty();
        set.retired.push(RuleId("RULE-007".into()));
        assert_eq!(add(&mut set), "RULE-008");
    }

    #[test]
    fn a_rule_deleted_by_hand_keeps_its_identifier_through_the_sequence() {
        let mut set = RuleSet::empty();
        add(&mut set);
        add(&mut set);
        set.rules.pop();
        assert_eq!(add(&mut set), "RULE-003");
    }

    #[test]
    fn present_rules_win_over_a_lowered_sequence() {
        let mut set = RuleSet::empty();
        add(&mut set);
        set.sequences.insert("RULE".into(), 0);
        assert_eq!(add(&mut set), "RULE-002");
    }

    #[test]
    fn prefixes_have_independent_sequences() {
        let mut set = RuleSet::empty();
        add(&mut set);
        let br = IdPrefix::parse("BR").unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(set.add(&br, draft()).map(|i| i.0), Ok("BR-001".to_owned()));
    }

    fn id(n: u32) -> RuleId {
        RuleId(format!("RULE-{n:03}"))
    }

    /// A set holding RULE-001 (in `status`) and RULE-002 (proposed).
    fn set_with(status: RuleStatus) -> RuleSet {
        let mut set = RuleSet::empty();
        add(&mut set);
        add(&mut set);
        set.rules[0].status = status;
        set
    }

    fn status(set: &RuleSet, n: u32) -> RuleStatus {
        set.find(&id(n)).map(|r| r.status).expect("rule present")
    }

    #[test]
    fn approve_validates_a_proposed_rule() {
        let mut set = set_with(RuleStatus::Proposed);
        assert_eq!(set.approve(&id(1)), Ok(()));
        assert_eq!(status(&set, 1), RuleStatus::Validated);
        assert_eq!(status(&set, 2), RuleStatus::Proposed);
        assert_eq!(set.retired, Vec::<RuleId>::new());
    }

    #[test]
    fn reject_retires_the_identifier() {
        let mut set = set_with(RuleStatus::Proposed);
        assert_eq!(set.reject(&id(1)), Ok(()));
        assert_eq!(status(&set, 1), RuleStatus::Rejected);
        assert_eq!(set.retired, [id(1)]);
    }

    #[test]
    fn obsolete_retires_and_records_the_replacement() {
        let mut set = set_with(RuleStatus::Validated);
        assert_eq!(set.obsolete(&id(1), Some(&id(2))), Ok(()));
        assert_eq!(status(&set, 1), RuleStatus::Obsolete);
        assert_eq!(set.rules[0].superseded_by, Some(id(2)));
        assert_eq!(set.retired, [id(1)]);
    }

    #[test]
    fn obsolete_without_replacement_records_none() {
        let mut set = set_with(RuleStatus::Validated);
        assert_eq!(set.obsolete(&id(1), None), Ok(()));
        assert_eq!(set.rules[0].superseded_by, None);
    }

    #[test]
    fn retiring_twice_does_not_duplicate() {
        let mut set = set_with(RuleStatus::Proposed);
        set.retired.push(id(1));
        assert_eq!(set.reject(&id(1)), Ok(()));
        assert_eq!(set.retired, [id(1)]);
    }

    #[test]
    fn every_other_transition_is_refused_and_changes_nothing() {
        use RuleStatus::{Obsolete, Proposed, Rejected, Validated};
        for from in [Proposed, Validated, Rejected, Obsolete] {
            type Action = fn(&mut RuleSet) -> Result<(), LifecycleError>;
            let actions: [(&str, RuleStatus, Action); 3] = [
                ("approve", Proposed, |s| s.approve(&id(1))),
                ("reject", Proposed, |s| s.reject(&id(1))),
                ("obsolete", Validated, |s| s.obsolete(&id(1), None)),
            ];
            for (action, required, run) in actions {
                if from == required {
                    continue;
                }
                let mut set = set_with(from);
                let before = set.clone();
                assert_eq!(
                    run(&mut set),
                    Err(LifecycleError::TransitionRefused {
                        id: id(1),
                        status: from,
                        required,
                        action,
                    })
                );
                assert_eq!(set, before, "{action} from {from} changed the set");
            }
        }
    }

    #[test]
    fn an_unknown_rule_is_refused() {
        let mut set = set_with(RuleStatus::Proposed);
        let before = set.clone();
        for result in [
            set.approve(&id(9)),
            set.reject(&id(9)),
            set.obsolete(&id(9), None),
        ] {
            assert_eq!(result, Err(LifecycleError::RuleNotFound(id(9))));
        }
        assert_eq!(set, before);
    }

    #[test]
    fn a_bad_replacement_is_refused_and_changes_nothing() {
        let mut set = set_with(RuleStatus::Validated);
        let before = set.clone();
        assert_eq!(
            set.obsolete(&id(1), Some(&id(9))),
            Err(LifecycleError::ReplacementNotFound(id(9)))
        );
        assert_eq!(
            set.obsolete(&id(1), Some(&id(1))),
            Err(LifecycleError::SelfReplacement(id(1)))
        );
        assert_eq!(set, before);
    }

    #[test]
    fn overflow_is_an_error_and_changes_nothing() {
        let mut set = RuleSet::empty();
        set.sequences.insert("RULE".into(), u64::MAX);
        let before = set.clone();
        assert_eq!(
            set.add(&IdPrefix::default(), draft()),
            Err(Error::IdentifierSpaceExhausted("RULE".into()))
        );
        assert_eq!(set, before);
    }
}
