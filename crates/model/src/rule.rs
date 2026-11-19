use serde::{Deserialize, Serialize};

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

impl Origin {
    /// Parses `<kind>:<reference>` (for example `jira:SHOP-42`).
    ///
    /// # Errors
    /// [`crate::Error::InvalidRule`] if either part is missing.
    pub fn parse(raw: &str) -> Result<Self, crate::Error> {
        match raw.split_once(':') {
            Some((kind, reference)) if !kind.trim().is_empty() && !reference.trim().is_empty() => {
                Ok(Self {
                    kind: kind.trim().to_owned(),
                    reference: Some(reference.trim().to_owned()),
                    url: None,
                })
            }
            _ => Err(crate::Error::InvalidRule(format!(
                "origin `{raw}` must look like <kind>:<reference>, for example jira:SHOP-42"
            ))),
        }
    }
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
