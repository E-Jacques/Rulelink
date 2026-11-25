# Rulelink

**Every business rule stays attached to its code, even when the team changes.**

Rulelink links each business rule to the code that implements it. It helps you answer two questions:

1. Which rule does this code implement?
2. Which rules no longer have any code?

## How it works

A **Rule** is one short, verifiable statement with a permanent identifier. Rules are kept in a **Source**, a JSON file in the current version:

```json
{
  "id": "RULE-042",
  "title": "Line-level VAT rounding",
  "statement": "VAT is calculated and rounded to the cent on each invoice line, then summed.",
  "status": "validated"
}
```

Code points to a Rule with an **Annotation**, a comment that works in any language:

```rust
// @rule RULE-042 -- rounding applied before the sum
fn line_vat(line: &InvoiceLine) -> Money { ... }
```

Rulelink reads these comments, so a Rule can be traced to its code and back.

## Installation

Rulelink is distributed as the `rulelink` command-line tool. The installation steps for each operating system are on the documentation site, which owns this section:

<https://rulelink.dev/get-started/installation/> (the site is being set up and may not be live yet)

## Documentation

**Public documentation** is for people who use Rulelink. It will be published at <https://rulelink.dev> (not live yet). Until then, read it in this repository:

- [Rule format](docs/public/rule-format.md): what a Rule is and how it is named.
- [Specify rules in code](docs/public/specify-rules-in-code.md): how to write an Annotation.
- [CLI reference](crates/cli/docs/reference/init.md): one page per command, in the same folder.

**Contributor documentation** is for people who build Rulelink. Start with the [documentation index](docs/README.md), then see the [product overview](docs/product/01-product-overview.md), the [global architecture](docs/architecture/00-global-architecture.md) and the [repository structure](docs/repository/01-repository-structure.md). Terms are defined in the [glossary](GLOSSARY.md).

## Contributing

Contributions are welcome. [CONTRIBUTING.md](CONTRIBUTING.md) explains how to write commits and open a pull request. Working notes for AI agents are in [AGENTS.md](AGENTS.md).

## License

[MIT](LICENSE)
