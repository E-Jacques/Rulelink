# rulelink where

Lists the places in your code that cite a Rule, one `file:line` per Annotation, tests included. Run it from the project root. What counts as an Annotation, and which files are scanned, is described in [Specify Rules in code](../../../../docs/public/specify-rules-in-code.md).

## Example

```bash
rulelink where RULE-042
```

Prints, sorted by file then line:

```text
src/pricing.rs:12
tests/pricing_test.rs:40
```

An Identifier needs a distinct answer when there is no code to list:

- A Rule that no Annotation cites prints `RULE-042 is not cited by any code`.
- An `obsolete` or `rejected` Rule prints `RULE-042 is obsolete: its Identifier is retired`, then the locations that still cite it, if any.
- An Identifier that is not in the Source stops the command with an error that names it.

## Arguments

| name   | description                                   | example    |
| ------ | --------------------------------------------- | ---------- |
| `<id>` | Names the Identifier of the Rule to look for. | `RULE-042` |
