//! The known name a misspelled one was probably meant to be.

/// The most single-character edits that still count as a misspelling.
const MAX_EDITS: usize = 2;

/// The name of `known` closest to `name`, if close enough to be a misspelling of it.
pub(crate) fn closest(name: &str, known: &[&'static str]) -> Option<&'static str> {
    let name = name.to_lowercase();
    known
        .iter()
        .map(|candidate| (edits(&name, candidate), *candidate))
        // Fewer edits than letters: otherwise any short name would "match".
        .filter(|(count, candidate)| *count <= MAX_EDITS && *count < candidate.len())
        .min_by_key(|(count, _)| *count)
        .map(|(_, candidate)| candidate)
}

/// How many characters to insert, remove or replace to turn `a` into `b` (Levenshtein).
fn edits(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    // `row[j]` is the distance between the part of `a` read so far and the first `j` of `b`.
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for (i, from) in a.chars().enumerate() {
        let mut diagonal = row[0];
        row[0] = i + 1;
        for (j, to) in b.iter().enumerate() {
            let replaced = diagonal + usize::from(from != *to);
            diagonal = row[j + 1];
            row[j + 1] = replaced.min(row[j] + 1).min(diagonal + 1);
        }
    }
    row[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misspellings_are_matched_and_unrelated_names_are_not() {
        let known = ["version", "source", "rules", "scan", "check"];
        for (typed, meant) in [
            ("sources", "source"),
            ("checks", "check"),
            ("scann", "scan"),
            ("Check", "check"),
            ("ruels", "rules"),
        ] {
            assert_eq!(closest(typed, &known), Some(meant), "{typed}");
        }
        for typed in ["lvls", "rules.prefx", "", "a"] {
            assert_eq!(closest(typed, &known), None, "{typed}");
        }
    }
}
