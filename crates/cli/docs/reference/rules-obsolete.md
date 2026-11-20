# rulelink rules obsolete

Retires a `validated` Rule that no longer applies. The Rule stays in the Source with the status `obsolete` and its Identifier is retired: it is never allocated again. You can record the Rule that replaces it. Run it from the project root. The command stops without changing anything if the Rule does not exist, is not `validated`, or if the replacement does not exist.

## Example

```bash
rulelink rules obsolete RULE-001 --superseded-by RULE-007
```

## Arguments

| name   | description                                  | example    |
| ------ | -------------------------------------------- | ---------- |
| `<id>` | Names the Identifier of the Rule to retire.  | `RULE-001` |

## Options

| name              | description                                                                              | example                    |
| ----------------- | ---------------------------------------------------------------------------------------- | -------------------------- |
| `--superseded-by` | Records the Identifier of the Rule that replaces this one. It must exist and differ from `<id>`. | `--superseded-by RULE-007` |
