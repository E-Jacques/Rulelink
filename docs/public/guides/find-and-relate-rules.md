---
title: Find and relate Rules
description: Learn how to find Rules in a large Source, see where they are cited and record related Rules.
---

# Find and relate Rules

A large Source holds many Rules. You can list them, search them, and see which code cites each one. You can also record that a new Rule is related to another.

## Before you start

- A project with a Source and a few Rules. See [`rulelink rules add`](../cli/rules-add.md).
- Run every command from the project root.

## List Rules by status

List every Rule, one per line, in Identifier order. Add `--status` to keep one status.

```bash
rulelink rules list
rulelink rules list --status proposed
```

Each line shows the Identifier, the status and the title:

```text
RULE-001  validated  Line-level VAT rounding
RULE-002  proposed   Reduced VAT rate on books
```

## Search for a word

Search finds the Rules that contain a word. It looks in the Identifier, the title, the statement, the module, the applications and the covered and excluded cases. It ignores case and accents, so `enonce` finds a Rule titled "Énoncé".

```bash
rulelink rules search vat
```

Narrow the result to one status:

```bash
rulelink rules search vat --status validated
```

Nothing is printed when no Rule matches. An empty search text is an error.

## See where a Rule is cited

Once you have an Identifier, list the code that cites it. Each line is a file and a line number.

```bash
rulelink where RULE-001
```

```text
src/pricing.rs:1
```

A Rule that no Annotation cites prints `RULE-002 is not cited by any code`. See [Specify Rules in code](../concepts/specify-rules-in-code.md) to add Annotations.

## Record a related Rule

Two Rules can be related, for example when one refines another. Record the link when you create the new Rule, with `--related`. The Rule you name must already exist. Repeat the option for several Rules.

```bash
rulelink rules add --title "Reduced VAT rate on books" --statement "Books are invoiced with the reduced VAT rate." --module "VAT calculation" --related RULE-001
```

If the Identifier does not exist, the command stops with an error and changes nothing.

The link is stored in the `related` field of the new Rule in the Source file. No command lists or edits related Rules yet.

## Next steps

- [`rulelink rules list`](../cli/rules-list.md)
- [`rulelink rules search`](../cli/rules-search.md)
- [`rulelink where`](../cli/where.md)
- [`rulelink rules add`](../cli/rules-add.md)
- [Rule format](../concepts/rule-format.md)
