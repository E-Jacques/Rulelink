# rulelink rules format

Rewrites the rules file into Canonical form: one property or array element per line, Rules in natural Identifier order (`RULE-2` before `RULE-10`) and keys in a fixed order. Run it from the project root after resolving a Git merge conflict in the rules file, or after editing it by hand. It leaves a file that is already in Canonical form untouched, and stops without changing anything if the file is missing or is not valid.

## Example

```bash
rulelink rules format
```
