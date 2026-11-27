# Plan: per-property configuration parsing and surface-owned output

Spec: [architecture.md](architecture.md), decisions D1 to D6. There is no GitHub issue.

## Interface

```rust
// adapter-sdk
trait Property { const KEY: &str; fn parse(Option<&RawValue>) -> ParseResult<Self>; }
trait AdapterConfig { fn kind(&self) -> &'static str; fn parse(&self, RawTable) -> ParseResult<ConfigMap>; }
TableParser::new(table).property::<P>().section(name, parse).reserved(name).finish() -> ParseResult<ConfigMap>
// config
load(root, &AdapterCatalog) -> Result<Parsed<ConfigMap>, config::Error>
// app
Workspace::open(root) -> Result<Parsed<Workspace>, app::Error>
```

Seams under test: `TableParser`, each owner's parse, `config::load`, `Workspace`, the binary.

**Misuse.** A property declared nowhere makes `get` fail (internal error, covered by one test per owner). A use case cannot run before the configuration is read, because it takes a `Workspace`. `ConfigMap` cannot be built or edited outside the SDK.

**Ownership.** SDK: accumulation, unknown keys, suggestions, `ProjectPath`. Each property: presence, type, default, validation. `config`: file reading, TOML to `RawValue`, `version` first, kind lookup. `app`: the catalog and error classes. CLI: all output.

**One sentence.** `load` turns `rulelink.toml` into the map each owner validated.

**Smells.** Temporal coupling: `Workspace` removes the separate warning pass. Wrong error class: `UndeclaredProperty` is internal. Catch-all: `config::Error` keeps one variant per cause.

## Slices

1. SDK contract.
2. Owners: `ScanExclude`, `SourcePath`, then the core properties.
3. `config::load` with the catalog.
4. `Workspace`, use cases, error conversion.
5. `Presenter` and `Entrypoint::report_failure`.

## DRY

Reused: `IdPrefix::parse`, the `Code` and `Level` parsers of `check`, the override builder of `scan-fs`. Moved, not copied: the relative path check (`ProjectPath`) and the escaping of unknown names.

## Dependent tickets

No issue, so no GitHub dependency list. Tickets that read the configuration:

| Ticket | Reuses | Missing |
| --- | --- | --- |
| #60 `doctor` | every configuration problem in one run | the Source location when parsing fails (D6) |
| #63 property reference | one owner per property | the pages |
| #16 JSON report | warnings as data | a schema decision |
| #15, #18, #20, #21 | `Workspace` instead of loading `Config` | nothing |

## Input boundaries (`rulelink.toml`)

- Parse at the edge: each property returns the model type (`IdPrefix`, `ProjectPath`, `Levels`).
- Trim and reject blanks: `scan.exclude`. Other values are refused as written, as today.
- Strict: unknown kind and unknown keys are errors. The only default is an empty `scan.exclude` and no `check.levels`.
- Paths: relative, no `..`.
- Failure: parsing happens before any write.
- Errors: all are configuration errors (exit 2), name the key, and escape names taken from the file so they cannot add lines.
- Arithmetic: none. An integer too large for `i64` is kept as "too large", and fails only where a property reads it.

## Quality rubric, after review

| Group | Score | Evidence, and what is partial |
| --- | --- | --- |
| Interface design | yes | `Workspace::open` is the one way in; use cases take it; typed properties |
| Boundaries | partly | unknown keys and kinds refused, `scan.exclude` trimmed; `ProjectPath` still accepts blank and control characters (report, finding 10) |
| Errors | partly | classes and exit codes unchanged; one unreachable internal case is classed as configuration (finding 7) |
| Readability | yes | domain literals named; clippy and its 40-line limit pass |
| Hygiene | yes | `arch` passes; unit and end-to-end tests; architecture documents updated; no glossary term changed; deferred findings name tickets |
