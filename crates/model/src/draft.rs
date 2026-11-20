use crate::{Error, Rule, RuleId, RuleStatus, Scope};

/// Longest accepted title, in characters.
pub const TITLE_MAX_CHARS: usize = 120;

/// The user-supplied content of a new Rule, validated and trimmed.
///
/// It has no Identifier and no status: [`RuleSet::add`](crate::RuleSet::add) gives it both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleDraft {
    title: String,
    statement: String,
    applications: Vec<String>,
    module: Option<String>,
    covered: Vec<String>,
    excluded: Vec<String>,
}

impl RuleDraft {
    /// Validates the values of a new Rule.
    ///
    /// Text is trimmed before it is checked and stored. `module` is `None` when not given.
    ///
    /// # Errors
    /// [`Error::Empty`] for an empty title, statement, module or list item,
    /// [`Error::TitleTooLong`] past [`TITLE_MAX_CHARS`], [`Error::Duplicate`] for a repeated
    /// list item.
    pub fn new(
        title: &str,
        statement: &str,
        applications: &[String],
        module: Option<&str>,
        covered: &[String],
        excluded: &[String],
    ) -> Result<Self, Error> {
        let title = non_empty("--title", title)?;
        let len = title.chars().count();
        if len > TITLE_MAX_CHARS {
            return Err(Error::TitleTooLong {
                len,
                max: TITLE_MAX_CHARS,
            });
        }
        Ok(Self {
            title,
            statement: non_empty("--statement", statement)?,
            applications: unique_items("--app", applications)?,
            module: module.map(|m| non_empty("--module", m)).transpose()?,
            covered: unique_items("--covered", covered)?,
            excluded: unique_items("--excluded", excluded)?,
        })
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
            related: Vec::new(),
            superseded_by: None,
        }
    }
}

fn non_empty(field: &'static str, raw: &str) -> Result<String, Error> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        Err(Error::Empty { field })
    } else {
        Ok(trimmed.to_owned())
    }
}

fn unique_items(field: &'static str, raw: &[String]) -> Result<Vec<String>, Error> {
    let mut items: Vec<String> = Vec::with_capacity(raw.len());
    for item in raw {
        let item = non_empty(field, item)?;
        if items.contains(&item) {
            return Err(Error::Duplicate { field, value: item });
        }
        items.push(item);
    }
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draft(title: &str, statement: &str) -> Result<RuleDraft, Error> {
        RuleDraft::new(title, statement, &[], None, &[], &[])
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
        assert_eq!(draft(" ", "S"), Err(Error::Empty { field: "--title" }));
        assert_eq!(
            draft("T", "\t"),
            Err(Error::Empty {
                field: "--statement"
            })
        );
    }

    #[test]
    fn title_length_is_counted_in_characters_after_trimming() {
        let ok = format!(" {} ", "é".repeat(TITLE_MAX_CHARS));
        assert!(draft(&ok, "S").is_ok());
        let long = "é".repeat(TITLE_MAX_CHARS + 1);
        assert_eq!(
            draft(&long, "S"),
            Err(Error::TitleTooLong { len: 121, max: 120 })
        );
    }

    #[test]
    fn blank_list_items_and_module_are_refused() {
        let blank = vec![" ".to_owned()];
        assert!(RuleDraft::new("T", "S", &blank, None, &[], &[]).is_err());
        assert!(RuleDraft::new("T", "S", &[], None, &blank, &[]).is_err());
        assert!(RuleDraft::new("T", "S", &[], None, &[], &blank).is_err());
        assert!(RuleDraft::new("T", "S", &[], Some(""), &[], &[]).is_err());
    }

    #[test]
    fn duplicate_items_are_refused_after_trimming() {
        let dup = vec!["a".to_owned(), " a".to_owned()];
        assert_eq!(
            RuleDraft::new("T", "S", &dup, None, &[], &[]),
            Err(Error::Duplicate {
                field: "--app",
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
            RuleDraft::new("T", "S", &[], None, &cases, &[]).unwrap_or_else(|e| panic!("{e}"));
        let rule = some.into_proposed(RuleId("RULE-001".into()));
        assert_eq!(rule.scope.map(|s| s.covered), Some(cases));
        assert_eq!(rule.status, RuleStatus::Proposed);
    }
}
