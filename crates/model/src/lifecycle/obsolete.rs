//! `validated → obsolete`.

use crate::{LifecycleError, RuleId, RuleSet, RuleStatus};

impl RuleSet {
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
}

#[cfg(test)]
mod tests {
    use super::super::support::{
        assert_refused_unless, assert_unknown_rule_refused, id, set_with, status,
    };
    use super::*;

    #[test]
    fn retires_and_records_the_replacement() {
        let mut set = set_with(RuleStatus::Validated);
        assert_eq!(set.obsolete(&id(1), Some(&id(2))), Ok(()));
        assert_eq!(status(&set, 1), RuleStatus::Obsolete);
        assert_eq!(set.rules[0].superseded_by, Some(id(2)));
        assert_eq!(set.retired, [id(1)]);
    }

    #[test]
    fn without_replacement_records_none() {
        let mut set = set_with(RuleStatus::Validated);
        assert_eq!(set.obsolete(&id(1), None), Ok(()));
        assert_eq!(set.rules[0].superseded_by, None);
        assert_eq!(set.retired, [id(1)]);
    }

    #[test]
    fn an_unknown_replacement_is_refused_and_changes_nothing() {
        let mut set = set_with(RuleStatus::Validated);
        let before = set.clone();
        assert_eq!(
            set.obsolete(&id(1), Some(&id(9))),
            Err(LifecycleError::ReplacementNotFound(id(9)))
        );
        assert_eq!(set, before);
    }

    #[test]
    fn a_rule_cannot_replace_itself() {
        let mut set = set_with(RuleStatus::Validated);
        let before = set.clone();
        assert_eq!(
            set.obsolete(&id(1), Some(&id(1))),
            Err(LifecycleError::SelfReplacement(id(1)))
        );
        assert_eq!(set, before);
    }

    #[test]
    fn is_refused_unless_validated() {
        assert_refused_unless(RuleStatus::Validated, "obsolete", |s| {
            s.obsolete(&id(1), None)
        });
    }

    #[test]
    fn is_refused_for_an_unknown_rule() {
        assert_unknown_rule_refused(|s, id| s.obsolete(id, None));
    }
}
