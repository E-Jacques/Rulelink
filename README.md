# Checkmate

**Every business rule stays attached to its code, even when the team changes.**

Checkmate answers two questions:

1. **Which business rule does this code implement?**
2. **Which rules no longer have any code?**

> **Status: design phase.** The product and architecture are specified in [`docs/`](docs/README.md); no code has been written yet. The MVP is the only committed release.

## How it works

Rules live in a source of truth (a JSON file in the MVP):

```json
{
  "id": "RG-042",
  "title": "Line-level VAT rounding",
  "statement": "VAT is calculated and rounded to the cent on each invoice line, then summed.",
  "status": "validated",
  "application": "Billing",
  "module": "VAT calculation"
}
```

Code points to them with a comment, in any language:

```rust
// @rule RG-042 -- rounding applied before the sum
fn line_vat(line: &InvoiceLine) -> Money { ... }
```

A deterministic check runs in CI:

| Check | Case detected | Default level |
|---|---|---|
| Unknown reference | The code cites an identifier missing from the source | Error |
| Obsolete rule cited | The code still cites an abandoned rule | Error |
| Duplicate identifier | Two rules share the same identifier | Error |
| Rule without code | A validated rule is cited nowhere | Warning |
| Unvalidated rule cited | The code cites a rule that is still proposed | Warning |

AI only intervenes through skills run by the developer's own agent, always with human validation.

## Planned CLI (MVP)

| Command | Purpose |
|---|---|
| `checkmate init` | Create the configuration and install the skills |
| `checkmate check` | Run the checks (locally or in CI) |
| `checkmate explain <file>` | List the rules implemented in a file |
| `checkmate where <rule-id>` | List the code locations of a rule |

## Roadmap

| Release | Target | Theme |
|---|---|---|
| **MVP** | Developer working with AI agents | Rule format, CLI, JSON source, CI check, two skills |
| V1 | Product Owner, community | Jira → JSON → Confluence flow, IDE access, external adapters |
| V2 | Teams and large organisations | AI authoring skills, remote sources, IntelliJ |

Details: [Features and releases](docs/product/02-features-and-releases.md).

## Documentation

| Part | Start here |
|---|---|
| Product — what and why | [Product overview](docs/product/01-product-overview.md) |
| Architecture — how it works | [Global architecture](docs/architecture/00-global-architecture.md) |
| Repository — how it is built and shipped | [Repository structure](docs/repository/01-repository-structure.md) |

Full index: [`docs/README.md`](docs/README.md).

## Development

Planned stack: Rust (Cargo workspace) orchestrated by [moon](https://moonrepo.dev). Once the code exists, a fresh clone will build and test with:

```sh
moon setup
moon run :test      # or: cargo test --workspace
```

Working notes for AI agents are in [`AGENTS.md`](AGENTS.md). Specs and issues live as Markdown under `.scratch/<feature>/` (see [`docs/agents/issue-tracker.md`](docs/agents/issue-tracker.md)).

## License

[MIT](LICENSE) (see ADR-012 in the [global architecture](docs/architecture/00-global-architecture.md)).
