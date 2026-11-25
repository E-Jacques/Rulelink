---
title: Glossary
description: Find the meaning of each term used in the Rulelink documentation.
---

# Glossary

The terms used across this documentation, defined once.

## Rules

**Rule**
: An atomic, testable statement of business behaviour with a stable Identifier, such as `RULE-042`. It states one criterion that is either satisfied or not. It is the only object Rulelink manages.

**Source**
: The single place where Rules are written and stored. It is one JSON file.

**Identifier**
: The permanent name of a Rule, such as `RULE-042`. It is never changed, reassigned or reused, even after the Rule is rejected or obsolete. See [Rule format](../concepts/rule-format.md) for the valid forms.

**Canonical form**
: The one line-oriented, sorted layout in which the Source file is always written, so that independent edits touch different lines. `rulelink rules format` restores it.

## Rule lifecycle

**proposed**
: A Rule that waits for a human review.

**validated**
: A Rule a human has approved. Only validated Rules are expected to have code.

**obsolete**
: A validated Rule that no longer applies. Its Identifier is retired.

**rejected**
: A proposed Rule a human declined. It stays in the Source so it is not proposed again, and its Identifier is retired.

**Retired identifier**
: The Identifier of an obsolete or rejected Rule. It is never allocated again.

## Linking

**Annotation**
: A `@rule <ID>` comment in code that states which Rule the nearby code implements. See [Specify Rules in code](../concepts/specify-rules-in-code.md).

## Releases

**Product**
: A distinct deliverable you install or use, such as the command line tool.

**Release notes**
: The changes a Product brings between two versions.

**Roadmap**
: What each Product offers now, what comes next and what comes later, without dates.

## Next steps

- [Rule format](../concepts/rule-format.md)
- [Specify Rules in code](../concepts/specify-rules-in-code.md)
- [CLI reference](../cli/README.md)
