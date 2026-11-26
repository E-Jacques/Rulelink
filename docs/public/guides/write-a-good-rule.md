---
title: Write a good Rule
description: Learn how to write a Rule that states one criterion anyone can verify, and how to cite it in code.
---

# Write a good Rule

A good Rule is one criterion that is either satisfied or not. In this guide you write three Rules about a surface field, approve them and cite them in code. Short Rules are easy to check, easy to test and easy to retire.

## Before you start

- Install the `rulelink` command.
- Run `rulelink init` at the root of your project. See [`rulelink init`](../cli/init.md).

## State one criterion

Ask one question: can a person answer yes or no by looking at the code or a test? If yes, the Rule is good. If you need the word "and", write two Rules.

Here are three Rules for a surface field. Each one has a single criterion.

```bash
rulelink rules add --title "Surface is filled" --statement "The surface field must be filled."
rulelink rules add --title "Surface has a maximum" --statement "The surface must not be above 10,000."
rulelink rules add --title "Surface has a minimum" --statement "The surface must not be below 9 m²."
```

Each command prints the new Identifier:

```text
Created RULE-001 (proposed)
Created RULE-002 (proposed)
Created RULE-003 (proposed)
```

The title is a short label. The statement is the criterion. Write the statement in the language of the business, not of the code.

## Approve the Rules

A new Rule is `proposed`. Once a person has reviewed it, approve it.

```bash
rulelink rules approve RULE-001
rulelink rules approve RULE-002
rulelink rules approve RULE-003
```

## Cite the Rules in code

Write an Annotation in a comment, next to the code that implements the Rule. Cite the Rule in your tests too.

```rust
// @rule RULE-001
fn surface_is_filled(surface: Option<f64>) -> bool {
    surface.is_some()
}

// @rule RULE-002
// @rule RULE-003
fn surface_is_in_range(surface: f64) -> bool {
    (9.0..=10_000.0).contains(&surface)
}
```

Save this in `src/surface.rs`. Then ask where a Rule is cited:

```bash
rulelink where RULE-001
```

```text
src/surface.rs:1
```

Every validated Rule should print at least one place. See [Specify Rules in code](../concepts/specify-rules-in-code.md) for what counts as an Annotation.

## Do and do not

| Do                                       | Do not                                                                      |
| ---------------------------------------- | --------------------------------------------------------------------------- |
| "The surface field must be filled."      | "The form must be valid." (vague: valid how?)                               |
| "The surface must not be above 10,000."  | "The surface must be filled and must not be above 10,000." (two criteria)   |
| "The surface must not be below 9 m²."    | "Use a `for` loop to reject surfaces below 9." (an instruction, not a Rule) |
| One Rule, one `@rule` comment, one test. | One long Rule cited by half of the project.                                 |

## Next steps

- [Rule format](../concepts/rule-format.md)
- [Specify Rules in code](../concepts/specify-rules-in-code.md)
- [`rulelink rules add`](../cli/rules-add.md)
- [`rulelink where`](../cli/where.md)
