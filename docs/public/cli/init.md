---
title: rulelink init
since: 0.1.0
---

# rulelink init

Sets up Rulelink in the current directory by creating `rulelink.toml` and an empty rules file at `rules/rules.json`. Run it once, at the root of your project, before you add any Rule. It stops without changing anything if either file already exists.

## Example

```bash
rulelink init
```

## Options

| name          | description                                                                                                        | example          |
| ------------- | ------------------------------------------------------------------------------------------------------------------ | ---------------- |
| `--id-prefix` | Sets the prefix of Rule identifiers (capital letters, digits and `_`, starting with a letter). Defaults to `RULE`. | `--id-prefix BR` |

## Exit codes

| code | meaning                                                                |
| ---- | ---------------------------------------------------------------------- |
| `0`  | The command succeeded.                                                 |
| `2`  | A value or the configuration is invalid. Nothing was changed.          |
| `3`  | The rules file is missing, unreadable or invalid. Nothing was changed. |
| `10` | Unexpected internal error.                                             |

## See also

- [`rulelink rules add`](rules-add.md)
- [`rulelink rules list`](rules-list.md)
