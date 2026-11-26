---
name: code-review-report
description: Writes a templated code-review report so recurring issue types can be spotted over time. Use after running the `code-review` skill, before opening a pull request.
---

# Code review report

Turn the findings of a `code-review` run into one Markdown file. The report only records findings. It does not track whether they get fixed.

Start from [templates/code-review-report-template.md](templates/code-review-report-template.md). Replace every `{{ placeholder }}` and delete the HTML comments.

## Where the report goes

`.scratch/<feature>/code-review-reports/report-{{issue_number}}-{{YYYY-MM-DD}}-{{counter}}.md`

- `<feature>`: the feature directory of the issue being implemented (see `docs/agents/issue-tracker.md`). If there is no local feature directory, use the feature slug from the branch name, or ask.
- `issue_number`: the GitHub issue number (`42`).
- `YYYY-MM-DD`: the review date.
- `counter`: two digits, starting at `01`. Take the highest existing counter for the same issue and date, and add one.

Create the `code-review-reports/` directory if it does not exist.

## Content

- Review the diff from the merge-base with `main` to `HEAD`. Do not report problems in untouched code.
- One row per finding. Each finding has exactly one severity and one category.
- Quote the location as `path:line`. Describe the problem in one sentence, and the fix in another if it is not obvious.
- A clean review is a valid report: keep the summary at zero and write `No findings.` in the findings section.

## Deferred findings

A finding that is not fixed carries a `blocks` value: the dependent tickets that will hit the same problem (found with `gh api repos/{owner}/{repo}/issues/<n>/dependencies/blocking --jq '.[].number'`), or `none`. "Out of scope" is only a valid reason when `blocks` is `none`.

## Refactor considered

End the report with one line: what the refactor step changed, or `none, because …`.

## Severity

- `critical`: wrong behaviour, data loss, security, or a crash on a normal path.
- `high`: likely to cause a bug, impact performance, or breaks a documented rule or the spec.
- `minor`: maintainability, style, naming; no behaviour risk.

## Categories

Use one of these. When a finding fits several, pick the category of its root cause, not of its symptom (e.g. a crash caused by a swallowed error is `error-handling`, not `bug`). If none fits, use `other` and say why in the description. If `other` keeps appearing for the same reason, propose a new category to the user.

| category            | covers                                                                      |
| ------------------- | --------------------------------------------------------------------------- |
| `bug`               | logic or behaviour errors                                                   |
| `spec-gap`          | behaviour missing, partial or extra compared with the issue or spec         |
| `rust-idiom`        | ownership, needless clones, iterator misuse, clippy-level problems          |
| `error-handling`    | swallowed errors, panics on user input, unhelpful messages                  |
| `design`            | duplication, speculative code, anything design-related not listed below     |
| `temporal-coupling` | calls that must happen in a given order, or state changed before validation |
| `layering`          | domain logic in `app` or `cli`, a layer that knows too much                 |
| `boundary`          | an unchecked line of the `input-boundaries` checklist                       |
| `atomicity`         | a failure leaves a file or state half-changed; concurrent writers           |
| `naming`            | names that do not match `GLOSSARY.md` or do not reveal intent               |
| `tests`             | missing, brittle, or implementation-coupled tests                           |
| `docs`              | CLI documentation page or other docs out of sync with the code              |
| `other`             | anything else                                                               |
