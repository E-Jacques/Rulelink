# checkmate rules add

Adds a new Rule to your rules file. The Rule starts as `proposed` and receives the next free Identifier (`RULE-001`, then `RULE-002`, and so on); an Identifier is never given out twice, even if its Rule was later retired or deleted by hand. Run it from the root of a project set up with `checkmate init`.

## Example

```bash
checkmate rules add --title "Line-level VAT rounding" --statement "VAT is rounded on each invoice line, not on the invoice total."
```

On success it prints the Identifier it allocated, for example `RULE-001`.

## Options

| name          | description                                                                                       | example                                  |
| ------------- | ------------------------------------------------------------------------------------------------- | ---------------------------------------- |
| `--title`     | Sets the short title of the Rule (1 to 120 characters). Required.                                 | `--title "Line-level VAT rounding"`      |
| `--statement` | Sets the statement of the Rule. Required.                                                         | `--statement "VAT is rounded per line."` |
| `--app`       | Restricts the Rule to an application; repeat the option for several. Defaults to the whole project. | `--app billing`                          |
| `--module`    | Sets the functional module of the Rule.                                                           | `--module invoicing`                     |
| `--covered`   | Adds a case the Rule covers; repeat the option for several.                                       | `--covered "Invoices in EUR"`            |
| `--excluded`  | Adds a case the Rule excludes; repeat the option for several.                                     | `--excluded "Credit notes"`              |
| `--related`   | Links the Rule to an existing Rule by Identifier; repeat the option for several.                  | `--related RULE-001`                     |
| `--origin`    | Records where the Rule comes from, as `kind:reference`.                                           | `--origin jira:BILL-42`                  |

Values may be written in any language. If the input is invalid (an empty or too long title, an unknown related Rule, a malformed origin), the command explains why and leaves the rules file untouched.
