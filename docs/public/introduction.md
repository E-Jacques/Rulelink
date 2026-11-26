---
title: Introduction
description: Learn why Rulelink exists, what a Rule is and where to read next.
---

# Introduction

Rulelink links the business rules of your product to the code that implements them. This page explains why, what a Rule is and how the pieces fit. It takes a few minutes to read.

## The problem

Business knowledge hides in code. A team decides that VAT is rounded on each invoice line. A developer writes it. A year later the team has changed, the ticket is thin, and nobody knows why the code rounds that way, or whether it still should.

Rulelink keeps that knowledge attached to the code, even when the team changes.

## Two questions

Rulelink helps you answer two questions:

1. **Which Rule does this code implement?**
2. **Which Rules have no code?**

Rulelink shows that a Rule is linked to code. It does not prove the code is correct. Your tests stay the best way to check that. Rulelink links the tests to the Rule, so you can find them.

## How it works

Three things work together.

1. **Rules.** A Rule is one statement of business behaviour that is either satisfied or not. It has a permanent Identifier, such as `RULE-042`. All Rules live in one file, the Source, inside your project.
2. **Annotations.** You write the Identifier in a comment next to the code that implements the Rule: `// @rule RULE-042`. It works in any language.
3. **The command line.** The `rulelink` command creates and reviews Rules, and lists the places that cite a Rule.

Here is the whole loop:

```bash
rulelink init
rulelink rules add --title "Line-level VAT rounding" --statement "VAT is rounded to the cent on each invoice line, then summed."
rulelink rules approve RULE-001
rulelink where RULE-001
```

The last command lists every Annotation that cites `RULE-001`, tests included. It prints `RULE-001 is not cited by any code` until you add one.

## The basic rules

- **One Rule, one criterion.** A Rule states one thing anyone can verify.
- **A human decides.** A new Rule is `proposed`. It becomes `validated` only when a person approves it. A person can also reject it.
- **An Identifier is permanent.** It is never changed or reused, even when its Rule is `rejected` or `obsolete`.
- **The Source is the one place.** You write and keep Rules there, and nowhere else.
- **The link is a comment.** An Annotation is plain text, so Rulelink needs no change to your code or your tools.

## Next steps

- Learn what a Rule contains in [Rule format](concepts/rule-format.md).
- Learn how to link a Rule to code in [Specify Rules in code](concepts/specify-rules-in-code.md).
- Browse every command in the [CLI reference](cli/README.md).
- Read the [Roadmap](roadmap.md) to see what exists today and what is announced.
