# The rules file is written in a line-oriented canonical form

Every write of the JSON source puts each object property and each array element on its own line, sorts rules in natural identifier order and keys in schema order, and never aligns or reflows. `rulelink rules format` rewrites a file into this form and `check` warns (`RL106`) when a file is not canonical.

Concurrent edits to business rules happen all the time in practice, so Git merge conflicts on the single `rules.json` are expected rather than exceptional. With one property per line, independent changes touch different lines and a real conflict is small enough to fix by hand; the formatter then normalises the result.

## Considered options

- Compact or default pretty-printing: rejected, arrays on one line make unrelated edits collide.
- Rely on the `json-dir` source (one file per rule) to avoid conflicts: kept for V1, but it does not help the MVP's single file.
