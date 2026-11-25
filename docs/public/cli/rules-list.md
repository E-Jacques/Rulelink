---
title: rulelink rules list
since: 0.1.0
---

# rulelink rules list

Lists the Rules of the Source, one per line, with their Identifier, status and title, in Identifier order. Use `--status` to list only the Rules in one status, for example the `proposed` Rules that still need a review. Run it from the project root. Nothing is printed when no Rule matches.

## Example

```bash
rulelink rules list --status proposed
```

## Options

| name       | description                                                                                                       | example              |
| ---------- | ----------------------------------------------------------------------------------------------------------------- | -------------------- |
| `--status` | Lists only the Rules in this status: `proposed`, `validated`, `obsolete` or `rejected`. Defaults to every status. | `--status validated` |

## Exit codes

| code | meaning                                                                |
| ---- | ---------------------------------------------------------------------- |
| `0`  | The command succeeded.                                                 |
| `2`  | A value or the configuration is invalid. Nothing was changed.          |
| `3`  | The rules file is missing, unreadable or invalid. Nothing was changed. |
| `10` | Unexpected internal error.                                             |

## See also

- [`rulelink rules search`](rules-search.md)
- [`rulelink rules add`](rules-add.md)
