---
name: cli-documentation
description: Explains how a CLI command should be documentated for the endusers to consume and use it.
---

# CLI documentation

Write one Markdown page per CLI command, aimed at end users who want to run the command, not at contributors.

Start from [templates/cli-documentation-template.md](templates/cli-documentation-template.md). Copy it, replace every `{{ placeholder }}`, and delete the HTML comments (`<!-- If applicable -->`, `<!-- endif -->`).

## Where the page goes

`docs/public/cli/<command>.md`, one file per command, named after the command (`init.md` for `rulelink init`). For a subcommand, use `<command>-<subcommand>.md`. Nothing is documented inside the CLI crate. `cargo xtask docs-check` fails when a command has no page or when an option of `--help` is missing from its page.

## Front matter

Every page starts with `title` (the command as typed), `description` (one sentence saying what the command does) and `since`, the version of Rulelink that first had the command (`0.1.0`). Write `since` by hand when you create the page; never change it afterwards.

## Sections

Keep them in this order.

1. **Title** (`# …`): the command as typed, e.g. `# rulelink init`, the same as `title` in the front matter.
2. **Description**: one to three sentences. Say what the command does and when to use it. Do not describe the implementation.
3. **Example**: a single `bash` block with the most common, copy-pasteable invocation. Add a second block only if a distinct use case needs it.
4. **Arguments**: positional parameters. Omit the whole section, heading included, if the command has none.
5. **Options**: flags and named options. Omit the whole section if the command has none. Every option of `--help` must appear, with its long form in backticks.
6. **Exit codes**: a table `code | meaning`. Always present.
7. **See also**: links to the related command pages and to the concept pages that explain the behaviour.

## Tables

Both tables have the columns `name | description | example`.

- **name**: exactly as typed, in backticks. For options, list the long form and the short form if one exists (`--force`, `-f`). Mark optional arguments with brackets (`[path]`) and required ones with angle brackets (`<path>`).
- **description**: one sentence, present tense. Include the default value when there is one (`Defaults to the current directory`).
- **example**: a literal value or invocation in backticks, valid as written (`--force`, `./my-project`).

Keep the table column widths aligned so the Markdown source stays readable.

## Rules

- The CLI itself is the source of truth. Check the real flags with `--help` or the argument definitions in `crates/cli` before writing, and never document a flag that does not exist.
- Write for end users: no internal type names, no crate or module paths.
- Use the project's terms from `GLOSSARY.md`.
- Every example must run as written.
- Document behaviour a user can observe (what is created, changed or printed), not how it works inside.
- When a command changes, update its page in the same change.
- The page describes the command and nothing else. Anything that explains how Rulelink works beyond it (a file format, the Annotation grammar, which files are scanned) goes in `docs/public/concepts/`, and the page links to it.
