//! Rule lifecycle: one file per transition, the checks they share live here.
//!
//! `proposed → validated` ([`approve`]), `proposed → rejected` ([`reject`]) and
//! `validated → obsolete` ([`obsolete`]). Each transition checks everything first and mutates
//! last, so a refusal leaves the [`RuleSet`] untouched.

mod approve;
mod obsolete;
mod reject;

use crate::{LifecycleError, Rule, RuleId, RuleSet, RuleStatus};

impl RuleSet {
    pub(crate) fn find(&self, id: &RuleId) -> Option<&Rule> {
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
}

/// Fixtures shared by the tests of the transitions.
#[cfg(test)]
mod support {
    use crate::{IdPrefix, LifecycleError, RuleDraft, RuleId, RuleSet, RuleStatus};

    pub(super) fn id(n: u32) -> RuleId {
        RuleId(format!("RULE-{n:03}"))
    }

    /// A set holding RULE-001 (in `status`) and RULE-002 (proposed).
    pub(super) fn set_with(status: RuleStatus) -> RuleSet {
        let mut set = RuleSet::empty();
        for _ in 0..2 {
            let draft = RuleDraft::new("T", "S", &[], None, &[], &[], &[])
                .unwrap_or_else(|e| panic!("{e}"));
            set.add(&IdPrefix::default(), draft)
                .unwrap_or_else(|e| panic!("{e}"));
        }
        set.rules[0].status = status;
        set
    }

    pub(super) fn status(set: &RuleSet, n: u32) -> RuleStatus {
        set.find(&id(n)).map(|r| r.status).expect("rule present")
    }

    /// Asserts that `run` on RULE-001 in every status but `allowed` is refused with a
    /// `TransitionRefused` naming `action`, and changes nothing.
    pub(super) fn assert_refused_unless(
        allowed: RuleStatus,
        action: &'static str,
        run: impl Fn(&mut RuleSet) -> Result<(), LifecycleError>,
    ) {
        use RuleStatus::{Obsolete, Proposed, Rejected, Validated};
        for from in [Proposed, Validated, Rejected, Obsolete] {
            if from == allowed {
                continue;
            }
            let mut set = set_with(from);
            let before = set.clone();
            assert_eq!(
                run(&mut set),
                Err(LifecycleError::TransitionRefused {
                    id: id(1),
                    status: from,
                    required: allowed,
                    action,
                })
            );
            assert_eq!(set, before, "{action} from {from} changed the set");
        }
    }

    /// Asserts that `run` on an unknown Rule is refused and changes nothing.
    pub(super) fn assert_unknown_rule_refused(
        run: impl Fn(&mut RuleSet, &RuleId) -> Result<(), LifecycleError>,
    ) {
        let mut set = set_with(RuleStatus::Proposed);
        let before = set.clone();
        assert_eq!(
            run(&mut set, &id(9)),
            Err(LifecycleError::RuleNotFound(id(9)))
        );
        assert_eq!(set, before);
    }
}
