# Rulelink documentation

Rulelink keeps every business rule attached to its code, even when the team changes. This documentation is organised in three parts, to be read in this order.

## 1. Product — what and why

| File | Content |
|---|---|
| [`product/01-product-overview.md`](product/01-product-overview.md) | Need, target users, product principles, rule model, journeys, positioning, openness, success criteria, product decisions |
| [`product/02-features-and-releases.md`](product/02-features-and-releases.md) | Release frames (MVP, V1, V2), features by release, scope excluded from the MVP, exit criteria |

## 2. Architecture — how it works

| File | Content |
|---|---|
| [`architecture/00-global-architecture.md`](architecture/00-global-architecture.md) | Technology choice, context and container views, hexagonal architecture, component responsibilities and interactions, GoF patterns, cross-cutting conventions, architecture decisions |
| [`architecture/01-rule-registry.md`](architecture/01-rule-registry.md) | Rule model, lifecycle, identifier allocation, validation, search |
| [`architecture/02-sources-of-truth.md`](architecture/02-sources-of-truth.md) | Persistence, adapter contract, external adapters |
| [`architecture/03-code-linking.md`](architecture/03-code-linking.md) | `@rule` annotations, code scanning, `explain`, `where`, `annotate` |
| [`architecture/04-checks.md`](architecture/04-checks.md) | `check`, severities, reports, CI integration |
| [`architecture/05-import.md`](architecture/05-import.md) | File, Jira, CSV, GitHub and GitLab issue import |
| [`architecture/06-publishing-export.md`](architecture/06-publishing-export.md) | File export, Confluence publishing |
| [`architecture/07-agent-integration.md`](architecture/07-agent-integration.md) | Skills and agent context |
| [`architecture/08-ide-integration.md`](architecture/08-ide-integration.md) | LSP server, VS Code and IntelliJ extensions |
| [`architecture/09-project-configuration.md`](architecture/09-project-configuration.md) | `init`, configuration, wiring, command bus |
| [`architecture/10-runtime-lifecycle.md`](architecture/10-runtime-lifecycle.md) | Process bootstrap, signals, server states, resources |

## 3. Repository — how it is built and shipped

| File | Content |
|---|---|
| [`repository/01-repository-structure.md`](repository/01-repository-structure.md) | Monorepo layout, Cargo workspace orchestrated by moon, crate conventions, CI quality gates, mirror repositories |
| [`repository/02-release-lifecycle.md`](repository/02-release-lifecycle.md) | Branching, versioning matrix, release pipeline, channels, compatibility and deprecation, supply-chain security |

## Conventions

- Inside `architecture/`, a two-digit number (for example "see 04") refers to a file of that folder. Links to the other folders always name the folder.
- Every file states the release (MVP, V1, V2) of each feature it describes.
- Diagrams use Mermaid and are validated in CI (`docs-check` task, [repository/01](repository/01-repository-structure.md), section 9).
