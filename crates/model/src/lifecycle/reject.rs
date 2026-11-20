//! `proposed → rejected`.

use crate::{LifecycleError, RuleId, RuleSet, RuleStatus};

impl RuleSet {
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
}

#[cfg(test)]
mod tests {
    use super::super::support::{
        assert_refused_unless, assert_unknown_rule_refused, id, set_with, status,
    };
    use super::*;

    #[test]
    fn rejects_a_proposed_rule_and_retires_its_identifier() {
        let mut set = set_with(RuleStatus::Proposed);
        assert_eq!(set.reject(&id(1)), Ok(()));
        assert_eq!(status(&set, 1), RuleStatus::Rejected);
        assert_eq!(status(&set, 2), RuleStatus::Proposed);
        assert_eq!(set.retired, [id(1)]);
    }

    #[test]
    fn retiring_an_already_retired_identifier_does_not_duplicate_it() {
        let mut set = set_with(RuleStatus::Proposed);
        set.retired.push(id(1));
        assert_eq!(set.reject(&id(1)), Ok(()));
        assert_eq!(set.retired, [id(1)]);
    }

    #[test]
    fn is_refused_unless_proposed() {
        assert_refused_unless(RuleStatus::Proposed, "reject", |s| s.reject(&id(1)));
    }

    #[test]
    fn is_refused_for_an_unknown_rule() {
        assert_unknown_rule_refused(RuleSet::reject);
    }
}
