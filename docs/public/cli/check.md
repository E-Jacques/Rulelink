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

## JSON report

Scripts and agents read the Findings as JSON:

```bash
rulelink check --format json
```

For the project of the first example, it prints:

```json
{
  "schema": "rulelink/check-report/v1",
  "findings": [
    {
      "code": "RL001",
      "name": "unknown-reference",
      "severity": "error",
      "message": "RULE-099 is not a Rule of this project",
      "rule": "RULE-099",
      "file": "src/billing.rs",
      "line": 42
    },
    {
      "code": "RL002",
      "name": "obsolete-reference",
      "severity": "warning",
      "message": "RULE-003 is obsolete, superseded by RULE-007",
      "rule": "RULE-003",
      "file": "src/vat.rs",
      "line": 3
    },
    {
      "code": "RL004",
      "name": "orphan-rule",
      "severity": "warning",
      "message": "RULE-005 is validated but no code cites it",
      "rule": "RULE-005",
      "file": "rules/rules.json"
    }
  ],
  "totals": {
    "errors": 1,
    "warnings": 2
  }
}
```

| field                  | value                                                                                  |
| ---------------------- | -------------------------------------------------------------------------------------- |
| `schema`               | The version of the report: `rulelink/check-report/v1`.                                 |
| `findings`             | The Findings, in the same order as the human report. Empty when there are none.         |
| `findings[].code`      | The code of the Finding, such as `RL001`.                                              |
| `findings[].name`      | Its name, as used to [change its level](../concepts/findings.md#change-a-level).       |
| `findings[].severity`  | `error` or `warning`. No other value is added within `v1`.                             |
| `findings[].message`   | One sentence saying what is wrong.                                                     |
| `findings[].rule`      | The Identifier the Finding is about. Optional: left out if there is none.              |
| `findings[].file`      | The file of the Annotation, or the rules file for a Rule that no code cites. Optional. |
| `findings[].line`      | The line of the Annotation, from 1. Optional: left out when no line is at fault.       |
| `totals.errors`        | How many Findings are errors.                                                          |
| `totals.warnings`      | How many Findings are warnings.                                                        |

A field that does not apply is left out, never `null`. Paths start at the project root and use `/` on every system. A Finding turned `off` is neither listed nor counted. Today every Finding has a `rule` and a `file`, but both are optional so that later Findings can do without.

The report is described by a [JSON Schema](https://github.com/E-Jacques/Rulelink/blob/main/schemas/check-report.schema.json). Later versions of Rulelink may add Finding codes and fields without changing `v1`: ignore the fields you do not know, and rely on `severity` and `message` for a code you do not know. Removing, renaming or changing the type of a field gives a new version.

Warnings about `rulelink.toml` are not Findings: they stay on the error output. When no report can be made (exit codes `2`, `3` and `10`), nothing is printed on the standard output, and the reason is on the error output.

## Options

| name       | description                                                              | example         |
| ---------- | ------------------------------------------------------------------------ | --------------- |
| `--format` | The report to print, `human` or `json`. Defaults to `human`, even piped. | `--format json` |

## Exit codes

Both formats give the same exit codes.

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
