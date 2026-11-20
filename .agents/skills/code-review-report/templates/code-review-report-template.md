---
issue: {{ issue_number }}
date: {{ YYYY-MM-DD }}
counter: {{ counter }}
branch: {{ branch }}
base: {{ merge-base sha }}
---

# Code review report: issue {{ issue_number }}

## Summary

| severity | count |
| -------- | ----- |
| critical | {{ n }} |
| high     | {{ n }} |
| minor    | {{ n }} |

## Findings

<!-- If there are none, replace the table with: No findings. -->

| # | severity | category | location | description | fixed | blocks |
| - | -------- | -------- | -------- | ----------- | ----- | ------ |
| 1 | {{ critical, high or minor }} | {{ category }} | `{{ path:line }}` | {{ one sentence }} | {{ yes or no }} | {{ ticket numbers or none }} |
<!-- endif -->

## Refactor considered

{{ what the refactor step changed, or `none, because …` }}
