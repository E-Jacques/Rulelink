//! Code linking: the Annotation grammar and the index of where each Rule is cited.
//!
//! Lexical and language-agnostic: an Annotation is `@rule` followed by Identifiers, on a line
//! that has a comment opener before it. No syntax tree, no enclosing symbol.

use std::collections::HashMap;

use rulelink_model::RuleId;

/// The tag that introduces an Annotation.
const TAG: &str = "@rule";

/// Openers of a comment in the common languages (`//`, `#`, `--`, `/* */`, `<!-- -->`, ...).
/// A line that starts with `*` continues a block comment and counts too.
const COMMENT_OPENERS: [&str; 7] = ["//", "#", "--", "/*", "<!--", ";", "%"];

/// One place that cites a Rule.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    /// File, relative to the project root, `/`-separated.
    pub path: String,
    /// Line, starting at 1.
    pub line: u32,
}

/// The Identifiers cited by the Annotations in `text`, each with its 1-based line.
///
/// An `@rule` with no valid Identifier after it is not an Annotation and is dropped.
#[must_use]
pub fn annotations(text: &str) -> Vec<(RuleId, u32)> {
    let mut found = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let number = u32::try_from(index + 1).unwrap_or(u32::MAX);
        for id in line_identifiers(line) {
            found.push((id, number));
        }
    }
    found
}

/// Identifiers of every Annotation on one line.
fn line_identifiers(line: &str) -> Vec<RuleId> {
    let mut ids = Vec::new();
    for (at, _) in line.match_indices(TAG) {
        let (before, after) = (&line[..at], &line[at + TAG.len()..]);
        let spaced = after.chars().next().is_some_and(char::is_whitespace);
        if spaced && in_comment(before) {
            ids.extend(identifier_list(after));
        }
    }
    ids
}

fn in_comment(before: &str) -> bool {
    before.trim_start().starts_with('*') || COMMENT_OPENERS.iter().any(|o| before.contains(o))
}

/// The leading Identifiers of `text`, separated by spaces or commas; stops at anything else
/// (`--`, `*/`, `-->`, a note).
fn identifier_list(text: &str) -> impl Iterator<Item = RuleId> + '_ {
    text.split(|c: char| c.is_whitespace() || c == ',')
        .filter(|token| !token.is_empty())
        .map(|token| token.trim_end_matches(['.', ':', ';', ')', ']', '"', '\'']))
        .map_while(|token| is_identifier(token).then(|| RuleId(token.to_owned())))
}

/// `PREFIX-SUFFIX`: an upper-case prefix like the ones `init` accepts, one hyphen, then
/// upper-case letters, digits or `_`.
fn is_identifier(token: &str) -> bool {
    let Some((prefix, suffix)) = token.split_once('-') else {
        return false;
    };
    let mut chars = prefix.chars();
    let word = |c: char| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_';
    chars.next().is_some_and(|c| c.is_ascii_uppercase())
        && chars.all(word)
        && !suffix.is_empty()
        && suffix.chars().all(word)
}

/// Where each Rule is cited, built file by file.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LinkIndex {
    by_rule: HashMap<RuleId, Vec<Location>>,
}

impl LinkIndex {
    /// An index with no Annotation.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records every Annotation of `text`, the content of the file `path`.
    pub fn add_file(&mut self, path: &str, text: &str) {
        for (id, line) in annotations(text) {
            let locations = self.by_rule.entry(id).or_default();
            let location = Location {
                path: path.to_owned(),
                line,
            };
            // Two Annotations of one Rule on one line are one location.
            if !locations.contains(&location) {
                locations.push(location);
            }
        }
    }

    /// The places that cite `id`, sorted by path then line; empty if there are none.
    #[must_use]
    pub fn locations(&self, id: &RuleId) -> Vec<Location> {
        let mut found = self.by_rule.get(id).cloned().unwrap_or_default();
        found.sort();
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(text: &str) -> Vec<(String, u32)> {
        annotations(text)
            .into_iter()
            .map(|(id, line)| (id.0, line))
            .collect()
    }

    #[test]
    fn finds_annotations_after_every_common_comment_opener() {
        for line in [
            "// @rule RULE-1",
            "x = 1 # @rule RULE-1",
            "-- @rule RULE-1",
            "/* @rule RULE-1 */",
            "<!-- @rule RULE-1 -->",
            " * @rule RULE-1",
            "; @rule RULE-1",
            "% @rule RULE-1",
        ] {
            assert_eq!(ids(line), [("RULE-1".to_owned(), 1)], "{line}");
        }
    }

    #[test]
    fn an_identifier_list_stops_at_a_note_or_a_closer() {
        assert_eq!(ids("// @rule RULE-1, RULE-2 RULE-3 -- RULE-4").len(), 3);
        assert_eq!(ids("/* @rule RULE-1 */ RULE-9").len(), 1);
    }

    #[test]
    fn trailing_punctuation_does_not_hide_an_annotation() {
        for line in [
            "// @rule RULE-1.",
            "// @rule RULE-1: why",
            "// see (@rule RULE-1)",
        ] {
            assert_eq!(ids(line), [("RULE-1".to_owned(), 1)], "{line}");
        }
    }

    #[test]
    fn text_outside_a_comment_or_without_an_identifier_is_not_an_annotation() {
        assert_eq!(ids("let s = \"@rule RULE-1\";").len(), 0);
        assert_eq!(ids("// @rule").len(), 0);
        assert_eq!(ids("// @rule rule-1").len(), 0);
        assert_eq!(ids("// @rules RULE-1").len(), 0);
        assert_eq!(ids("// @ruleRULE-1").len(), 0);
    }

    #[test]
    fn lines_are_one_based_and_several_annotations_are_all_reported() {
        assert_eq!(
            ids("a\n// @rule RULE-1\n\n# @rule RULE-2 @rule RULE-3"),
            [
                ("RULE-1".to_owned(), 2),
                ("RULE-2".to_owned(), 4),
                ("RULE-3".to_owned(), 4)
            ]
        );
    }

    #[test]
    fn the_index_sorts_locations_and_merges_duplicates() {
        let mut index = LinkIndex::new();
        index.add_file("b.rs", "// @rule RULE-1\n");
        index.add_file("a.rs", "x\n// @rule RULE-1 @rule RULE-1\n// @rule RULE-1\n");
        let id = RuleId("RULE-1".to_owned());
        let got: Vec<_> = index
            .locations(&id)
            .into_iter()
            .map(|l| (l.path, l.line))
            .collect();
        assert_eq!(
            got,
            [("a.rs".into(), 2), ("a.rs".into(), 3), ("b.rs".into(), 1)]
        );
        assert_eq!(index.locations(&RuleId("RULE-2".to_owned())).len(), 0);
    }
}
