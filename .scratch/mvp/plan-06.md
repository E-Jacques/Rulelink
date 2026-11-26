# Plan 06: annotation scan and `where` (issue #13)

## Interface

```rust
// link (domain, pure)
pub struct Location { pub path: String, pub line: u32 }          // path relative to root, `/`-separated
pub fn annotations(text: &str) -> Vec<(RuleId, u32)>;              // the grammar: ids and 1-based line
pub struct LinkIndex;                                              // RuleId -> sorted Locations
impl LinkIndex { pub fn add_file(&mut self, path: &str, text: &str); pub fn locations(&self, &RuleId) -> &[Location]; }
// adapter-scan-fs
pub fn scan(root: &Path, exclude: &[String]) -> Result<LinkIndex, Error>;
// model
impl RuleSet { pub fn get(&self, &RuleId) -> Option<&Rule>; }
// config
Config { scan_exclude: Vec<String> }                               // optional `[scan] exclude = [...]`
// app
pub fn links::locate(root: &Path, id: &RuleId) -> Result<Located { status, locations }, Error>;
// cli
where <id>
```

One sentence: `where` scans the project for `@rule` Annotations and lists those citing one Identifier.

## Misuse

- Unknown Identifier: `Error::Input` naming it, before any scan runs (cheap, and no silent empty answer).
- Retired or uncited Identifiers are answers, not errors; `Located.status` lets the CLI word them differently.
- The grammar lives only in `link`; the adapter cannot disagree with it. `LinkIndex` is built by `add_file`, so a caller cannot insert an unsorted location (sorted on read by construction: BTreeMap + sorted at the end).
- A bad exclusion glob is `Error::InvalidExclude` naming the pattern, mapped to `Config`.

## Ownership

| Decision                                                                   | Owner                                      |
| -------------------------------------------------------------------------- | ------------------------------------------ |
| What an Annotation is (tag, comment openers, id shape)                     | link                                       |
| Which files are visited (ignore files, exclusions, size, binary, encoding) | adapter-scan-fs                            |
| Unknown / retired / uncited wording                                        | cli (status from app)                      |
| Location ordering                                                          | link (`LinkIndex` sorts by path then line) |

## Input boundaries

| Input           | Line checked                                                                                                                                                          |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `<id>`          | blank refused (`RuleId::parse`); unknown refused with the value                                                                                                       |
| `scan.exclude`  | each entry must be a valid gitignore glob; invalid names the pattern                                                                                                  |
| Files           | binary (NUL) and non-UTF-8 files skipped; files over 2 MB skipped; hidden files skipped; symlinks not followed; unreadable file is an error naming the path           |
| Ignore files    | `.gitignore`, `.ignore`, `.rulelinkignore`, honoured even outside a Git repository                                                                                    |
| Annotation text | `@rule` must be followed by whitespace and sit after a comment opener on its line; the id list stops at the first token that is not an Identifier (`--`, `*/`, `-->`) |

## Smells

- Duplicated lexing of the comment opener: kept as one small table in `link`.
- Lexical limits (accepted, spec says no tree-sitter): an opener inside a string literal before the tag counts as a comment; a block comment spanning lines is recognised only through `*` continuation or an opener on the same line.

## Reuse

`Config::load`, `read_source`, `RuleId::parse`, the `Error` mapping pattern, e2e `Repo`. `LinkIndex` is what `explain` (#14) and `check` RL004 (#15) need.

## Dependent tickets

| Ticket                  | Reuses                   | Missing                                                                                      |
| ----------------------- | ------------------------ | -------------------------------------------------------------------------------------------- |
| explain / annotate      | `LinkIndex`, `scan`      | `at(path, line)` query; added then                                                           |
| check RL004/RL005/RL006 | `LinkIndex`, `locations` | malformed Annotations are not reported yet (`annotations` drops an `@rule` with no valid id) |
