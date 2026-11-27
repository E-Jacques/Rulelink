---
title: rulelink rules format
description: Rewrite the rules file into Canonical form, for example after a Git merge.
since: 0.1.0
---

# rulelink rules format

Rewrites the rules file into Canonical form: one property or array element per line, Rules in natural Identifier order (`RULE-2` before `RULE-10`) and keys in a fixed order. Run it from the project root after resolving a Git merge conflict in the rules file, or after editing it by hand. It leaves a file that is already in Canonical form untouched, and stops without changing anything if the file is missing or is not valid JSON of the expected shape, for instance while Git conflict markers are still in it: resolve them first, then run the command. It only changes layout: it does not check the Rules themselves, which [`rulelink check`](check.md) does. `check` also tells you when the file is not in Canonical form (`RL106`).

## Example

```bash
rulelink rules format
```

## Exit codes

| code | meaning                                                                |
| ---- | ---------------------------------------------------------------------- |
| `0`  | The command succeeded.                                                 |
| `2`  | A value or the configuration is invalid. Nothing was changed.          |
| `3`  | The rules file is missing, unreadable or invalid. Nothing was changed. |
| `10` | Unexpected internal error.                                             |

## See also

- [`rulelink rules list`](rules-list.md)
- [`rulelink init`](init.md)
- [Rule format](../concepts/rule-format.md)
