---
title: Findings
description: Learn what each Finding of rulelink check means and how to change its level.
---

# Findings

A Finding is one result of [`rulelink check`](../cli/check.md), about a Rule or an Annotation. Each Finding has a code, a name and a level. An `error` makes `check` fail. A `warning` is reported and nothing more.

## The Findings

| Code    | Name                    | What it means                                                  | Default level |
| ------- | ----------------------- | -------------------------------------------------------------- | ------------- |
| `RL001` | `unknown-reference`     | The code cites an Identifier that no Rule has.                 | `error`       |
| `RL002` | `obsolete-reference`    | The code cites an `obsolete` Rule.                             | `warning`     |
| `RL004` | `orphan-rule`           | A `validated` Rule is cited by no code.                        | `warning`     |
| `RL005` | `unvalidated-reference` | The code cites a Rule that is still `proposed`.                | `warning`     |
| `RL009` | `retired-reference`     | The code cites a `rejected` Rule, or a Retired identifier.     | `error`       |

You get one Finding per line of code that cites the Identifier, and one `RL004` per Rule. When an `obsolete` Rule records its replacement, `RL002` names it. `RL002` is a `warning` so that you can move citations to the new Rule step by step. To make `check` fail on them, set `obsolete-reference = "error"`.

An Identifier listed as retired whose Rule is no longer in the Source gives `RL009`, not `RL001`.

## While you have no Rules

Right after [`rulelink init`](../cli/init.md) the Source has no Rules. `RL001` is then a `warning`, so that Annotations written early do not fail your build. Once the Source has a Rule, `RL001` is an `error`. If you set its level yourself, your level applies in both cases.

## Change a level

Set levels in `rulelink.toml`, by name, in the `[check.levels]` section:

```toml
[check.levels]
orphan-rule = "error"
unvalidated-reference = "off"
```

A level is `error`, `warning` or `off`. `off` hides the Finding. A name or a level that is not listed here is a configuration error, and so is any other key under `[check]`. A misspelled `[check]` itself only gives a [warning](configuration.md#misspelled-sections). There is no other way to set a level: not per folder, and not on the command line.

To keep some files out of the check, for example documentation that shows Annotations, see [which files are scanned](specify-rules-in-code.md#which-files-are-scanned).
