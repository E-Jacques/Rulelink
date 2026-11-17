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

| # | severity | category | location | description |
| - | -------- | -------- | -------- | ----------- |
| 1 | {{ critical, high or minor }} | {{ category }} | `{{ path:line }}` | {{ one sentence }} |
<!-- endif -->
