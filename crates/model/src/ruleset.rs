use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{AddError, IdPrefix, Rule, RuleDraft, RuleId, RuleStatus, SearchText};

/// The `format` value of the current Source layout.
pub const FORMAT_VERSION: u32 = 1;

/// The content of a Source: its rules and the bookkeeping that keeps Identifiers unique.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
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
    /// Every related Rule of `draft` must exist in the Source.
    ///
    /// The number follows the highest of: the recorded sequence, the Rules present and the
    /// retired Identifiers. A Rule removed from the Source by hand therefore never has its
    /// Identifier reused, and `sequences` never decreases. Nothing changes on error.
    ///
    /// # Errors
    /// [`AddError::UnknownRelated`] for a related Rule that is not in the Source,
    /// [`AddError::IdentifierSpaceExhausted`] if the number would overflow.
    pub fn add(&mut self, prefix: &IdPrefix, draft: RuleDraft) -> Result<RuleId, AddError> {
        if let Some(unknown) = draft.related().iter().find(|id| self.find(id).is_none()) {
            return Err(AddError::UnknownRelated(unknown.clone()));
        }
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
            .ok_or_else(|| AddError::IdentifierSpaceExhausted(prefix.as_str().to_owned()))?;
        let id = prefix.identifier(next);
        self.sequences.insert(prefix.as_str().to_owned(), next);
        self.rules.push(draft.into_proposed(id.clone()));
        Ok(id)
    }

    /// The Rules that contain `text` (any, if `None`) and are in `status` (any, if `None`),
    /// in natural Identifier order.
    #[must_use]
    pub fn select(&self, text: Option<&SearchText>, status: Option<RuleStatus>) -> Vec<&Rule> {
        let mut found: Vec<&Rule> = self
            .rules
            .iter()
            .filter(|r| status.is_none_or(|s| r.status == s))
            .filter(|r| text.is_none_or(|t| r.contains(t)))
            .collect();
        found.sort_by(|a, b| a.id.natural_cmp(&b.id));
        found
    }

    /// Serialises in Canonical form: rules in natural Identifier order, keys in schema
    /// order, one property or array element per line, LF endings, trailing newline.
    ///
    /// # Panics
    /// Never in practice: a `RuleSet` only holds string-keyed maps and plain values.
    #[must_use]
    pub fn to_canonical(&self) -> String {
        let mut sorted = self.clone();
        sorted.rules.sort_by(|a, b| a.id.natural_cmp(&b.id));
        sorted.retired.sort_by(RuleId::natural_cmp);
        let mut out = serde_json::to_string_pretty(&sorted)
            .expect("a RuleSet always serialises: string keys, no custom serialisers");
        out.push('\n');
        out
    }

    /// Whether `text` is exactly this Source in Canonical form, byte for byte.
    #[must_use]
    pub fn is_canonical(&self, text: &str) -> bool {
        text == self.to_canonical()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draft() -> RuleDraft {
        RuleDraft::new("T", "S", &[], None, &[], &[], &[]).unwrap_or_else(|e| panic!("{e}"))
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
            Err(AddError::IdentifierSpaceExhausted("RULE".into()))
        );
        assert_eq!(set, before);
    }

    fn draft_related(related: &[&str]) -> RuleDraft {
        let ids: Vec<RuleId> = related.iter().map(|r| RuleId((*r).into())).collect();
        RuleDraft::new("T", "S", &[], None, &[], &[], &ids).unwrap_or_else(|e| panic!("{e}"))
    }

    #[test]
    fn related_rules_are_stored_on_the_new_rule() {
        let mut set = RuleSet::empty();
        add(&mut set);
        add(&mut set);
        let id = set
            .add(
                &IdPrefix::default(),
                draft_related(&["RULE-002", "RULE-001"]),
            )
            .unwrap_or_else(|e| panic!("{e}"));
        let related = &set.find(&id).expect("added").related;
        assert_eq!(
            related,
            &[RuleId("RULE-002".into()), RuleId("RULE-001".into())]
        );
    }

    #[test]
    fn an_unknown_related_rule_is_refused_before_any_identifier_is_allocated() {
        let mut set = RuleSet::empty();
        add(&mut set);
        let before = set.clone();
        assert_eq!(
            set.add(
                &IdPrefix::default(),
                draft_related(&["RULE-001", "RULE-009"])
            ),
            Err(AddError::UnknownRelated(RuleId("RULE-009".into())))
        );
        assert_eq!(set, before);
    }

    fn select_ids(set: &RuleSet, text: Option<&str>, status: Option<RuleStatus>) -> Vec<String> {
        let text = text.map(|t| SearchText::parse(t).unwrap_or_else(|e| panic!("{e}")));
        set.select(text.as_ref(), status)
            .into_iter()
            .map(|r| r.id.0.clone())
            .collect()
    }

    /// RULE-001..RULE-010; RULE-002 validated and titled "Énoncé", the others `proposed`.
    fn ten_rules() -> RuleSet {
        let mut set = RuleSet::empty();
        for _ in 0..10 {
            add(&mut set);
        }
        set.rules[1].status = RuleStatus::Validated;
        set.rules[1].title = "Énoncé".into();
        set.rules[9].module = Some("Facturation".into());
        set
    }

    #[test]
    fn select_without_filter_returns_every_rule_in_natural_order() {
        let ids = select_ids(&ten_rules(), None, None);
        assert_eq!(ids.len(), 10);
        assert_eq!((ids[1].as_str(), ids[9].as_str()), ("RULE-002", "RULE-010"));
    }

    #[test]
    fn select_filters_by_status() {
        let set = ten_rules();
        assert_eq!(
            select_ids(&set, None, Some(RuleStatus::Validated)),
            ["RULE-002"]
        );
        assert_eq!(
            select_ids(&set, None, Some(RuleStatus::Rejected)),
            Vec::<String>::new()
        );
    }

    #[test]
    fn select_by_text_looks_in_the_module_too_and_combines_with_status() {
        let set = ten_rules();
        assert_eq!(select_ids(&set, Some("ENONCE"), None), ["RULE-002"]);
        assert_eq!(select_ids(&set, Some("facturation"), None), ["RULE-010"]);
        assert_eq!(
            select_ids(&set, Some("enonce"), Some(RuleStatus::Proposed)),
            Vec::<String>::new()
        );
    }
}
