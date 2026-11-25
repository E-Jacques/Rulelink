---
title: rulelink rules approve
since: 0.1.0
---

# rulelink rules approve

Marks a `proposed` Rule as `validated`, meaning a human has reviewed and accepted it. Run it from the project root. The command stops without changing anything if the Rule does not exist or is not `proposed`.

## Example

```bash
rulelink rules approve RULE-001
```

## Arguments

| name   | description                                  | example    |
| ------ | -------------------------------------------- | ---------- |
| `<id>` | Names the Identifier of the Rule to approve. | `RULE-001` |

## Exit codes

| code | meaning                                                                |
| ---- | ---------------------------------------------------------------------- |
| `0`  | The command succeeded.                                                 |
| `2`  | A value or the configuration is invalid. Nothing was changed.          |
| `3`  | The rules file is missing, unreadable or invalid. Nothing was changed. |
| `10` | Unexpected internal error.                                             |

## See also

- [`rulelink rules add`](rules-add.md)
- [`rulelink rules reject`](rules-reject.md)
