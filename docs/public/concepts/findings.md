---
title: Findings
description: Learn what each Finding of rulelink check means and how to change its level.
---

# Findings

A Finding is one result of [`rulelink check`](../cli/check.md), about a Rule, an Annotation or the rules file. Each Finding has a code, a name and a level. An `error` makes `check` fail. A `warning` is reported and nothing more.

## Findings about your code

| Code    | Name                    | What it means                                                  | Default level |
| ------- | ----------------------- | -------------------------------------------------------------- | ------------- |
| `RL001` | `unknown-reference`     | The code cites an Identifier that no Rule has.                 | `error`       |
| `RL002` | `obsolete-reference`    | The code cites an `obsolete` Rule.                             | `warning`     |
| `RL004` | `orphan-rule`           | A `validated` Rule is cited by no code.                        | `warning`     |
| `RL005` | `unvalidated-reference` | The code cites a Rule that is still `proposed`.                | `warning`     |
| `RL009` | `retired-reference`     | The code cites a `rejected` Rule, or a Retired identifier.     | `error`       |

You get one Finding per line of code that cites the Identifier, and one `RL004` per Rule. When an `obsolete` Rule records its replacement, `RL002` names it. `RL002` is a `warning` so that you can move citations to the new Rule step by step. To make `check` fail on them, set `obsolete-reference = "error"`.

An Identifier listed as retired whose Rule is no longer in the Source gives `RL009`, not `RL001`.

## Findings about the rules file

`check` reads the rules file before it looks at your code. A merge or an edit by hand can leave it malformed or inconsistent:

| Code    | Name                 | What it means                                                           | Default level | How to fix it                                          |
| ------- | -------------------- | ----------------------------------------------------------------------- | ------------- | ------------------------------------------------------ |
| `RL003` | `duplicate-id`       | Two or more Rules share an Identifier.                                  | `error`       | Give the newer Rule an Identifier never used before.   |
| `RL100` | `invalid-shape`      | The file is valid JSON of the wrong shape.                              | `error`       | Fix the key or value the message names.                |
| `RL101` | `invalid-identifier` | An Identifier does not follow [the grammar](rule-format.md#identifier). | `error`       | Correct the Identifier.                                |
| `RL102` | `reused-retired-id`  | A `proposed` or `validated` Rule has an Identifier listed as retired.   | `error`       | Give the Rule an Identifier never used before.         |
| `RL103` | `unknown-related`    | `related` names a Rule that is not in the file.                         | `warning`     | Correct the entry or remove it.                        |
| `RL105` | `empty-field`        | A required field is empty or holds only spaces.                         | `error`       | Fill the field in.                                     |
| `RL106` | `not-canonical`      | The file is not in [Canonical form](rule-format.md#source-file).        | `warning`     | Run [`rulelink rules format`](../cli/rules-format.md). |

- **Wrong shape** means an unknown key, a missing field, a value of the wrong type or an unknown status. `check` reports the first such problem with its line, and no other Finding. Fix it and run `check` again to see the rest.
- **Required fields** are `id`, `title`, `statement` and, when a Rule has an `origin`, `origin.kind`.
- **Identifiers** are checked wherever the file holds one: `id`, `retired`, `related` and `superseded_by`. The grammar applies whatever [prefix you configured](configuration.md). A mistyped `related` entry gives both `RL101` and `RL103`.
- You get one `RL003` per duplicated Identifier, and one `RL103` per Rule and unknown Identifier.

`check` cannot tell that you deleted a Rule and wrote a new one by hand under the same Identifier, unless the Identifier is listed as retired.

### Errors in the rules file stop the code checks

While the rules file has an `RL003`, `RL100`, `RL101`, `RL102` or `RL105`, `check` reports only the Findings about the rules file: it cannot trust the Rules it would compare with your code. `RL103` and `RL106` do not stop the code checks, even if you raise them to `error`.

### A file that cannot be read

A rules file that is missing or cannot be read, that is not valid JSON (for instance while Git conflict markers are still in it) or that has an unsupported `format` is not a Finding. No report is printed and `check` exits with code `3`.

## While you have no Rules

Right after [`rulelink init`](../cli/init.md) the Source has no Rules. `RL001` is then a `warning`, so that Annotations written early do not fail your build. Once the Source has a Rule, `RL001` is an `error`. If you set its level yourself, your level applies in both cases.

## Change a level

Set levels in `rulelink.toml`, by name, in the `[check.levels]` section:

```toml
[check.levels]
orphan-rule = "error"
unvalidated-reference = "off"
```

A level is `error`, `warning` or `off`. `off` hides the Finding. `duplicate-id`, `invalid-shape`, `invalid-identifier`, `reused-retired-id` and `empty-field` are always errors: setting one to `warning` or `off` is a configuration error. You can raise `unknown-related` and `not-canonical` to `error`, or turn them `off`. A name or a level that is not listed here is a configuration error. So is an [unknown section or key](configuration.md#unknown-sections-and-keys), such as another key under `[check]` or a misspelled `[check]` itself. There is no other way to set a level: not per folder, and not on the command line.

To keep some files out of the check, for example documentation that shows Annotations, see [which files are scanned](specify-rules-in-code.md#which-files-are-scanned).
