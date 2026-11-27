---
title: Configuration
description: Learn what you can set in rulelink.toml and what happens when something in it is wrong.
---

# Configuration

[`rulelink init`](../cli/init.md) creates `rulelink.toml` at the root of your project. Every other command reads it.

## What you can set

```toml
version = 1

[source]
kind = "json"
path = "rules/rules.json"

[rules]
prefix = "RULE"

[scan]
exclude = ["vendor/"]

[check.levels]
orphan-rule = "error"
```

- `version`, `[source]` and `[rules]` are required.
- The keys of `[source]` other than `kind` depend on the kind. With `kind = "json"`, `path` is required: a relative path inside the project, without `..`.
- `[scan]` is optional. `exclude` lists files the code scan skips, in `.gitignore` syntax. See [which files are scanned](specify-rules-in-code.md#which-files-are-scanned).
- `[check]` is optional. See [how to change a level](findings.md#change-a-level).

## Mistakes in rulelink.toml

Every command that reads `rulelink.toml` checks all of it first, even the settings it does not use. If something is wrong, the command changes nothing, prints the problem on the error output and exits with code 2.

## Unknown sections and keys

A section or key that Rulelink does not know is an error. If you write `excludes` instead of `exclude`, the message names the key with its section, and suggests the closest known one:

```text
rulelink: configuration error: invalid rulelink.toml: unknown section or key `scan.excludes`; did you mean `scan.exclude`?
```

A top-level name that itself contains a dot is shown in double quotes, so that you can tell it from a key of a section.

Names that start with `x-`, and the `[adapters]` section, are accepted. Use an `x-` name for your own notes or for other tools.

## Several problems at once

The message lists every problem found. For example, if you write `prefx` instead of `prefix` under `[rules]`:

```text
rulelink: configuration error: invalid rulelink.toml:
  - missing `rules.prefix`
  - unknown section or key `rules.prefx`; did you mean `rules.prefix`?
```

Two mistakes hide the others until you fix them: a file that is not valid TOML, and a missing or unsupported `version`. If `source.kind` is wrong, the other keys of `[source]` are not checked.
