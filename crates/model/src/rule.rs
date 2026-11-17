use serde::{Deserialize, Serialize};

use crate::Error;

/// Longest accepted rule title, in characters.
pub const MAX_TITLE_CHARS: usize = 120;

/// Permanent identifier of a rule (e.g. `RULE-042`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct RuleId(pub String);

/// Lifecycle status of a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum RuleStatus {
    /// Awaiting human review.
    Proposed,
    /// Approved by a human.
    Validated,
    /// No longer applicable.
    Obsolete,
    /// Declined by a human; kept so it is not proposed again.
    Rejected,
}

/// Covered and excluded cases of a rule.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Scope {
    /// Cases the rule covers.
    #[serde(default)]
    pub covered: Vec<String>,
    /// Cases the rule explicitly excludes.
    #[serde(default)]
    pub excluded: Vec<String>,
}

/// Where a rule comes from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Origin {
    /// Kind of origin (`jira`, `github`, `retrodoc`, `manual`, ...).
    pub kind: String,
    /// Reference in the origin system.
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Link to the origin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// A business rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Rule {
    /// Identifier.
    pub id: RuleId,
    /// Short title (1 to 120 characters).
    pub title: String,
    /// Statement of the rule.
    pub statement: String,
    /// Lifecycle status.
    pub status: RuleStatus,
    /// Applications the rule applies to; empty means the whole project.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applications: Vec<String>,
    /// Functional module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// Covered and excluded cases.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// Where the rule comes from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
    /// Related rules.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<RuleId>,
    /// Replacement of an obsolete rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub superseded_by: Option<RuleId>,
}

impl Rule {
    /// Checks the rule against the model, independently of any Source.
    ///
    /// # Errors
    /// Returns [`Error::InvalidRule`] describing the first problem found.
    pub fn validate(&self) -> Result<(), Error> {
        let title_len = self.title.chars().count();
        if self.title.trim().is_empty() {
            return Err(invalid("the title must not be empty"));
        }
        if title_len > MAX_TITLE_CHARS {
            return Err(invalid(&format!(
                "the title is {title_len} characters long, the maximum is {MAX_TITLE_CHARS}"
            )));
        }
        if self.statement.trim().is_empty() {
            return Err(invalid("the statement must not be empty"));
        }
        if let Some(origin) = &self.origin
            && origin.kind.trim().is_empty()
        {
            return Err(invalid("the origin kind must not be empty"));
        }
        Ok(())
    }
}

fn invalid(reason: &str) -> Error {
    Error::InvalidRule(reason.to_owned())
}
