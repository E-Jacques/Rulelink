---
title: Configuration
description: Learn what you can set in rulelink.toml and what happens when a name is misspelled.
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
- `[scan]` is optional. `exclude` lists files the code scan skips, in `.gitignore` syntax. See [which files are scanned](specify-rules-in-code.md#which-files-are-scanned).
- `[check]` is optional. See [how to change a level](findings.md#change-a-level).

## Misspelled sections

A section that Rulelink does not know, such as `[checks.levels]` instead of `[check.levels]`, has no effect. The command still runs and keeps the same exit code, but prints a warning:

```text
rulelink: warning: rulelink.toml: unknown section or key `checks` is ignored; check its spelling
```

The warning goes to the error output, so it does not mix with the result of the command. It is not a Finding: it is not counted in the totals of `check`.

Names that start with `x-`, and the `[adapters]` section, give no warning. Use an `x-` name for your own notes or for other tools.

If you misspell a required section, such as `[sources]` instead of `[source]`, the command fails and names the missing section. It prints no warning when the configuration is invalid.

Warnings cover only the names at the top level of the file. Inside `[check]`, a name that Rulelink does not know is a configuration error. Inside `[source]`, `[rules]` and `[scan]`, it is ignored without a warning: `excludes` instead of `exclude` excludes nothing.
