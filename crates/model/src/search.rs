//! Keyword search: text is compared without case and accents.

use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

use crate::{BlankSearchText, Rule};

/// Text to look for in Rules, already trimmed and folded.
///
/// It can only be built folded, so it is never compared against unfolded text by mistake.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchText(String);

impl SearchText {
    /// Reads the text a user typed.
    ///
    /// # Errors
    /// [`BlankSearchText`] if nothing is left after trimming.
    pub fn parse(raw: &str) -> Result<Self, BlankSearchText> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(BlankSearchText)
        } else {
            Ok(Self(fold(trimmed)))
        }
    }

    fn is_in(&self, text: &str) -> bool {
        fold(text).contains(&self.0)
    }
}

/// Lower case, with accents removed: `Énoncé` → `enonce`.
fn fold(text: &str) -> String {
    text.nfd()
        .filter(|c| !is_combining_mark(*c))
        .flat_map(char::to_lowercase)
        .collect()
}

impl Rule {
    /// Whether `text` appears in the Identifier, title, statement, module, applications or
    /// covered and excluded cases.
    pub(crate) fn contains(&self, text: &SearchText) -> bool {
        let scope = self
            .scope
            .iter()
            .flat_map(|s| s.covered.iter().chain(&s.excluded));
        [self.id.0.as_str(), &self.title, &self.statement]
            .into_iter()
            .chain(self.module.as_deref())
            .chain(self.applications.iter().map(String::as_str))
            .chain(scope.map(String::as_str))
            .any(|field| text.is_in(field))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(query: &str, text: &str) -> bool {
        SearchText::parse(query)
            .unwrap_or_else(|e| panic!("{e}"))
            .is_in(text)
    }

    #[test]
    fn ignores_case_and_accents_on_both_sides() {
        assert!(found("enonce", "Énoncé du devis"));
        assert!(found("ÉNONCÉ", "enonce"));
        assert!(found("naïve", "NAIVE"));
    }

    #[test]
    fn finds_a_substring_and_misses_other_text() {
        assert!(found("round", "VAT rounding"));
        assert!(!found("vat rounding", "VAT is rounded"));
    }

    #[test]
    fn surrounding_spaces_are_ignored_and_blank_is_refused() {
        assert!(found("  tva ", "TVA"));
        assert_eq!(SearchText::parse(" \t"), Err(BlankSearchText));
    }
}
