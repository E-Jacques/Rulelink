use serde::{Deserialize, Serialize};

/// Permanent identifier of a rule (e.g. `RULE-042`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct RuleId(pub String);

impl RuleId {
    /// Reads an Identifier typed by a user: surrounding whitespace is dropped, blank is refused.
    /// Whether a Rule has this Identifier is for the [`crate::RuleSet`] to say.
    ///
    /// # Errors
    /// [`crate::BlankRuleId`] naming `field` if nothing is left after trimming.
    pub fn parse(raw: &str, field: &'static str) -> Result<Self, crate::BlankRuleId> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(crate::BlankRuleId { field })
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }
}

impl std::fmt::Display for RuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

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

impl RuleStatus {
    /// Every status, in lifecycle order.
    pub const ALL: [Self; 4] = [
        Self::Proposed,
        Self::Validated,
        Self::Obsolete,
        Self::Rejected,
    ];
}

impl std::fmt::Display for RuleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Proposed => "proposed",
            Self::Validated => "validated",
            Self::Obsolete => "obsolete",
            Self::Rejected => "rejected",
        })
    }
}

impl std::str::FromStr for RuleStatus {
    type Err = crate::UnknownStatus;

    /// Reads a status typed by a user, exactly as [`Display`](std::fmt::Display) writes it.
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        [
            Self::Proposed,
            Self::Validated,
            Self::Obsolete,
            Self::Rejected,
        ]
        .into_iter()
        .find(|status| status.to_string() == raw)
        .ok_or_else(|| crate::UnknownStatus {
            value: raw.to_owned(),
        })
    }
}

/// Covered and excluded cases of a rule.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_status_reads_back_what_it_displays() {
        for status in RuleStatus::ALL {
            assert_eq!(status.to_string().parse(), Ok(status));
        }
    }

    #[test]
    fn an_unknown_status_names_the_value_and_the_allowed_ones() {
        let err = "draft"
            .parse::<RuleStatus>()
            .expect_err("refused")
            .to_string();
        assert!(err.contains("`draft`") && err.contains("proposed"), "{err}");
        assert!("Proposed".parse::<RuleStatus>().is_err());
    }
}
