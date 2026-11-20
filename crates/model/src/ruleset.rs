use std::cmp::Ordering;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Error, IdPrefix, Rule, RuleDraft, RuleId};

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

    /// Adds `draft` as a `proposed` Rule under the next free Identifier of `prefix`.
    ///
    /// The number follows the highest of: the recorded sequence, the Rules present and the
    /// retired Identifiers. A Rule removed from the Source by hand therefore never has its
    /// Identifier reused, and `sequences` never decreases. Nothing changes on error.
    ///
    /// # Errors
    /// [`Error::IdentifierSpaceExhausted`] if the number would overflow.
    pub fn add(&mut self, prefix: &IdPrefix, draft: RuleDraft) -> Result<RuleId, Error> {
        let recorded = self.sequences.get(prefix.as_str()).copied().unwrap_or(0);
        let highest = self
            .rules
            .iter()
            .map(|r| &r.id)
            .chain(&self.retired)
            .filter_map(|id| prefix.number_of(id))
            .fold(recorded, u64::max);
        let next = highest
            .checked_add(1)
            .ok_or_else(|| Error::IdentifierSpaceExhausted(prefix.as_str().to_owned()))?;
        let id = prefix.identifier(next);
        self.sequences.insert(prefix.as_str().to_owned(), next);
        self.rules.push(draft.into_proposed(id.clone()));
        Ok(id)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn draft() -> RuleDraft {
        RuleDraft::new("T", "S", &[], None, &[], &[]).unwrap_or_else(|e| panic!("{e}"))
    }

    fn add(set: &mut RuleSet) -> String {
        set.add(&IdPrefix::default(), draft())
            .unwrap_or_else(|e| panic!("{e}"))
            .0
    }

    #[test]
    fn first_identifier_is_one_and_the_sequence_follows() {
        let mut set = RuleSet::empty();
        assert_eq!(add(&mut set), "RULE-001");
        assert_eq!(add(&mut set), "RULE-002");
        assert_eq!(set.sequences.get("RULE"), Some(&2));
    }

    #[test]
    fn retired_identifiers_are_skipped() {
        let mut set = RuleSet::empty();
        set.retired.push(RuleId("RULE-007".into()));
        assert_eq!(add(&mut set), "RULE-008");
    }

    #[test]
    fn a_rule_deleted_by_hand_keeps_its_identifier_through_the_sequence() {
        let mut set = RuleSet::empty();
        add(&mut set);
        add(&mut set);
        set.rules.pop();
        assert_eq!(add(&mut set), "RULE-003");
    }

    #[test]
    fn present_rules_win_over_a_lowered_sequence() {
        let mut set = RuleSet::empty();
        add(&mut set);
        set.sequences.insert("RULE".into(), 0);
        assert_eq!(add(&mut set), "RULE-002");
    }

    #[test]
    fn prefixes_have_independent_sequences() {
        let mut set = RuleSet::empty();
        add(&mut set);
        let br = IdPrefix::parse("BR").unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(set.add(&br, draft()).map(|i| i.0), Ok("BR-001".to_owned()));
    }

    #[test]
    fn overflow_is_an_error_and_changes_nothing() {
        let mut set = RuleSet::empty();
        set.sequences.insert("RULE".into(), u64::MAX);
        let before = set.clone();
        assert_eq!(
            set.add(&IdPrefix::default(), draft()),
            Err(Error::IdentifierSpaceExhausted("RULE".into()))
        );
        assert_eq!(set, before);
    }
}
