use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

/// Separates the prefix of an Identifier from its suffix.
const IDENTIFIER_HYPHEN: char = '-';

/// What an Identifier is made of, after the first letter of its prefix and around its hyphen.
pub(crate) fn is_identifier_char(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'
}

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

    /// Whether this follows the grammar of an Identifier, whatever prefix a project
    /// configures: a prefix as [`IdPrefix`](crate::IdPrefix) accepts it, one hyphen, then
    /// upper-case letters, digits or `_`.
    ///
    /// ```
    /// use rulelink_model::RuleId;
    ///
    /// let id = |raw: &str| RuleId(raw.to_owned());
    /// assert!(id("PRICING_V2-A1").is_well_formed());
    /// assert!(!id("rule-042").is_well_formed());
    /// ```
    #[must_use]
    pub fn is_well_formed(&self) -> bool {
        let Some((prefix, suffix)) = self.0.split_once(IDENTIFIER_HYPHEN) else {
            return false;
        };
        crate::IdPrefix::parse(prefix).is_ok()
            && !suffix.is_empty()
            && suffix.chars().all(is_identifier_char)
    }

    /// The order Identifiers are listed in: digit runs compare by value, so `RULE-2` comes
    /// before `RULE-10`. Not [`Ord`]: `RULE-2` and `RULE-02` are equal here, yet different.
    ///
    /// ```
    /// use rulelink_model::RuleId;
    ///
    /// let id = |raw: &str| RuleId(raw.to_owned());
    /// assert!(id("RULE-2").natural_cmp(&id("RULE-10")).is_lt());
    /// ```
    #[must_use]
    pub fn natural_cmp(&self, other: &Self) -> Ordering {
        natural_cmp(&self.0, &other.0)
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
        Self::ALL
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

/// Compares strings so that digit runs compare by value (`RULE-2` < `RULE-10`).
fn natural_cmp(a: &str, b: &str) -> Ordering {
    let (mut a, mut b) = (a.chars().peekable(), b.chars().peekable());
    loop {
        match (a.peek().copied(), b.peek().copied()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(x), Some(y)) if x.is_ascii_digit() && y.is_ascii_digit() => {
                let (na, nb) = (take_digits(&mut a), take_digits(&mut b));
                let (ta, tb) = (na.trim_start_matches('0'), nb.trim_start_matches('0'));
                let ord = ta.len().cmp(&tb.len()).then_with(|| ta.cmp(tb));
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            (Some(x), Some(y)) => {
                if x != y {
                    return x.cmp(&y);
                }
                a.next();
                b.next();
            }
        }
    }
}

fn take_digits(it: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
    let mut s = String::new();
    while let Some(c) = it.next_if(char::is_ascii_digit) {
        s.push(c);
    }
    s
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
