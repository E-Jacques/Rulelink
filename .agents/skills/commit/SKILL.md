---
name: commit
description: How to write commits in this project (conventional commits with header, body and footer). Use whenever you commit.
---

# Commits

Follow Conventional Commits. Every commit has a header, a body and a footer.

## Header

`type(scope): summary`, saying what changes for the reader of the changelog.

- `feat` and `fix`: write for the **end user** (they feed the changelog).
- Every other type: write for the **next developer**.

## Body

What was done in this commit, and what was hard. Include tooling problems you hit (a flaky tool, a misleading error, a workaround).

## Footer

Technical trailers for this commit. Replace `<model>` with the model you are running as.

## Example

```text
feat(cli): add init command

Register the `checkmate init` command and its `--id-prefix` option.
Hand-rolled argument parsing was producing too much code, so the CLI now uses `clap`.

Assisted-by: <model>
```
