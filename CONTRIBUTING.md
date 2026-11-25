# Contributing to Rulelink

Thank you for helping. This page explains how to write commits and open a pull request. Terms such as Rule, Source and Annotation are defined in the [glossary](GLOSSARY.md).

> Bug reports and feature requests are coming later. Their process is not defined yet, so please do not rely on one for now.

## Before you start

- Read the [repository structure](docs/repository/01-repository-structure.md) to see how the code is organised and which tasks exist.
- Read the [release lifecycle](docs/repository/02-release-lifecycle.md) to see how changes reach users.
- Work on a branch, never directly on `main`.

## Build and test

The project is a Rust workspace orchestrated by [moon](https://moonrepo.dev). Before you push, this command must pass. It is the command CI runs:

```sh
moon ci
```

For faster feedback while you work, use `cargo fmt --all`, `cargo test --workspace` and `cargo clippy --workspace --all-targets`.

## Commits

Follow [Conventional Commits](https://www.conventionalcommits.org). Every commit has a header, a body and a footer.

- **Header**: `type(scope): summary`. For `feat` and `fix`, write for the end user, because these feed the release notes. For every other type, write for the next developer.
- **Body**: what you did in this commit, and what was hard, including tooling problems and workarounds.
- **Footer**: technical trailers for this commit.

Example:

```text
feat(cli): add init command

Register the `rulelink init` command and its `--id-prefix` option.
Hand-rolled argument parsing was producing too much code, so the CLI now uses `clap`.
```

The full rules are in the [commit skill](.agents/skills/commit/SKILL.md).

## Pull requests

1. Keep the change small and focused on one topic.
2. Update the documentation in the same pull request as the code it describes.
3. Make sure `moon ci` passes.
4. Open the pull request with the GitHub command line tool, `gh`.
5. Write the body with these sections: Summary, Evidence and Merge Danger, then Blockers, Development notes and Code review.
6. The merge method is squash.

The full rules are in the [pull request skill](.agents/skills/create-pull-request/SKILL.md). Specs and issues are tracked as GitHub issues, as described in the [issue tracker notes](docs/agents/issue-tracker.md).

## Documentation

Public documentation for users lives in [docs/public](docs/public/concepts/rule-format.md). Contributor documentation lives in [docs](docs/README.md). Guidance for creating and editing public guides will be added here when it exists.
