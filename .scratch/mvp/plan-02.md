# Plan 02: `rules add` and Identifier allocation

## Interface

```rust
// model
pub struct IdPrefix(String);                       // moved from config; IdPrefix::parse(&str)
pub struct RuleDraft { .. }                        // RuleDraft::new(title, statement, apps, module, covered, excluded) -> Result<_, Error>
impl RuleSet { pub fn add(&mut self, prefix: &IdPrefix, draft: RuleDraft) -> Result<RuleId, Error>; }
// config
pub struct Config { pub id_prefix: IdPrefix, pub source_path: PathBuf }   // Config::load(root)
// adapter-json
pub fn read_source(path: &Path) -> Result<RuleSet, Error>;
pub fn write_source(path: &Path, set: &RuleSet) -> Result<(), Error>;      // atomic, Canonical form
// app
pub fn rules::add(root: &Path, draft: RuleDraft) -> Result<RuleId, AppError>;
```

One sentence: `rules::add` appends a validated draft to the Source as a `proposed` Rule and returns its new Identifier.

## Misuse

- Allocation and insertion are one call (`RuleSet::add`): the caller cannot allocate and forget to insert, or insert with a hand-made Identifier.
- A `RuleDraft` can only be built validated (private fields), so `add` cannot receive an empty title.
- Status is not a parameter: a Rule created here is always `proposed`.
- `add` computes everything, then mutates; a failure (overflow) leaves the set untouched. The file is written last, atomically, so a failed command leaves the Source as it was.

## Ownership

| Decision | Owner |
| --- | --- |
| Prefix syntax, Identifier format, never-reuse allocation, draft validation | model |
| Reading `rulelink.toml` (strict, relative path inside the project) | config |
| Reading/writing the file, atomic write, Canonical form | adapter-json |
| Orchestration, error class mapping | app |
| Arguments, printing the new Identifier | cli |

Allocation takes the max of `sequences[prefix]`, the numbers of the Rules still present and the retired Identifiers with that prefix, plus one. This covers a Rule deleted by hand and a `sequences` entry lowered by hand; `sequences` never decreases.

## Input boundaries

| Input | Line checked |
| --- | --- |
| `--title`, `--statement` | trimmed, non-empty, title 1–120 chars (counted on stored value) |
| `--app`, `--covered`, `--excluded` | each item trimmed, non-empty, no duplicates |
| `--module` | trimmed; empty is refused |
| `rulelink.toml` | version 1, `source.kind = "json"` only, `source.path` relative without `..`, prefix valid, missing section is an error |
| Source file | not found vs unreadable vs invalid JSON vs unsupported `format`; never overwritten when invalid |
| Sequence numbers | `checked_add`, named error on overflow |
| Failure | nothing is written; atomic temp-file + rename |
| Error class | invalid input → `AppError::Input` (exit 2); config → 2; Source → 3 |

## Smells

- Primitive obsession: `IdPrefix` moved to the model rather than passing `&str`. Fixed.
- Domain logic in app/cli: allocation and validation live in model. Fixed.
- Wrong error class: new `AppError::Input` instead of reusing `Config`. Fixed.
- Temporal coupling: none left (single `add`).

## Reuse

Searched `map_err`, `OpenOptions`, canonical writing: reuse `RuleSet::to_canonical`, `IdPrefix`, `AppError`, the `internal` mapper pattern of `init`. The write-new-file logic of `create_source` stays; `write_source` is the replace variant.

## Dependent tickets

| Ticket | Reuses | Missing |
| --- | --- | --- |
| 03 lifecycle | `read_source`/`write_source`, `RuleSet` mutation pattern, `retired` | transitions (theirs) |
| 04 search/related | `read_source`, `RuleDraft` (`related` to add later) | `related` option on `add` is theirs |
| 05 format | `read_source` + `write_source` (Canonical on every write) | `--check`, non-canonical detection |
| 06 scan/where | `Config::load`, `read_source` | exclusions config (theirs) |

Deferred and recorded: interactive input, `--id`, `--status`, `--dry-run`, `--origin` (not in the ticket). Concurrent-modification detection (exit 5) is not covered: inherited by a later ticket if needed.
