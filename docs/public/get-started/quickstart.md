---
title: Quickstart
description: Learn how to set up Rulelink, create your first Rule and find the code that cites it.
---

# Quickstart

In a few minutes you will set up Rulelink in a project, create your first Rule, link it to code with an Annotation and find that code again. Each command below is run against the real `rulelink` command by a test, so the page matches what you see.

## Before you start

- The `rulelink` command, installed and on your `PATH`.
- A project directory. An empty one is fine.

Run every command from the root of your project.

## 1. Set up the project

[`rulelink init`](../cli/init.md) creates the configuration, `rulelink.toml`, and an empty rules file, the Source.

```console
$ rulelink init
Initialised Rulelink: wrote rulelink.toml and rules/rules.json
```

## 2. Create your first Rule

[`rulelink rules add`](../cli/rules-add.md) creates a Rule and prints its Identifier. A new Rule is `proposed`: a person still has to approve it.

```console
$ rulelink rules add --title "Line-level VAT rounding" --statement "VAT is rounded to the cent on each invoice line, then summed."
Created RULE-001 (proposed)
```

## 3. Annotate it in code

Write the Identifier in a comment next to the code that implements the Rule. This is an Annotation. It works in any language. Add it with your editor, or with these commands:

```console
$ mkdir -p src
$ printf '// @rule RULE-001\nfn round_vat() {}\n' > src/pricing.rs
```

## 4. Find the code that cites the Rule

[`rulelink where`](../cli/where.md) lists every Annotation that cites a Rule, as `file:line`.

```console
$ rulelink where RULE-001
src/pricing.rs:1
```

You now have a Rule, and the code that implements it is one command away.

## What comes next

A command that checks the whole project for Findings is announced, not available yet. See the [Roadmap](../roadmap.md). Until then, use `rulelink where` to see whether a Rule has code.

## Next steps

- Learn what a Rule contains in [Rule format](../concepts/rule-format.md).
- Learn what counts as an Annotation in [Specify Rules in code](../concepts/specify-rules-in-code.md).
- Write better Rules with [Write a good Rule](../guides/write-a-good-rule.md).
- Review and approve Rules in [Manage a Rule lifecycle](../guides/manage-a-rule-lifecycle.md).
