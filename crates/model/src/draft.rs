use crate::{DraftError, Rule, RuleId, RuleStatus, Scope};

/// Longest accepted title, in characters.
pub const TITLE_MAX_CHARS: usize = 120;

/// The user-supplied content of a new Rule, validated and trimmed.
///
/// It has no Identifier and no status: [`RuleSet::add`](crate::RuleSet::add) gives it both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleDraft {
    /// Trimmed, 1 to [`TITLE_MAX_CHARS`] characters.
    title: String,
    /// Trimmed, not empty.
    statement: String,
    /// Applications the Rule applies to, trimmed, without duplicates.
    applications: Vec<String>,
    /// Functional module of the Rule; `None` when not given.
    module: Option<String>,
    /// Cases the Rule covers, trimmed, without duplicates.
    covered: Vec<String>,
    /// Cases the Rule explicitly excludes, trimmed, without duplicates.
    excluded: Vec<String>,
    /// Related Rules, without duplicates. Whether they exist is for the
    /// [`RuleSet`](crate::RuleSet) to say.
    related: Vec<RuleId>,
}

impl RuleDraft {
    /// Validates the values of a new Rule.
    ///
    /// Text is trimmed before it is checked and stored. `module` is `None` when not given.
    ///
    /// # Errors
    /// [`DraftError::Empty`] for an empty title, statement, module or list item,
    /// [`DraftError::TitleTooLong`] past [`TITLE_MAX_CHARS`], [`DraftError::Duplicate`] for a repeated
    /// list item or related Rule.
    pub fn new(
        title: &str,
        statement: &str,
        applications: &[String],
        module: Option<&str>,
        covered: &[String],
        excluded: &[String],
        related: &[RuleId],
    ) -> Result<Self, DraftError> {
        let title = non_empty("title", title)?;
        let len = title.chars().count();
        if len > TITLE_MAX_CHARS {
            return Err(DraftError::TitleTooLong {
                len,
                max: TITLE_MAX_CHARS,
            });
        }
        Ok(Self {
            title,
            statement: non_empty("statement", statement)?,
            applications: unique_items("application", applications)?,
            module: module.map(|m| non_empty("module", m)).transpose()?,
            covered: unique_items("covered case", covered)?,
            excluded: unique_items("excluded case", excluded)?,
            related: unique_ids("related Rule", related)?,
        })
    }

    /// The Rules this draft points at, to be checked against the Source before it is added.
    pub(crate) fn related(&self) -> &[RuleId] {
        &self.related
    }

    /// The `proposed` Rule this draft becomes under `id`.
    pub(crate) fn into_proposed(self, id: RuleId) -> Rule {
        let scope = (!self.covered.is_empty() || !self.excluded.is_empty()).then_some(Scope {
            covered: self.covered,
            excluded: self.excluded,
        });
        Rule {
            id,
            title: self.title,
            statement: self.statement,
            status: RuleStatus::Proposed,
            applications: self.applications,
            module: self.module,
            scope,
            origin: None,
            related: self.related,
            superseded_by: None,
        }
    }
}

fn non_empty(field: &'static str, raw: &str) -> Result<String, DraftError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        Err(DraftError::Empty { field })
    } else {
        Ok(trimmed.to_owned())
    }
}

fn unique_items(field: &'static str, raw: &[String]) -> Result<Vec<String>, DraftError> {
    let mut items: Vec<String> = Vec::with_capacity(raw.len());
    for item in raw {
        let item = non_empty(field, item)?;
        if items.contains(&item) {
            return Err(DraftError::Duplicate { field, value: item });
        }
        items.push(item);
    }
    Ok(items)
}

fn unique_ids(field: &'static str, raw: &[RuleId]) -> Result<Vec<RuleId>, DraftError> {
    let mut ids: Vec<RuleId> = Vec::with_capacity(raw.len());
    for id in raw {
        if ids.contains(id) {
            return Err(DraftError::Duplicate {
                field,
                value: id.0.clone(),
            });
        }
        ids.push(id.clone());
    }
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draft(title: &str, statement: &str) -> Result<RuleDraft, DraftError> {
        RuleDraft::new(title, statement, &[], None, &[], &[], &[])
    }

    #[test]
    fn text_is_trimmed() {
        let d = draft("  Title \n", " Statement ").unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(
            (d.title.as_str(), d.statement.as_str()),
            ("Title", "Statement")
        );
    }

    #[test]
    fn blank_title_and_statement_are_refused() {
        assert_eq!(draft(" ", "S"), Err(DraftError::Empty { field: "title" }));
        assert_eq!(
            draft("T", "\t"),
            Err(DraftError::Empty { field: "statement" })
        );
    }

    #[test]
    fn title_length_is_counted_in_characters_after_trimming() {
        let ok = format!(" {} ", "é".repeat(TITLE_MAX_CHARS));
        assert!(draft(&ok, "S").is_ok());
        let long = "é".repeat(TITLE_MAX_CHARS + 1);
        assert_eq!(
            draft(&long, "S"),
            Err(DraftError::TitleTooLong { len: 121, max: 120 })
        );
    }

    #[test]
    fn blank_list_items_and_module_are_refused() {
        let blank = vec![" ".to_owned()];
        assert!(RuleDraft::new("T", "S", &blank, None, &[], &[], &[]).is_err());
        assert!(RuleDraft::new("T", "S", &[], None, &blank, &[], &[]).is_err());
        assert!(RuleDraft::new("T", "S", &[], None, &[], &blank, &[]).is_err());
        assert!(RuleDraft::new("T", "S", &[], Some(""), &[], &[], &[]).is_err());
    }

    #[test]
    fn duplicate_items_are_refused_after_trimming() {
        let dup = vec!["a".to_owned(), " a".to_owned()];
        assert_eq!(
            RuleDraft::new("T", "S", &dup, None, &[], &[], &[]),
            Err(DraftError::Duplicate {
                field: "application",
                value: "a".into()
            })
        );
    }

    #[test]
    fn scope_is_absent_unless_a_case_is_given() {
        let none = draft("T", "S").unwrap_or_else(|e| panic!("{e}"));
        assert!(
            none.into_proposed(RuleId("RULE-001".into()))
                .scope
                .is_none()
        );
        let cases = vec!["x".to_owned()];
        let some =
            RuleDraft::new("T", "S", &[], None, &cases, &[], &[]).unwrap_or_else(|e| panic!("{e}"));
        let rule = some.into_proposed(RuleId("RULE-001".into()));
        assert_eq!(rule.scope.map(|s| s.covered), Some(cases));
        assert_eq!(rule.status, RuleStatus::Proposed);
    }

    #[test]
    fn a_repeated_related_rule_is_refused_naming_it() {
        let related = [RuleId("RULE-001".into()), RuleId("RULE-001".into())];
        assert_eq!(
            RuleDraft::new("T", "S", &[], None, &[], &[], &related),
            Err(DraftError::Duplicate {
                field: "related Rule",
                value: "RULE-001".into()
            })
        );
    }
}
