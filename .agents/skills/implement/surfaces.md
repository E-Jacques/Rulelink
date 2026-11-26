# Surfaces

Pick every surface the ticket touches. The core (model, registry, link, check, import, config) comes first, then each surface that consumes it.

## CLI

Code in `crates/cli`. Use the `cli-implement` skill.

- **Doc**: `docs/public/cli/<command>.md`, via `cli-documentation`.
- **Smoke test**: the documented example, run against the binary.
- **Role**: parse arguments, call `app`, print. No domain logic.
- **Extra gates**: `cargo xtask docs-check` (every option documented), exit codes tested.

## Library

Reusable crates: `model`, `registry`, `link`, `check`, `import`, `config`, `adapter-sdk`.

- **Doc**: rustdoc on every public item, with an example. `docs/public/developers/crates.md` if the crate list changes.
- **Smoke test**: the doc example, as a doctest.
- **Role**: the public API is the contract. Keep it small and typed. Hide steps behind one deep operation.
- **Extra gates**: `moon run :api-docs` (warnings denied). Mark a breaking change in the commit footer.
- **Check**: a new public item has a caller. No caller, no export.

## Editor

Code in `crates/lsp`, `editors/vscode`, `editors/intellij`. See `docs/architecture/08-ide-integration.md`.

- **Doc**: the editor's `docs/` folder, plus a guide in `docs/public/guides/` when a user-visible feature appears (`public-documentation`).
- **Smoke test**: an LSP request and its expected response. Start from the protocol, not the UI.
- **Role**: the server holds all logic and reuses the library crates. The extension only forwards requests and renders answers.
- **Extra gates**: extension build and lint of the touched editor.
- **Check**: a feature that exists in an extension but not in the server is a bug.

## Several surfaces

Order: core, then library API, then CLI, then server, then extensions. One commit series per surface, one pull request if the slices ship together.
