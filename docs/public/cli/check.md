---
title: rulelink check
description: Report the Findings about the links between your Rules and your code.
since: 0.1.0
---

# rulelink check

Compares your Rules with the Annotations of the whole project and reports Findings. Run it from the project root, on your machine or in CI. It verifies the link between a Rule and the code, not that the code is correct. What each Finding means, and how to change its level, is described in [Findings](../concepts/findings.md).

## Example

```bash
rulelink check
```

Prints one line per Finding with its level, code, message and location, then the totals:

```text
error RL001: RULE-099 is not a Rule of this project (src/billing.rs:42)
warning RL002: RULE-003 is obsolete, superseded by RULE-007 (src/vat.rs:3)
warning RL004: RULE-005 is validated but no code cites it (rules/rules.json)
1 error, 2 warnings
check verifies the link between a Rule and the code, not that the code is correct.
```

Errors come first, then Findings are ordered by code, Identifier, file and line. The location is the `file:line` of the Annotation. A Finding about a Rule that no code cites names the rules file.

While the rules file has no Rules, the first line says so and the code is still scanned. An Annotation citing an unknown Identifier (`RL001`) is then a warning by default:

```text
The rules file has no Rules yet; the code is still scanned.
0 errors, 0 warnings
check verifies the link between a Rule and the code, not that the code is correct.
```

## Exit codes

| code | meaning                                                  |
| ---- | -------------------------------------------------------- |
| `0`  | No error Finding. There may be warnings.                 |
| `1`  | At least one error Finding.                              |
| `2`  | A value or the configuration is invalid. Nothing ran.    |
| `3`  | The rules file is missing, unreadable or invalid.        |
| `10` | Unexpected internal error.                               |

## See also

- [Findings](../concepts/findings.md)
- [Configuration](../concepts/configuration.md)
- [Specify Rules in code](../concepts/specify-rules-in-code.md)
- [`rulelink where`](where.md)
- [Run Rulelink in CI](../guides/run-rulelink-in-ci.md)
