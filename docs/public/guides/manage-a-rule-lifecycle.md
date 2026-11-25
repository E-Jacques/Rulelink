---
title: Manage a Rule's lifecycle
description: Learn the four states of a Rule, who decides each change and which command applies it.
---

# Manage a Rule's lifecycle

A Rule moves through four states. A person decides each move, and a command records it. In this guide you take one Rule from `proposed` to `validated` to `obsolete`, and decline another.

## Before you start

- Run `rulelink init` at the root of your project. See [`rulelink init`](../cli/init.md).

## The four states

| State       | Meaning                                                     |
| ----------- | ----------------------------------------------------------- |
| `proposed`  | The Rule waits for a review. Every new Rule starts here.    |
| `validated` | A person approved the Rule. It is expected to have code.    |
| `obsolete`  | A validated Rule that no longer applies.                    |
| `rejected`  | A person declined a proposed Rule.                          |

Rulelink never changes a state on its own. A person runs the command.

## Create a proposed Rule

```bash
rulelink rules add --title "Discount is capped" --statement "A discount must not be above 30 percent."
```

```text
Created RULE-001 (proposed)
```

List the Rules that wait for a review with `rulelink rules list --status proposed`.

## Approve a Rule

Approve a `proposed` Rule once you agree with it.

```bash
rulelink rules approve RULE-001
```

```text
RULE-001 is now validated
```

## Reject a Rule

Reject a `proposed` Rule you do not want. It stays in the Source, so it is not proposed again.

```bash
rulelink rules add --title "Discount is rounded up" --statement "A discount is rounded up."
rulelink rules reject RULE-002
```

```text
Created RULE-002 (proposed)
RULE-002 is now rejected; its Identifier is retired
```

## Make a Rule obsolete

When a validated Rule no longer applies, retire it. You can name the Rule that replaces it.

```bash
rulelink rules add --title "Discount is capped at 20" --statement "A discount must not be above 20 percent."
rulelink rules approve RULE-003
rulelink rules obsolete RULE-001 --superseded-by RULE-003
```

```text
Created RULE-003 (proposed)
RULE-003 is now validated
RULE-001 is now obsolete; its Identifier is retired
```

Only a `validated` Rule can become obsolete. Only a `proposed` Rule can be approved or rejected. A command that does not fit the state of the Rule stops and changes nothing.

## Retired identifiers

The Identifier of a rejected or obsolete Rule is retired. It is never given to another Rule. The Rule stays in the Source, so the Identifier keeps one meaning for ever.

```bash
rulelink rules add --title "Discount is shown" --statement "The discount is shown on the invoice."
```

```text
Created RULE-004 (proposed)
```

`RULE-001` and `RULE-002` are retired, so the next Rule is `RULE-004`. Code that still cites a retired Identifier is reported by `rulelink where`:

```bash
rulelink where RULE-001
```

```text
RULE-001 is obsolete: its Identifier is retired
```

## Next steps

- [Rule format](../concepts/rule-format.md)
- [`rulelink rules approve`](../cli/rules-approve.md)
- [`rulelink rules reject`](../cli/rules-reject.md)
- [`rulelink rules obsolete`](../cli/rules-obsolete.md)
- [`rulelink rules list`](../cli/rules-list.md)
