# checkmate rules add

Creates a Rule in your rules file. The new Rule starts as `proposed` and receives the next free Identifier for the configured prefix (for example `RULE-001`, then `RULE-002`). An Identifier is never given out twice, even if a Rule was retired or deleted by hand. Run it from the root of a project set up with `checkmate init`.

## Example

```bash
checkmate rules add --title "Refund window" --statement "A customer can request a refund within 30 days of delivery."
```

## Options

| name          | description                                                                                     | example                              |
| ------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------ |
| `--title`     | Sets the short title of the Rule (1 to 120 characters). Required.                               | `--title "Refund window"`            |
| `--statement` | Sets the statement of the Rule. Required.                                                       | `--statement "Refunds are allowed."` |
| `--app`       | Names an application the Rule applies to. Repeat it for several. Defaults to the whole project. | `--app billing`                      |
| `--module`    | Sets the functional module of the Rule.                                                         | `--module payments`                  |
| `--covered`   | Adds a case the Rule covers. Repeat it for several.                                             | `--covered "Physical goods"`         |
| `--excluded`  | Adds a case the Rule excludes. Repeat it for several.                                           | `--excluded "Gift cards"`            |
| `--related`   | Links an existing Rule by Identifier. Repeat it for several.                                    | `--related RULE-001`                 |
| `--origin`    | Records where the Rule comes from, as `<kind>:<reference>`. Defaults to no origin.              | `--origin jira:SHOP-42`              |

Values can be written in any language. Invalid input (a missing or too long title, an empty statement, an unknown related Rule) is refused with a message and leaves the rules file untouched.
