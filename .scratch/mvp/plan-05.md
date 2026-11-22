# Plan 05: `rules format` and Canonical-form integrity (issue #12)

## Interface

```rust
// adapter-json
pub enum Rewrite { Unchanged, Rewritten }
pub fn rewrite_source(path: &Path) -> Result<Rewrite, Error>;   // read, then write Canonical form only if different
// app
pub fn rules::format(root: &Path) -> Result<Formatted { path, rewrite }, Error>;  // the CLI prints the path
// model
impl RuleSet { pub fn is_canonical(&self, text: &str) -> bool; }   // owned by the model, reused by #15 (RL106)
// cli
rules format        (no argument, no option)
```

One sentence: `format` rewrites the Source into Canonical form if it is not already.

## Misuse

- No caller can format "a different file": the path comes from `rulelink.toml` via the same `Config::load` as every command; there is no path argument.
- A file that is not valid JSON of the right shape (conflict markers, wrong `format`) is refused by `read_source`'s code path and never overwritten: the formatter does not try to repair.
- The unchanged case does not touch the file (no mtime change, no editor reload), and the write is the existing atomic `write_source` (the one writer, no second path), so a failure leaves the previous file.
- `format` only changes layout and never repairs or judges content: a Source with duplicate Identifiers or a dangling `related` parses, keeps its content and is only re-laid-out; reporting those is `check`'s job (#15, RL003/RL103). Tested by neither, by design.
- Reading and parsing share one function with `read_source`; there is no second parser that could accept something `check` rejects.

## Ownership

| Decision | Owner |
| --- | --- |
| What Canonical form is (order, layout) | model (`RuleSet::to_canonical`, unchanged) |
| Read, compare bytes, atomic write | adapter-json (`rewrite_source`) |
| Which file, error class | app |
| Printing "formatted" / "already canonical" | cli |

## Input boundaries

| Input | Line checked |
| --- | --- |
| Arguments | none: the command takes no argument; clap refuses any extra one (usage error) |
| Source path | from `rulelink.toml`, same as the other commands; missing is `Error::Source` naming the path, unreadable is distinguished |
| Source content | parsed by the same code as `read_source`: invalid JSON, unknown field, foreign `format` refused, file untouched |
| Failure | atomic temporary file and rename; unchanged file is not rewritten |
| Line endings / BOM | CRLF or extra whitespace parse fine and are normalised to LF; a BOM is refused by serde as invalid JSON (named, file untouched) |

Errors: reuses `adapter-json::Error` and its explicit `match` in `app`; no new error type, no new class.

## Smells

- Temporal coupling: none, one call.
- Duplicated parsing: avoided by extracting `parse_source(path, text)` from `read_source`.
- Domain logic in `cli`: none, `cli` prints one of two lines.

## Reuse

`Config::load`, `read_source` (split into read text + parse), `write_source`, `RuleSet::to_canonical`, `Error` mapping, e2e `Repo`. Looked for another place that compares a file with its Canonical form: none yet.

## Dependent tickets

| Ticket | Reuses | Missing |
| --- | --- | --- |
| #15 RL106 "not canonical" Finding | the text-vs-canonical comparison | `rewrite_source` writes; #15 only needs the comparison. `RuleSet::is_canonical(text)` is built now, in the model, and used by `rewrite_source`; #15 reads the file text and calls it. |

Deferred: `--check` (report without writing; `check` does it through RL106), formatting several files (`json-dir` is V1), `--path`.

## Deferred after review

- Permissions and symlink of the Source are lost by the atomic rename (pre-existing in `write_source`, all write commands). Blocks none.
- `adapter-json::Error::Io` mixes causes and write failures carry no path (pre-existing). Blocks none.
- `Rewrite` lives in the adapter and is re-exported by `app`; moving it into `app` is cosmetic.
- Lost update between read and rename: accepted, no lock.
