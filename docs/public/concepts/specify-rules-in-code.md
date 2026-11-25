---
title: Specify Rules in code
description: How an Annotation in a comment links a Rule to the code that implements it.
---

# Specify Rules in code

You link a Rule to the code that implements it by writing an Annotation in a comment. Rulelink reads Annotations as plain text: nothing is enforced by your compiler, and the language does not matter. `rulelink where <ID>` lists every Annotation that cites a Rule.

## Annotation

```rust
// @rule RULE-042
// @rule RULE-042, RULE-043
// @rule RULE-042 -- rounding applied before the sum
```

```python
# @rule RULE-017
```

```sql
-- @rule RULE-101
```

```html
<!-- @rule RULE-205 -->
```

```java
/**
 * @rule RULE-042
 */
```

An Annotation is:

- `@rule`, in a comment, followed by a space (`@rules` is not an Annotation);
- then one or more Identifiers (see [Rule format](rule-format.md)), separated by spaces or commas;
- the list stops at the first token that is not an Identifier (`--`, `*/`, `-->`, a note). A trailing `.`, `:`, `;`, `)`, `]`, `"` or `'` on an Identifier is ignored.

Annotate tests too: a Rule cited from its tests is found like any other.

## What counts as a comment

The scan reads one line at a time and has no syntax tree. `@rule` is in a comment when the text before it on the same line has:

- `//`, `#`, `--`, `/*` or `<!--` anywhere before it;
- `;` or `%` as the first non-blank character of the line (Lisp, ini, assembly, TeX, Matlab). Elsewhere they are code: `foo(); @rule RULE-1` is not an Annotation;
- or the line starts with `*`, as inside a block comment.

## Known limits

- An opener inside a string literal earlier on the line (`"a // b" @rule RULE-1`) is taken for a comment.
- A multi-line block comment is recognised only on lines that start with `*`.

These are accepted false positives, to be removed when the scan understands the syntax of each language.

## Which files are scanned

Skipped: files ignored by `.gitignore`, `.ignore` or `.rulelinkignore`; files matching `scan.exclude` in `rulelink.toml`; hidden files; symbolic links; binary files; files that are not UTF-8; files over 2 MB.
