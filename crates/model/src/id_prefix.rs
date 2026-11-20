use crate::{InvalidIdPrefix, RuleId};

/// Default Identifier prefix.
pub const DEFAULT_ID_PREFIX: &str = "RULE";

/// An Identifier prefix: an upper-case letter followed by upper-case letters, digits or `_`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdPrefix(String);

impl IdPrefix {
    /// Validates a prefix.
    ///
    /// # Errors
    /// Returns [`InvalidIdPrefix`] if `raw` does not match `[A-Z][A-Z0-9_]*`.
    pub fn parse(raw: &str) -> Result<Self, InvalidIdPrefix> {
        let mut chars = raw.chars();
        let valid = chars.next().is_some_and(|c| c.is_ascii_uppercase())
            && chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
        if valid {
            Ok(Self(raw.to_owned()))
        } else {
            Err(InvalidIdPrefix(raw.to_owned()))
        }
    }

    /// The prefix as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The Identifier of number `n` under this prefix, zero-padded to three digits (`RULE-007`).
    #[must_use]
    pub fn identifier(&self, n: u64) -> RuleId {
        RuleId(format!("{}-{n:03}", self.0))
    }

    /// The number of `id` if it is `<prefix>-<digits>`, otherwise `None`.
    #[must_use]
    pub fn number_of(&self, id: &RuleId) -> Option<u64> {
        let digits = id.0.strip_prefix(self.0.as_str())?.strip_prefix('-')?;
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        digits.parse().ok()
    }
}

impl Default for IdPrefix {
    fn default() -> Self {
        Self(DEFAULT_ID_PREFIX.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_letters_digits_and_underscore() {
        assert!(IdPrefix::parse("BR_2").is_ok());
    }

    #[test]
    fn rejects_empty_lowercase_and_leading_digit() {
        for raw in ["", "br", "2BR", "B R", "B-R"] {
            assert_eq!(IdPrefix::parse(raw), Err(InvalidIdPrefix(raw.to_owned())));
        }
    }

    #[test]
    fn identifier_is_zero_padded_but_grows_past_three_digits() {
        let prefix = IdPrefix::default();
        assert_eq!(prefix.identifier(7).0, "RULE-007");
        assert_eq!(prefix.identifier(1234).0, "RULE-1234");
    }

    #[test]
    fn number_of_reads_only_its_own_prefix() {
        let prefix = IdPrefix::default();
        assert_eq!(prefix.number_of(&RuleId("RULE-042".into())), Some(42));
        assert_eq!(prefix.number_of(&RuleId("RULE-FACT_3".into())), None);
        assert_eq!(prefix.number_of(&RuleId("RULES-042".into())), None);
        assert_eq!(prefix.number_of(&RuleId("RULE-".into())), None);
        assert_eq!(prefix.number_of(&RuleId("OTHER-042".into())), None);
        assert_eq!(
            prefix.number_of(&RuleId("RULE-99999999999999999999999".into())),
            None
        );
    }
}
