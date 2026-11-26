---
name: create-pull-request
description: How to open a pull request in this project with `gh`, including the required project-specific sections. Use when work on a branch is ready for review.
---

# Creating a pull request

Use the `gh` CLI. If it is not available, stop and tell the user.

## Body

Write the body with the `pr` skill (Summary, Evidence, Merge Danger). Then append these project sections:

````markdown
## Blockers

<none, or what blocks the merge, e.g. e2e tests still failing after 3 attempts>

## Development notes

- What went wrong during development
- Problems with tooling

## Code review

```markdown
## Code review

Report: <path to the `code-review-report` file>

| severity | count |
| -------- | ----- |
| critical | 0     |
| high     | 0     |
| minor    | 0     |
````

Copy the counts from the report's summary.

## Merge settings

- Squash is the merge method. Enable it on the created PR (`gh pr merge --auto --squash` or the repository setting).
- If there is any blocker, mark the PR as not ready to merge (draft, plus a "not ready to merge" line at the top of the body).
