# checkmate init

Sets up Checkmate in the current directory by creating `checkmate.toml` and an empty rules file at `rules/rules.json`. Run it once, at the root of your project, before you add any Rule. It stops without changing anything if either file already exists.

## Example

```bash
checkmate init
```

## Options

| name          | description                                                                                                        | example          |
| ------------- | ------------------------------------------------------------------------------------------------------------------ | ---------------- |
| `--id-prefix` | Sets the prefix of Rule identifiers (capital letters, digits and `_`, starting with a letter). Defaults to `RULE`. | `--id-prefix BR` |
