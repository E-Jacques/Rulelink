# Plan: #15 Source-integrity Findings

Surfaces: library (`model`, `check`, `config`), JSON adapter, app, CLI. Contract: `docs/public/concepts/findings.md`.

Decided with the user (2026-11-27): the short names (`duplicate-id`, `invalid-shape`, `invalid-identifier`, `reused-retired-id`, `unknown-related`, `empty-field`, `not-canonical`); RL101 covers `id`, `retired`, `related` and `superseded_by`; the human report says when the code was not checked.

## 1. Plan

### Interface

```rust
// model: the grammar moves here from `link`, which calls it.
impl RuleId { pub fn is_well_formed(&self) -> bool; }

// check
pub enum Source<'a> {
    Rules { rules: &'a RuleSet, text: &'a str },
    WrongShape { detail: &'a str, line: Option<u32> },
}
pub fn check(source: Source<'_>, links: &LinkIndex, levels: &Levels) -> Report;
pub struct Report { pub findings: Vec<Finding>, pub code_check: CodeCheck }
pub enum CodeCheck { Run, RunWithoutRules, Stopped }
pub struct Finding { severity, rule: Option<RuleId>, problem: Problem, place: Place }
pub enum Place { Code(Location), RulesFile { line: Option<u32> } }
impl Levels { pub fn set(&mut self, Code, Level) -> Result<(), FixedLevel>; }

// adapter-json
pub enum Inspected { Rules { rules: RuleSet, text: String }, WrongShape { detail: String, line: Option<u32> } }
pub fn inspect_source(path: &Path) -> Result<Inspected, Error>;
```

### Misuse

- Running the code checks on a broken Source: impossible, `check` is the only entry and it decides.
- Passing a wrong "is canonical" flag: there is none, the engine gets the text and compares.
- Lowering a Source error: `Levels::set` refuses it, so no `Levels` value can hold it.

### Ownership

| Decision | Owner |
| --- | --- |
| Identifier grammar | `model` |
| The seven checks, which codes stop the code checks, which levels are fixed | `check` |
| JSON syntax vs shape vs `format` | `adapter-json` |
| Configuration error for a refused level | `config` |
| "The code is not checked" line | `cli` |

### One sentence

`checks::run` reports the Findings of the project.

### Smells

- Primitive obsession: `Level` refusal is typed (`FixedLevel`), the stop rule is a method on `Code`, not a severity test (RL106 raised to `error` must not stop).
- Catch-all: the adapter's `Invalid` (exit 3, not JSON) is split from `WrongShape` (Finding for `check`, exit 3 elsewhere).
- Temporal coupling: none, one function.

## 2. DRY

Looked for: the Identifier grammar (`link::is_identifier`, moved to the model), the canonical comparison (`RuleSet::is_canonical`, reused), `RuleSet::find` (reused for RL103), level parsing in `config` (extended).

## 3. Roadmap

No ticket is declared as blocked by #15.

| Ticket | Reuses | Missing |
| --- | --- | --- |
| #63 property reference | the names of `[check.levels]` | nothing |
| #60 `doctor` | `check`, `inspect_source` | nothing known |
| #59 `rules edit` | `RuleId::is_well_formed` | nothing |

## Input boundaries

| Item | Source file | `[check.levels]` |
| --- | --- | --- |
| Parse at the edge | adapter returns `RuleSet` or a shape problem | `Code`, `Level` |
| Normalise and bound | required fields are tested trimmed; nothing is rewritten | n/a |
| Duplicates, self-references | RL003; self-supersession is RL104 (V1) | TOML refuses duplicates |
| `checked_*` | no arithmetic added | n/a |
| Strict | unknown keys are RL100; an array or a map where an object or a status belongs is refused too (serde would accept it by position) | fixed levels are refused |
| Paths | unchanged | n/a |
| Failure | `check` writes nothing | n/a |
| Errors | missing, unreadable, not JSON, unsupported format: exit 3 | exit 2, naming the key |
| Output | detail and Identifiers come from the file: the CLI already escapes control characters | n/a |

## Slices

1. Adapter: `WrongShape` and `inspect_source`.
2. Model grammar.
3. Engine: `Source`, RL100, then RL003, RL101, RL102, RL105, RL103, RL106, stop rule.
4. Fixed levels, config.
5. App, CLI, JSON report, architecture docs.

## Known limits

- RL100 reports the first problem only (serde stops there).
- An array in the place of an object has no line.
- RL104 is V1: a dangling `superseded_by` is not reported.
