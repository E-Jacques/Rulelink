//! `proposed → validated`.

use crate::{LifecycleError, RuleId, RuleSet, RuleStatus};

impl RuleSet {
    /// `proposed` → `validated`.
    ///
    /// # Errors
    /// [`LifecycleError`] if the Rule does not exist or is not `proposed`. Nothing changes.
    pub fn approve(&mut self, id: &RuleId) -> Result<(), LifecycleError> {
        let at = self.position(id, "approve", RuleStatus::Proposed)?;
        self.rules[at].status = RuleStatus::Validated;
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
    fn validates_a_proposed_rule_and_only_that_one() {
        let mut set = set_with(RuleStatus::Proposed);
        assert_eq!(set.approve(&id(1)), Ok(()));
        assert_eq!(status(&set, 1), RuleStatus::Validated);
        assert_eq!(status(&set, 2), RuleStatus::Proposed);
    }

    #[test]
    fn does_not_retire_the_identifier() {
        let mut set = set_with(RuleStatus::Proposed);
        set.approve(&id(1)).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(set.retired, Vec::<RuleId>::new());
    }

    #[test]
    fn is_refused_unless_proposed() {
        assert_refused_unless(RuleStatus::Proposed, "approve", |s| s.approve(&id(1)));
    }

    #[test]
    fn is_refused_for_an_unknown_rule() {
        assert_unknown_rule_refused(RuleSet::approve);
    }
}
