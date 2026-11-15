use std::cmp::Ordering;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Rule, RuleId};

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

    /// Serialises in Canonical form: rules in natural Identifier order, keys in schema
    /// order, one property or array element per line, LF endings, trailing newline.
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
