# Plan: `check --format json` (#16)

## Decisions

Taken with the maintainer before implementation.

| # | Decision |
|---|---|
| 1 | #15 is reopened and comes after this ticket. `rule` is optional in the schema |
| 2 | A Finding about a Rule that no code cites has the rules file as `file`, and no `line` |
| 3 | A value that does not apply is left out, never `null` |
| 4, 5 | v1 accepts new codes and new fields (ADR-0006). The tests refuse undeclared fields |
| 6 | `schema`, `findings`, `totals.errors`, `totals.warnings` |
| 7 | The "link, not correctness" sentence is not in the JSON report |
| 8, 9 | No no-Rules flag, no configuration warnings (they stay on stderr) |
| 10 | A run without a report prints text on stderr and nothing on stdout |
| 11 | `human` by default, piped or not |
| 12 | `schemas/check-report.schema.json`, linked from the `check` page |
| 13 | The report types live in `rulelink-check` |

Defaults chosen without asking, announced to the maintainer: `severity` is a closed list; order and `off` as in the human report; paths are `/`-separated from the project root without `./`; `message` is the human one; only `check` gets `--format`; the report is indented by two spaces.

## Interface

```rust
// rulelink-check, module report (JSON Schema behind the `schema` feature, as in rulelink-model)
pub struct CheckReportV1 { /* private: schema, findings, totals */ }   // Serialize
impl CheckReportV1 {
    pub fn new(report: &Report, source_path: &Path) -> Self;
}

// rulelink-app
impl Checked { pub fn report_v1(&self) -> CheckReportV1; }
```

- **Misuse**: the fields are private and `new` is the only constructor, so the totals cannot disagree with the Findings. `Checked::report_v1` passes the right rules file path itself.
- **Ownership**: `rulelink-check` owns the wire shape, including how the rules file path is written. The CLI parses `--format`, serialises and prints. Exit code comes from the same `Report` for both formats, computed in one place.
- **One sentence**: `report_v1` gives the published form of a check result.

The types are separate from `Finding`: #15 can reshape it without changing the published format.

Smells: display text in the core does not apply, `message` is already in `rulelink-check` and now has two outputs. Primitive obsession: `--format` is a `clap` value enum. No other smell found.

## Input boundaries

The one new input is `--format`.

| Check | `--format` |
|---|---|
| Parse at the edge | `clap` value enum; CLI only, no model type |
| Normalise and bound | closed list, case-sensitive; empty refused |
| Strict configuration | not applicable: not a setting |
| Paths | not applicable to the option. The rules file path is already a checked `ProjectPath`; `.` segments are dropped and separators become `/` |
| Failure | `check` writes no file; a refused value runs nothing |
| Errors | usage error, exit 2, names the option and the accepted values |
| Tests | `tests/e2e/tests/check_json.rs` |

Output is a boundary too: Rule text is not in the report. The JSON serialiser escapes only the control characters below U+0020. The CLI also escapes DEL and U+0080 to U+009F anywhere in the report (added after review). #20 prints Rule text as JSON and needs the same escaping.

## Reuse

Looked for existing JSON output (none in the CLI), schema generation (`xtask schemas`: extended with one entry and its settings) and path rendering (`scan-fs` produces `/`-separated scan paths; it works on walked file paths, not on a configured one). Counting reuses `Report::count`.

## Dependent tickets

| Ticket | Reuses | Missing |
|---|---|---|
| #20 `context` JSON | ADR-0006, the `schema` field, `xtask schemas`, the closed-schema test technique, `--format` value enum | its own report types, outside `rulelink-check`'s Findings; the control-character escaping, private to `check` today, to share before a second JSON output; its formats in the schema, as `written` removes every `format`, not only the integer ones |
| #22 skills | the schema and the `check` page as report reference | nothing for `check` |
| #15 (not blocked, follows) | `CheckReportV1` | it makes `Finding::rule` optional and maps it to the already optional `rule`; it may need a `line` in the rules file, already optional |

## Seams

The CLI process only, as spec #7 asks: `tests/e2e/tests/check_json.rs`. The schema check uses `jsonschema` without default features (no network), a dev-only use in `rulelink-e2e`.
