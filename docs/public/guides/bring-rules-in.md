---
title: Bring Rules in from another tool
description: Learn how to bring a list of existing Rules into the Source as proposed Rules that a person reviews.
---

# Bring Rules in from another tool

You may already have business rules in a spreadsheet, a document or another tool. In this guide you bring them into the Source as `proposed` Rules. Nothing you bring in is `validated` until a person approves it.

## Before you start

- Run `rulelink init` at the root of your project. See [`rulelink init`](../cli/init.md).
- Export your existing rules to a text file with one rule per line. Put a tab between the title and the statement.

Here is an example, saved as `rules-to-import.tsv`:

```text
Invoice has a number	Every invoice has a number.
Discount is capped	A discount must not be above 30 percent.
```

## Add each line as a Rule

Treat the text you bring in as untrusted. It is a list of candidates, not a list of Rules. Rulelink does not read the file for you today, so you add each line with `rulelink rules add`. This shell loop does it for a Unix shell:

```bash
while IFS=$'\t' read -r title statement; do
  rulelink rules add --title "$title" --statement "$statement"
done < rules-to-import.tsv
```

Each line becomes one Rule with a new Identifier:

```text
Created RULE-001 (proposed)
Created RULE-002 (proposed)
```

The Identifiers come from Rulelink. Do not reuse the numbers of the other tool.

## Review the proposed Rules

List the Rules that wait for a review:

```bash
rulelink rules list --status proposed
```

```text
RULE-001  proposed   Invoice has a number
RULE-002  proposed   Discount is capped
```

Read each one. A good Rule states one criterion. If a line holds two criteria, reject it and add two new Rules. See [Write a good Rule](write-a-good-rule.md).

Then approve or reject each Rule. See [Manage a Rule's lifecycle](manage-a-rule-lifecycle.md).

```bash
rulelink rules approve RULE-002
rulelink rules reject RULE-001
```

## What is not available yet

Other imports, for example from Jira, are announced. They are not available in this version. The Roadmap lists what comes next.

## Next steps

- [Write a good Rule](write-a-good-rule.md)
- [Manage a Rule's lifecycle](manage-a-rule-lifecycle.md)
- [`rulelink rules add`](../cli/rules-add.md)
- [`rulelink rules list`](../cli/rules-list.md)
