# Plan 04: Search, filter and `related` (`rules list`, `rules search`, `rules add --related`, issue #11)

## Interface

```rust
// model
impl FromStr for RuleStatus                      // Err = UnknownStatus { value } listing the four statuses
pub struct SearchText(String);                   // folded: trimmed, accents removed, lower case
impl SearchText { pub fn parse(raw: &str) -> Result<Self, BlankSearchText>; }
impl RuleSet {
    pub fn select(&self, text: Option<&SearchText>, status: Option<RuleStatus>) -> Vec<&Rule>; // Identifier order
    pub fn add(&mut self, prefix: &IdPrefix, draft: RuleDraft) -> Result<RuleId, AddError>;    // now checks `related`
}
RuleDraft::new(title, statement, applications, module, covered, excluded, related: &[RuleId])
pub enum AddError { IdentifierSpaceExhausted(String), UnknownRelated(RuleId) }
// app
pub fn rules::select(root: &Path, text: Option<&SearchText>, status: Option<RuleStatus>) -> Result<Vec<Rule>, Error>;
// cli
rules list [--status S]      rules search <TEXT> [--status S]      rules add ... [--related ID]...
```

One sentence: `select` reads the Source and returns the Rules that contain the text and have the status.

## Misuse

- `list` and `search` are one operation (`select`) with two optional filters: no second matching path that could disagree on accents.
- `SearchText` can only be built folded and non-blank: a caller cannot compare raw text with folded text, nor search for "".
- `related` is checked inside `RuleSet::add`, before the Identifier is allocated: no caller can add a dangling `related`, and a refusal leaves `sequences` untouched.
- Duplicate `related` is refused in `RuleDraft` (same line as `applications`); a self-reference is impossible on add (the Identifier does not exist yet).
- `select` is read-only: no Source write, so no partial failure.

## Ownership

| Decision                                                                    | Owner                                   |
| --------------------------------------------------------------------------- | --------------------------------------- |
| Folding (case, accents), which fields are searched, status filter, ordering | model (`SearchText`, `RuleSet::select`) |
| Status parsing (`FromStr`), `related` validity and uniqueness               | model                                   |
| Load config and Source, error class                                         | app                                     |
| Arguments, table printing                                                   | cli                                     |

## Input boundaries

| Input           | Line checked                                                                                                                                                                                                |
| --------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `<text>`        | trimmed, blank refused (`BlankSearchText`, naming "the search text", exit 2); folded with NFD, combining marks dropped, lower-cased; matched as a substring of the whole text (no word splitting, no regex) |
| `--status`      | closed set, parsed by clap through `FromStr`; unknown value is a usage error naming the value and the allowed ones                                                                                          |
| `--related`     | `RuleId::parse` (trim, blank refused naming `--related`); duplicate refused (`DraftError::Duplicate`); unknown refused (`AddError::UnknownRelated` names the Identifier, exit 2)                            |
| Source / config | same helpers as the other commands; `list` and `search` never write                                                                                                                                         |
| Failure         | refused `add` writes nothing (check before allocation, atomic write)                                                                                                                                        |
| Paths           | not applicable, no path argument                                                                                                                                                                            |

Errors: one type per cause (`UnknownStatus`, `BlankSearchText`, `AddError`), converted in `app` by explicit match / single-class `From`; no `#[non_exhaustive]`.

## Smells

- Wrong error class: `IdentifierSpaceExhausted` was in `model::Error` and `add` returned it; unknown `related` is input, exhaustion is Source. Split into `AddError`, matched variant by variant.
- Primitive obsession: `--status` and the search text are typed at the edge, not passed as `String`.
- Domain logic in `cli`: folding and matching are in the model, `cli` only prints.
- Temporal coupling: none, `add` validates then mutates.

## Reuse

`RuleId::parse`, `unique_items`-style duplicate check in `RuleDraft`, `natural_cmp` for ordering, `Config::load` + `read_source` (read half of `update_source`; not extracted, one read-only caller), `Display for RuleStatus`, the e2e `Repo` harness. New dependency: `unicode-normalization` (MIT/Apache-2.0, accepted by `deny.toml`) for accent folding; a hand-written accent table would miss languages.

## Dependent tickets

`gh api .../issues/11/dependencies/blocking` returns nothing; tickets that will touch this code:

| Ticket                      | Reuses                                  | Missing                                                                         |
| --------------------------- | --------------------------------------- | ------------------------------------------------------------------------------- |
| #15 RL103 unknown `related` | `Rule.related`, the same existence rule | a hand-edited Source can still hold a dangling `related`: the Finding is theirs |
| #18 explain `--status`      | `RuleStatus: FromStr`                   | none                                                                            |
| #20 context `--query`       | `SearchText`, `RuleSet::select`         | ranking (theirs)                                                                |
| #16 `--format json`         | `select` returns `Rule`s                | `list`/`search` have human output only                                          |

Deferred, recorded: `--limit`, `--format`, `--app`/`--module` filters, ranking, `rules edit` to change `related`, cycle detection (out of scope in the issue). Search tokens are not split: `"vat rounding"` must appear as that exact phrase.
