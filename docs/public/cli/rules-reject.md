---
title: rulelink rules reject
since: 0.1.0
---

# rulelink rules reject

Declines a `proposed` Rule. The Rule stays in the Source with the status `rejected`, so it is not proposed again, and its Identifier is retired: it is never allocated again. Run it from the project root. The command stops without changing anything if the Rule does not exist or is not `proposed`.

## Example

```bash
rulelink rules reject RULE-002
```

## Arguments

| name   | description                                 | example    |
| ------ | ------------------------------------------- | ---------- |
| `<id>` | Names the Identifier of the Rule to reject. | `RULE-002` |

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
