# Rule format

What a Rule is, and how it is named. For the commands that create and change Rules, see the [CLI reference](../cli/).

## Identifier

Every Rule has an Identifier, allocated by `rulelink rules add` (`RULE-001`, `RULE-002`, …). It is permanent: it is never changed, reassigned or reused, even after the Rule is rejected or obsolete.

An Identifier is:

1. a prefix: an upper-case ASCII letter, then upper-case letters, digits or `_`;
2. one hyphen;
3. a suffix: one or more upper-case letters, digits or `_`.

| Identifier      | Valid | Why                                    |
| --------------- | :---: | -------------------------------------- |
| `RULE-042`      |  yes  |                                        |
| `BR-7`          |  yes  |                                        |
| `PRICING_V2-A1` |  yes  |                                        |
| `rule-042`      |  no   | Lower case.                            |
| `RULE042`       |  no   | No hyphen.                             |
| `RULE-`         |  no   | Empty suffix.                          |
| `1RULE-042`     |  no   | The prefix starts with a digit.        |
| `RULE-04a`      |  no   | Lower case in the suffix.              |

## Lifecycle

A Rule is `proposed` until a human reviews it, then `validated`, or `rejected`. A validated Rule that no longer applies becomes `obsolete`. The Identifier of a rejected or obsolete Rule is retired and stays in the Source.

## Source file

Rules live in one JSON file, the Source, always written in a canonical, line-oriented layout so that independent edits touch different lines. `rulelink rules format` restores that layout, for instance after a Git merge conflict.
