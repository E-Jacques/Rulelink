---
name: cli-implement
description: Workflow for implementing a new CLI command or subcommand in checkmate, from end-user docs to pull request. Use when adding or changing a command in `crates/cli`.
---

# Implementing a CLI command

This is the CLI-specific wrapper around the generic implement loop. Follow the steps in order. Each commit follows the `commit` skill.

## 1. Create the branch

Create the git branch name after what should be done using the feature, fix or support prefix. The branch should contain the issue name.

## 2. Document first

Write or update the end-user page with the `cli-documentation` skill. It fixes the scope of the change.

Commit: `docs(cli): …`

## 3. Smoke tests

Write them before any implementation, and only where applicable:

- One test that runs the exact example from the documentation page.
- One test that runs the command with all defaults. Skip it if it duplicates the first.

## 4. Plan

Write `.scratch/<feature>/plan-<NN>.md` before coding. It follows the deliverables of the `strategic-programming` skill (interface, misuse, ownership, smells, reuse, dependent tickets). Run the `input-boundaries` checklist on every input of the command.

Read `GLOSSARY.md` and the ADRs in `docs/adr/`, and read the `codebase-design` skill whenever you add a function that other code will call, not only for a new module. Commit the plan note with the documentation commit or on its own (`docs(plan): …`).

## 5. Implement

Use the `tdd` skill (red → green) at the seams you planned. Run the smoke tests from step 3 until they pass.

Commit: `feat(cli): …`

## 6. Refactor

Ask what the change made awkward and what it would cost to fix. Apply the `strategic-programming` skill against the plan note: does the code still match it, and which smells appeared during implementation?

Do the cheap, worthwhile fixes. Commit: `refactor(cli): …`

If you change nothing, the review report must say `Refactor considered: none, because …`. Skipping this step silently is not allowed.

## 7. Review

Review in two passes, then record the findings.

1. **Correctness pass**: run the `code-review` skill against the merge-base with `main`. Also check Rust idioms (ownership, needless clones, clippy) and the `Errors` section of `input-boundaries` (one type per cause, no catch-all arm, no blanket `From`, each message names the value or path at fault) and that the code still matches the documentation page from step 2.
2. **Design pass**: start a fresh subagent with no history of the implementation. Give it only the diff, the plan note, the `strategic-programming` skill, the `input-boundaries` checklist and the list of dependent tickets. Ask it to report smells and unchecked boundary lines. A fresh context is not anchored on the code you just wrote.

Run `cargo xtask arch` and `cargo deny check` as well; they catch what no reviewer should have to.

Fix what is worth fixing. Record the findings of both passes with the `code-review-report` skill.

Commit: `refactor(cli): …` (or `fix(cli): …` if a bug was found)

## 8. Check acceptance criteria out

Control that acceptance criteria have been fulfilled and check them out if yes. If at least one is not, escaladate to user.

## 9. Pull request

Follow the `create-pull-request` skill. Hand it the review report and anything that went wrong along the way. The title and type must match the diff: a `docs(...)` PR contains no implementation.
