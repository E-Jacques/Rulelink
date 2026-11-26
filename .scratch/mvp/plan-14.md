# Plan: #14, `check` core

## Interface

```rust
// rulelink-check: pure, no I/O
pub fn check(rules: &RuleSet, links: &LinkIndex, levels: &Levels) -> Report;
pub struct Report { pub findings: Vec<Finding>, pub no_rules: bool }   // errors(), warnings()
pub struct Finding { pub severity: Severity, pub rule: RuleId, pub problem: Problem,
                     pub location: Option<Location> }                  // None: about the Rule itself
pub enum Problem { Unknown, Obsolete { superseded_by }, Orphan, Unvalidated, Rejected, Retired }
pub enum Code { UnknownReference, ObsoleteReference, OrphanRule, UnvalidatedReference,
                RetiredReference }                                     // id() "RL001", name(), FromStr(name)
pub enum Level { Error, Warning, Off }                                 // FromStr
pub struct Levels(..)                                                  // set(code, level); only explicit ones
pub const PROMISE: &str;

// rulelink-app
pub fn checks::run(root: &Path) -> Result<Checked { report, source_path }, Error>;
```

One sentence: `checks::run` reports the Findings of the project at `root`.

- **Misuse:** none to chain. The default level, including the no-Rules case, is resolved inside `check`.
- **Ownership:** names, levels, defaults, messages and order in `rulelink-check`; reading `[check.levels]` in `config`, which gets typed `Levels`; reading and scanning in `app`; the line layout, totals and exit code in `cli`.
- `config` depends on `check` (it parses into its types, as it does with `model`'s `IdPrefix`). `check` depends on `model` and `link`; the unused `registry` dependency and the stub `Error` go.
- `LinkIndex` gains `cited()`, the Identifiers it holds. `RuleId` gains `natural_cmp`, moved from `ruleset.rs`, so reports order `RULE-2` before `RULE-10`.
- Exit code 1 uses the existing `Outcome::ChecksFailed`: `dispatch` returns an `Outcome`.

## Which Finding

| Cited Identifier                 | Finding                  |
| -------------------------------- | ------------------------ |
| Rule `validated`                 | none                     |
| Rule `proposed`                  | RL005                    |
| Rule `obsolete`                  | RL002                    |
| Rule `rejected`                  | RL009                    |
| no Rule, listed in `retired`     | RL009                    |
| no Rule, not retired             | RL001 (warning while the Source has no Rules, unless a level is set) |

RL004: a `validated` Rule with no location. The status decides, not `retired`: a live Rule listed as retired is RL102 (#15). Duplicated Identifiers: the first Rule decides until RL003 stops the code checks (#15).

Order: errors first, then code, Identifier, path, line.

## Input boundaries

| Input | Parse | Bounds, strictness | Errors |
| ----- | ----- | ------------------ | ------ |
| `check` arguments | none | clap refuses any | usage, 2 |
| `[check]` | `config` | any key but `levels` refused (no overrides) | config, 2, names the key |
| `[check.levels]` keys | `Code::from_str` | exact name; codes such as `RL004` refused; TOML refuses duplicates | config, 2, names it and lists the names |
| `[check.levels]` values | `Level::from_str` | exact `error`, `warning`, `off`; non-strings refused | config, 2, names key and value |
| Source text in the report (`superseded_by`, Identifiers) and file paths | `cli` | control characters escaped, so one Finding stays one line | n/a |

Paths, atomic writes, arithmetic: not applicable, `check` reads only, through the existing Source reader and scan.

## DRY

Reused: `read_source`, `scan`, `LinkIndex`, `Location`, `RuleSet::find`, `natural_cmp`, `Outcome::ChecksFailed`, e2e `Repo`. No escaping helper exists yet.

## Dependent tickets

| Ticket | Reuses | Missing, left to it |
| ------ | ------ | ------------------- |
| #15 | `Code`, `Level`, `Levels`, `Report` | RL003/RL1xx variants, refusing to lower them, stopping code checks. A wrong-shape Source still exits 3 |
| #16 | `Finding` (code, name, severity, message, Rule, location), totals | JSON rendering, `--format` |
| #17 | nothing: closed as not planned, there is no `check --changed` | n/a |
| #23 | `check` exits 0 right after `init` | enabling the smoke test step |

## Changed after review

The scan skips the rules file, for `check` and `where`: Rule text quoting `@rule` hid RL004. Identifiers of equal numeric value (`RULE-5`, `RULE-05`) are ordered by their text. `Code::name` is not exported until #16 calls it.

## Left to dependent tickets

- #15 and #16: `Finding::rule` is required. A Source Finding (RL100, RL106) may have no Rule, so #15 reshapes `Finding`; do it before #16 fixes the JSON schema.
- #15: refusing to lower a Source error belongs in `rulelink-check` (make `Levels::set` fallible), not in `config`.
- #16: "no location means the rules file" is decided by the human renderer; move it where both renderers can use it.

## Slices

Tests: `tests/e2e/tests/check.rs` (the CLI process is the seam, per the spec), a doctest on `check`.
