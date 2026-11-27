---
title: Run Rulelink in CI
description: Learn how to make your CI fail on error Findings or a badly laid out rules file.
---

# Run Rulelink in CI

You can run `rulelink` in your CI workflow, like any other command. The build fails when the command exits with a code other than `0`. This guide shows a job that checks your Rules and their links to your code, and keeps the rules file in Canonical form so independent edits do not collide.

## Before you start

- A project with a Source, created by [`rulelink init`](../cli/init.md).
- A CI job where the `rulelink` command is installed. Run the steps below after that step.

## Read the exit code

Every `rulelink` command ends with an exit code. `0` means it succeeded. `2` means a value or the configuration is invalid. `3` means the rules file is missing, unreadable or invalid. For `check`, a rules file of the wrong shape is a Finding instead, with exit code `1`. `10` is an unexpected error. A CI system stops the job on any code other than `0`, so a missing rules file already fails the build.

```bash
rulelink rules list
```

## Check your Rules and your code

[`rulelink check`](../cli/check.md) checks the rules file, then scans the whole project. It exits with `1` when there is at least one error Finding, so the job fails. Warnings do not fail it.

```bash
rulelink check
```

To fail the build on a warning too, raise its level in `rulelink.toml`. See [Findings](../concepts/findings.md).

## Fail when the rules file is not in Canonical form

When the rules file stays in [Canonical form](../concepts/rule-format.md#source-file), two people can edit different Rules without a Git conflict. `check` reports a file that is not in it as a warning, `RL106`. To fail the job, raise it to `error` in `rulelink.toml`:

```toml
[check.levels]
not-canonical = "error"
```

When the job fails on `RL106`, run [`rulelink rules format`](../cli/rules-format.md) on your machine and commit the result.

Canonical form uses LF line endings. If Git converts them to CRLF when it checks out the project, as it often does on Windows, add this line to `.gitattributes`, with the path of your rules file:

```text
rules/rules.json text eol=lf
```

## Put it in a GitHub Actions workflow

This job runs `rulelink check`. Add the step that installs `rulelink` where marked by the comment line, using the method that suits your runner.

```yaml
name: rules
on: [pull_request]

jobs:
  rules:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      # Install the rulelink command here.
      - name: Check the Rules and the code
        run: rulelink check
```

## Next steps

- [`rulelink check`](../cli/check.md)
- [`rulelink rules format`](../cli/rules-format.md)
- [Rule format](../concepts/rule-format.md)
- [Find and relate Rules](find-and-relate-rules.md)
