---
name: cli-implement
description: CLI details of the implement workflow for a new command or subcommand in rulelink. Use when adding or changing a command in `crates/cli`.
---

# Implementing a CLI command

Follow the `implement` skill. These are the CLI specifics for each of its steps.

## 3. Document first

Use the `cli-documentation` skill. The page is `docs/public/cli/<command>.md`.

Commit: `docs(cli): …`

## 4. Feedback loop

- One test runs the exact example of the page.
- One test runs the command with all defaults. Skip it if it duplicates the first.

## 5. Plan

Run the `input-boundaries` checklist on every argument and option.

## 6. Implement

The CLI parses, calls `app` and prints. Rules and parsing that another command will need go in the core.

Commit: `feat(cli): …`

## 8. Review

Extra gates: `cargo xtask arch`, `cargo deny check` and `cargo xtask docs-check`. Check that every option of `--help` is on the page.

Commit: `refactor(cli): …` or `fix(cli): …`
