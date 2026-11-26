---
name: implement
description: Workflow for implementing a ticket in rulelink, from documentation to pull request, whatever the surface (CLI, library, editor). Use when adding or changing behaviour that ships to users or to other crates.
---

# Implementing a ticket

One loop for every surface. Surface-specific rules live in [surfaces.md](surfaces.md). `cli-implement` adds the CLI details. Each commit follows the `commit` skill.

## Principles

- **Document first.** The public contract is written before the code.
- **Feedback loop first.** No code before a command that fails for the right reason.
- **Thin vertical slices.** One narrow path through every layer, then widen.
- **Logic in the core.** Domain rules live in the model. CLI, LSP and extensions translate.
- **Fresh eyes review.** The reviewer never saw the implementation.
- **Escalate, don't guess.** An unmet criterion goes to the user.

## 1. Pick the surface

Name every surface the ticket touches: CLI, library, editor. Read its section in [surfaces.md](surfaces.md). A ticket often touches several: the core change comes first, each surface follows.

## 2. Open a worktree

Never work in the main checkout.

1. `EnterWorktree`, named after the issue (`12-rules-format`).
2. Rename the branch: `git branch -m <feature|fix|support>/<issue-name>`.
3. If the branch points to a missing directory, `git worktree prune`, then retry.
4. Leave with `ExitWorktree` (`keep`) after the pull request is open.
5. Inside it: Write or Edit for files, one plain command per Bash call, no `cd`, no heredoc.

## 3. Document first

Write or update the public page for each surface (see [surfaces.md](surfaces.md)). It fixes the scope.

Commit: `docs(<scope>): …`

## 4. Feedback loop

Write before any implementation, only where applicable:

- A test that runs the exact documented example.
- A test with all defaults, if it differs from the first.

Pick the seams where later tests go. Confirm them with the user if unsure.

## 5. Plan

Write `.scratch/<feature>/plan-<NN>.md`: the deliverables of `strategic-programming`, plus the `input-boundaries` checklist on every input.

Read `GLOSSARY.md` and the relevant ADRs. Read `codebase-design` before adding a function others will call.

Commit: `docs(plan): …`

## 6. Implement

Use `tdd`: red, then green, one slice at a time, at the planned seams.

Gate before every push: `moon ci` passes. A warning it reports is yours to fix. While iterating, run the narrowest test first.

Commit: `feat(<scope>): …`

## 7. Refactor

Ask what the change made awkward. Check the code against the plan with `strategic-programming`. Do the cheap fixes.

Commit: `refactor(<scope>): …`

Changed nothing? The report says `Refactor considered: none, because …`.

## 8. Review

1. **Correctness**: the `code-review` skill against the merge-base with `main`. A self-review does not count. Check the `Errors` section of `input-boundaries` and that the code matches the page from step 3.
2. **Design**: a fresh subagent. Give it only the diff, the plan, `strategic-programming`, `input-boundaries` and the dependent tickets. Ask for smells and unchecked boundaries.

Then:

- Update the `GLOSSARY.md` terms and `docs/architecture/` sections the diff contradicts.
- Score against `docs/agents/quality-rubric.md`.
- Run the extra gates of the surface.
- Record both passes with `code-review-report`.

Commit: `refactor(<scope>): …`, or `fix(<scope>): …` for a bug.

## 9. Acceptance criteria

Check each criterion against the running code. If one is unmet, stop and tell the user.

## 10. Pull request

Follow `create-pull-request`. Hand it the report and what went wrong. The title and type match the diff: a `docs(…)` PR has no implementation.
