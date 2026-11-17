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

Decide before coding: how data flows, where ownership is needed, and which traits and data types are required. Read `GLOSSARY.md` and the ADRs in `docs/adr/`, and use the `codebase-design` skill if a new module boundary is involved.

## 5. Implement

Use the `tdd` skill (red → green) at the seams you planned. Run the smoke tests from step 3 until they pass.

Commit: `feat(cli): …`

## 6. Refactor

Ask what the change made awkward and what it would cost to fix. Do the cheap, worthwhile ones.

Commit: `refactor(cli): …`

## 7. Review

Run the `code-review` skill against the branch's merge-base with `main`. Also check Rust idioms (error handling, ownership, needless clones, clippy) and that the code still matches the documentation page from step 2. Fix what is worth fixing.

Record the findings with the `code-review-report` skill.

Commit: `refactor(cli): …` (or `fix(cli): …` if a bug was found)

## 8. Check acceptance criteria out

Control that acceptance criteria have been fulfilled and check them out if yes. If at least one is not, escaladate to user.

## 9. Pull request

Follow the `create-pull-request` skill. Hand it the review report and anything that went wrong along the way.
