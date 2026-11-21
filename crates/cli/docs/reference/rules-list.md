# rulelink rules list

Lists the Rules of the Source, one per line, with their Identifier, status and title, in Identifier order. Use `--status` to list only the Rules in one status, for example the `proposed` Rules that still need a review. Run it from the project root. Nothing is printed when no Rule matches.

## Example

```bash
rulelink rules list --status proposed
```

## Options

| name       | description                                                                                                       | example              |
| ---------- | ----------------------------------------------------------------------------------------------------------------- | -------------------- |
| `--status` | Lists only the Rules in this status: `proposed`, `validated`, `obsolete` or `rejected`. Defaults to every status. | `--status validated` |
