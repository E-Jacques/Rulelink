---
title: Run Rulelink in CI
description: Learn how to make your CI fail when the rules file is not in Canonical form.
---

# Run Rulelink in CI

You can run `rulelink` in your CI workflow, like any other command. The build fails when the command exits with a code other than `0`. This guide shows a job that keeps the rules file in Canonical form, so independent edits do not collide.

## Before you start

- A project with a Source, created by [`rulelink init`](../cli/init.md).
- A CI job where the `rulelink` command is installed. Run the steps below after that step.

## Read the exit code

Every `rulelink` command ends with an exit code. `0` means it succeeded. `2` means a value or the configuration is invalid. `3` means the rules file is missing, unreadable or invalid. `10` is an unexpected error. A CI system stops the job on any code other than `0`, so a missing rules file already fails the build.

```bash
rulelink rules list
```

## Fail when the rules file is not in Canonical form

[`rulelink rules format`](../cli/rules-format.md) rewrites the rules file into Canonical form. It exits with `0` whether it changed the file or not, so compare the file before and after.

```bash
cp rules/rules.json "$RUNNER_TEMP/rules.before.json"
rulelink rules format
diff "$RUNNER_TEMP/rules.before.json" rules/rules.json
```

`diff` exits with `1` when the file changed. The job fails, and the log shows the lines that differ. Run `rulelink rules format` on your machine and commit the result to fix it.

## Put it in a GitHub Actions workflow

This job runs the steps above. Add the step that installs `rulelink` where marked by the comment line, using the method that suits your runner.

```yaml
name: rules
on: [pull_request]

jobs:
  rules-format:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      # Install the rulelink command here.
      - name: Rules file is in Canonical form
        run: |
          cp rules/rules.json "$RUNNER_TEMP/rules.before.json"
          rulelink rules format
          diff "$RUNNER_TEMP/rules.before.json" rules/rules.json
```

## Next steps

- [`rulelink rules format`](../cli/rules-format.md)
- [Rule format](../concepts/rule-format.md)
- [Find and relate Rules](find-and-relate-rules.md)
