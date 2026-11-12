use serde::{Deserialize, Serialize};

/// Stable identifier of a rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct RuleId(pub String);

/// Lifecycle status of a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum RuleStatus {
    /// Proposed, awaiting human validation.
    Proposed,
    /// Validated by a human.
    Validated,
    /// No longer applicable.
    Obsolete,
}

/// A business rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Rule {
    /// Identifier.
    pub id: RuleId,
    /// Short title.
    pub title: String,
    /// Lifecycle status.
    pub status: RuleStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_serialises_lowercase() {
        let json = serde_json::to_string(&RuleStatus::Validated).unwrap_or_default();
        assert_eq!(json, "\"validated\"");
    }
}
