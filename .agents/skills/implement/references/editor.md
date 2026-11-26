# Editor

Code in `crates/lsp`, `editors/vscode`, `editors/intellij`. Design in `docs/architecture/08-ide-integration.md`.

- **Doc**: the editor's `docs/` folder. Add a guide in `docs/public/guides/` for a user-visible feature (`public-documentation`).
- **Role**: the server holds all logic and reuses the library crates. The extension forwards requests and renders answers.
- **Feedback loop**: an LSP request and its expected response. Start from the protocol, not the UI.
- **Extra gates**: build and lint of the touched extension.
- **Check**: a feature in an extension but not in the server is a bug.
- **e2e**: no editor harness exists yet. If the ticket needs one, say so in the plan.
- **Commit scope**: `lsp`, `vscode` or `intellij`.
