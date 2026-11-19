use std::cmp::Ordering;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Error, Origin, Rule, RuleId, RuleStatus, Scope};

/// The `format` value of the current Source layout.
pub const FORMAT_VERSION: u32 = 1;

/// Longest accepted Rule title, in characters.
pub const TITLE_MAX_CHARS: usize = 120;

/// Minimum number of digits of an allocated Identifier (`RULE-001`).
const ID_DIGITS: usize = 3;

/// The fields a caller provides to create a Rule; the tool owns the Identifier and the status.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NewRule {
    /// Short title (1 to [`TITLE_MAX_CHARS`] characters).
    pub title: String,
    /// Statement of the rule.
    pub statement: String,
    /// Applications the rule applies to.
    pub applications: Vec<String>,
    /// Functional module.
    pub module: Option<String>,
    /// Cases the rule covers.
    pub covered: Vec<String>,
    /// Cases the rule excludes.
    pub excluded: Vec<String>,
    /// Where the rule comes from.
    pub origin: Option<Origin>,
    /// Related rules; each must exist.
    pub related: Vec<RuleId>,
}

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

    /// Creates a `proposed` Rule with the next Identifier of `prefix` and returns that Identifier.
    ///
    /// The Identifier is above every number ever seen for the prefix: the recorded sequence,
    /// the Rules present and the Retired identifiers. A Rule deleted by hand, or a sequence
    /// that was lost, therefore cannot make an old Identifier come back.
    ///
    /// # Errors
    /// [`Error::InvalidRule`] if `new` breaks the Rule model (nothing is changed), or
    /// [`Error::IdentifiersExhausted`] if the prefix has no number left.
    pub fn add_proposed(&mut self, prefix: &str, new: NewRule) -> Result<RuleId, Error> {
        let number = self
            .highest_number(prefix)
            .checked_add(1)
            .ok_or_else(|| Error::IdentifiersExhausted(prefix.to_owned()))?;
        let id = RuleId(format!("{prefix}-{number:0ID_DIGITS$}"));
        let rule = self.build_proposed(id.clone(), new)?;
        self.sequences.insert(prefix.to_owned(), number);
        self.rules.push(rule);
        Ok(id)
    }

    /// Checks `new` against the Rule model and builds the `proposed` Rule it describes.
    fn build_proposed(&self, id: RuleId, new: NewRule) -> Result<Rule, Error> {
        let title = new.title.trim();
        if title.is_empty() || title.chars().count() > TITLE_MAX_CHARS {
            return Err(Error::InvalidRule(format!(
                "the title must be 1 to {TITLE_MAX_CHARS} characters"
            )));
        }
        let statement = new.statement.trim();
        if statement.is_empty() {
            return Err(Error::InvalidRule("the statement must not be empty".into()));
        }
        if let Some(unknown) = new
            .related
            .iter()
            .find(|id| !self.rules.iter().any(|rule| rule.id == **id))
        {
            return Err(Error::InvalidRule(format!(
                "related rule {} does not exist",
                unknown.0
            )));
        }
        let scope = if new.covered.is_empty() && new.excluded.is_empty() {
            None
        } else {
            Some(Scope {
                covered: new.covered,
                excluded: new.excluded,
            })
        };
        Ok(Rule {
            id,
            title: title.to_owned(),
            statement: statement.to_owned(),
            status: RuleStatus::Proposed,
            applications: new.applications,
            module: new.module,
            scope,
            origin: new.origin,
            related: new.related,
            superseded_by: None,
        })
    }

    /// Highest number ever used with `prefix`, from the sequence, the Rules and the retired list.
    fn highest_number(&self, prefix: &str) -> u64 {
        let seen = self
            .rules
            .iter()
            .map(|rule| &rule.id)
            .chain(&self.retired)
            .filter_map(|id| id_number(&id.0, prefix));
        seen.chain(self.sequences.get(prefix).copied())
            .max()
            .unwrap_or(0)
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

/// The number of `id` if it is `<prefix>-<digits>`.
fn id_number(id: &str, prefix: &str) -> Option<u64> {
    let digits = id.strip_prefix(prefix)?.strip_prefix('-')?;
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok()
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
