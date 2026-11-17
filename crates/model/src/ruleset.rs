use std::cmp::Ordering;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Error, Rule, RuleId};

/// The `format` value of the current Source layout.
pub const FORMAT_VERSION: u32 = 1;

/// The content of a Source: its rules and the bookkeeping that keeps Identifiers unique.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct RuleSet {
    /// Source format version.
    pub format: u32,
    /// Highest number allocated per Identifier prefix.
    #[serde(default)]
    pub sequences: BTreeMap<String, u64>,
    /// Retired Identifiers, never allocated again.
    #[serde(default)]
    pub retired: Vec<RuleId>,
    /// The rules.
    #[serde(default)]
    pub rules: Vec<Rule>,
}

impl RuleSet {
    /// An empty Source of the current format.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            format: FORMAT_VERSION,
            sequences: BTreeMap::new(),
            retired: Vec::new(),
            rules: Vec::new(),
        }
    }

    /// Allocates the next Identifier for `prefix` and records it in `sequences`.
    ///
    /// The number is one above the highest of the recorded sequence, the Rules present and
    /// the Retired identifiers, so an Identifier is never reused, even if its Rule was
    /// deleted from the Source by hand.
    pub fn allocate_id(&mut self, prefix: &str) -> RuleId {
        let recorded = self.sequences.get(prefix).copied().unwrap_or(0);
        let seen = self
            .rules
            .iter()
            .map(|r| &r.id)
            .chain(&self.retired)
            .filter_map(|id| number_of(&id.0, prefix))
            .max()
            .unwrap_or(0);
        let next = recorded.max(seen) + 1;
        self.sequences.insert(prefix.to_owned(), next);
        RuleId(format!("{prefix}-{next:03}"))
    }

    /// Validates `rule` and appends it.
    ///
    /// # Errors
    /// Returns [`Error::InvalidRule`] if the rule is invalid, reuses an Identifier, or
    /// refers to a related Rule that does not exist.
    pub fn add(&mut self, rule: Rule) -> Result<(), Error> {
        rule.validate()?;
        if self.rules.iter().any(|r| r.id == rule.id) || self.retired.contains(&rule.id) {
            return Err(Error::InvalidRule(format!(
                "the identifier {} is already used",
                rule.id.0
            )));
        }
        if let Some(missing) = rule
            .related
            .iter()
            .find(|id| !self.rules.iter().any(|r| &r.id == *id))
        {
            return Err(Error::InvalidRule(format!(
                "the related rule {} does not exist",
                missing.0
            )));
        }
        self.rules.push(rule);
        Ok(())
    }

    /// Serialises in Canonical form: rules in natural Identifier order, keys in schema
    /// order, one property or array element per line, LF endings, trailing newline.
    ///
    /// # Panics
    /// Never in practice: a `RuleSet` only holds string-keyed maps and plain values.
    #[must_use]
    pub fn to_canonical(&self) -> String {
        let mut sorted = self.clone();
        sorted.rules.sort_by(|a, b| natural_cmp(&a.id.0, &b.id.0));
        sorted.retired.sort_by(|a, b| natural_cmp(&a.0, &b.0));
        let mut out = serde_json::to_string_pretty(&sorted)
            .expect("a RuleSet always serialises: string keys, no custom serialisers");
        out.push('\n');
        out
    }
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

/// The number of an Identifier `PREFIX-<digits>`, if it has that prefix.
fn number_of(id: &str, prefix: &str) -> Option<u64> {
    let digits = id.strip_prefix(prefix)?.strip_prefix('-')?;
    digits.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RuleStatus;

    fn rule(id: &str) -> Rule {
        Rule {
            id: RuleId(id.to_owned()),
            title: "T".into(),
            statement: "S".into(),
            status: RuleStatus::Proposed,
            applications: vec![],
            module: None,
            scope: None,
            origin: None,
            related: vec![],
            superseded_by: None,
        }
    }

    #[test]
    fn allocation_is_monotonic_per_prefix() {
        let mut set = RuleSet::empty();
        assert_eq!(set.allocate_id("RULE").0, "RULE-001");
        assert_eq!(set.allocate_id("RULE").0, "RULE-002");
        assert_eq!(set.allocate_id("BR").0, "BR-001");
    }

    #[test]
    fn allocation_skips_retired_and_present_identifiers() {
        let mut set = RuleSet::empty();
        set.retired.push(RuleId("RULE-007".into()));
        set.rules.push(rule("RULE-010"));
        set.rules.push(rule("OTHER-099"));
        assert_eq!(set.allocate_id("RULE").0, "RULE-011");
    }

    #[test]
    fn allocation_respects_a_recorded_sequence_above_everything_present() {
        let mut set = RuleSet::empty();
        set.sequences.insert("RULE".into(), 5);
        assert_eq!(set.allocate_id("RULE").0, "RULE-006");
    }

    #[test]
    fn add_refuses_unknown_related_and_invalid_rules() {
        let mut set = RuleSet::empty();
        let mut bad = rule("RULE-001");
        bad.related.push(RuleId("RULE-009".into()));
        assert!(set.add(bad).is_err());
        let mut long = rule("RULE-001");
        long.title = "x".repeat(121);
        assert!(set.add(long).is_err());
        assert!(set.add(rule("RULE-001")).is_ok());
        assert!(set.add(rule("RULE-001")).is_err());
        assert_eq!(set.rules.len(), 1);
    }
}
