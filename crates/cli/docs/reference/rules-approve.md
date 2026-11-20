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
