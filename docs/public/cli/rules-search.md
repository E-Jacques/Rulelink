---
title: rulelink rules search
description: Find the Rules that contain a keyword, ignoring case and accents.
since: 0.1.0
---

# rulelink rules search

Finds the Rules that contain a keyword, and prints them like `rulelink rules list`. The search ignores case and accents, so `enonce` finds "Énoncé", and works with values written in any language. It looks in the Identifier, title, statement, module, applications and covered and excluded cases. Run it from the project root. Nothing is printed when no Rule matches.

## Example

```bash
rulelink rules search "tva"
```

## Arguments

| name     | description                                                                               | example |
| -------- | ----------------------------------------------------------------------------------------- | ------- |
| `<text>` | Sets the text to look for. It must not be empty or blank. Surrounding spaces are ignored. | `"tva"` |

## Options

| name       | description                                                                                                       | example              |
| ---------- | ----------------------------------------------------------------------------------------------------------------- | -------------------- |
| `--status` | Keeps only the Rules in this status: `proposed`, `validated`, `obsolete` or `rejected`. Defaults to every status. | `--status validated` |

## Exit codes

| code | meaning                                                                |
| ---- | ---------------------------------------------------------------------- |
| `0`  | The command succeeded.                                                 |
| `2`  | A value or the configuration is invalid. Nothing was changed.          |
| `3`  | The rules file is missing, unreadable or invalid. Nothing was changed. |
| `10` | Unexpected internal error.                                             |

## See also

- [`rulelink rules list`](rules-list.md)
- [`rulelink where`](where.md)
