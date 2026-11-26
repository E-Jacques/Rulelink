# Annotation grammar

An annotation links a piece of code to a business rule. It is a tag followed by one or more rule identifiers, written **inside a comment**.

```
@rule RULE-042
@rule RULE-042, RULE-043
@rule RULE-042 -- rounding applied before the sum
```

## Grammar

- Tag: `@rule` by default (configurable by `scan.tag`).
- Identifiers match the project's pattern, `RULE-[A-Z0-9_]+` by default. Separate several with a comma or a space.
- An optional note follows `--` and runs to the end of the line. `check` ignores it.
- Several annotations per file, and one rule in several places (tests included), are normal.
- A tag without a valid identifier is reported as malformed (`RL006`).
- Only comments are scanned. `@rule` inside a string literal is not an annotation.

## Examples per comment style

```rust
// @rule RULE-042
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

```c
/* @rule RULE-042 */
```

Other languages use their own comment syntax: `//` (Go, TypeScript, C#, Kotlin, Swift, PHP), `#` (Shell, Ruby, YAML, TOML, Terraform), `--` (Lua, Haskell), `;` (Lisp-like, INI), `%`. Unknown extensions fall back to a generic scan that accepts the tag after any common comment prefix.

## Where to place it

Put the annotation directly above the code that implements or tests the rule (a function, a block, a test). Use `rulelink annotate <path>:<line> <RULE-ID>` rather than editing by hand: it picks the right comment syntax for the language and prints the diff. Add `--dry-run` to preview, `--remove` to take an identifier out.

## Rules to follow

- Only cite rules that exist in the source. Never invent an identifier.
- Do not cite a `rejected` or `obsolete` rule: `rulelink check` reports an error. See `statuses.md`.
- Never rename an identifier. It is immutable.
- Run `rulelink check` after annotating.

## Placement when documenting legacy code

When retro-documenting, annotate the code that actually enforces each rule, not the file header or the module declaration. If a rule is spread across several places, annotate each one. Annotate only after a human has validated the rule: before that, the rule is `proposed` and an annotation produces a warning (`RL005`).
