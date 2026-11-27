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

Report: <path to the `code-review-report` file>

| severity | count |
| -------- | ----- |
| critical | 0     |
| high     | 0     |
| minor    | 0     |
````

Copy the counts from the report's summary.

## Checks

After creating the pull request, and after every later push, run `gh pr checks <number> --watch`. For a failed check, read its log with `gh api repos/<owner>/<repo>/actions/jobs/<job-id>/logs` (the job id ends the check's URL). A check can fail on a job that `moon ci` does not run.

The step is done when every check has finished, and each failure is either fixed or listed under Blockers. Report that result to the user, and keep Blockers true to what the checks ran.

## After a review

Comments on an open pull request start another round:

1. **Answer first.** Check each comment against the code. Put every choice it opens to the user, and change code once they have decided.
2. **Replies speak for the user.** Draft them, and post the text the user approved.
3. **Review again.** A new commit that changes behaviour is unreviewed, including a fix for a review finding. Run both passes of step 8 of the `implement` skill on the commits made since the last reviewed one, and record them in a new `code-review-report`.
4. **Bring the pull request up to date.** Make the body and its review counts match the diff, then follow Checks.

The round is done when the latest commit that changes behaviour has been reviewed. Until then, tell the user which commits have not been, wherever you report the pull request's state.

## Merging

The user merges. Leave the pull request open, and tell the user which merge method to pick: squash, unless they asked for another one. Merge or enable auto-merge only when the user asks for it for this pull request.

If there is any blocker, keep the pull request in draft, with a "not ready to merge" line at the top of the body.
