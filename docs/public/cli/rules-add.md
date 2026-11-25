---
title: rulelink rules add
description: Create a new Rule with the status proposed and print its Identifier.
since: 0.1.0
---

# rulelink rules add

Creates a new Rule in the Source with the status `proposed` and prints its Identifier. The Identifier is allocated for you (`RULE-001`, `RULE-002`, …) and is never reused, even if a Rule was retired or deleted by hand. Run it from the project root, after `rulelink init`.

## Example

```bash
rulelink rules add --title "Line-level VAT rounding" --statement "VAT is rounded to the cent on each invoice line, then summed."
```

## Options

| name          | description                                                                                     | example                                  |
| ------------- | ----------------------------------------------------------------------------------------------- | ---------------------------------------- |
| `--title`     | Sets the short title of the Rule (1 to 120 characters). Required.                               | `--title "Line-level VAT rounding"`      |
| `--statement` | Sets the statement of the Rule. Required.                                                       | `--statement "VAT is rounded per line."` |
| `--app`       | Names an application the Rule applies to. Repeat it for several. Defaults to the whole project. | `--app billing`                          |
| `--module`    | Sets the functional module of the Rule.                                                         | `--module "VAT calculation"`             |
| `--covered`   | Adds a case the Rule covers. Repeat it for several.                                             | `--covered "B2B invoices"`               |
| `--excluded`  | Adds a case the Rule explicitly excludes. Repeat it for several.                                | `--excluded "Non-EU export invoices"`    |
| `--related`   | Records the Identifier of a related Rule. It must exist. Repeat it for several.                 | `--related RULE-001`                     |

Values can be written in any language. The command stops without changing anything if a value is empty, repeated, or the title is too long, or if a `--related` Identifier does not exist.

## Exit codes

| code | meaning                                                                |
| ---- | ---------------------------------------------------------------------- |
| `0`  | The command succeeded.                                                 |
| `2`  | A value or the configuration is invalid. Nothing was changed.          |
| `3`  | The rules file is missing, unreadable or invalid. Nothing was changed. |
| `10` | Unexpected internal error.                                             |

## See also

- [`rulelink rules approve`](rules-approve.md)
- [`rulelink rules list`](rules-list.md)
