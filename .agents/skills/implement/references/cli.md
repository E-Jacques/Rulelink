# CLI

Code in `crates/cli`.

- **Doc**: `docs/public/cli/<command>.md`, via `cli-documentation`.
- **Role**: parse arguments, call `app`, print. Rules and parsing another command will need go in the core.
- **Plan**: run the `input-boundaries` checklist on every argument and option.
- **Feedback loop**: an e2e test per documented example and one with all defaults ([e2e.md](e2e.md)).
- **Extra gates**: `cargo xtask docs-check` (every `--help` option is on the page), `cargo xtask arch`, `cargo deny check`. Exit codes are tested.
- **Commit scope**: `cli`.
