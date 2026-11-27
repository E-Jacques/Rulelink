# JSON reports accept new codes and fields within a version

A JSON report such as `rulelink/check-report/v1` is a published contract that agents and skills parse, and releases must not break it. Within one version, Rulelink may add Finding codes and fields: the schema takes any `RL` code followed by three digits and any kebab-case name, and does not set `additionalProperties: false`. Removing, renaming or retyping a field, or adding a severity, needs a new version. A consumer ignores unknown fields and relies on `severity` and `message` for an unknown code. A value that does not apply is left out, never `null`.

## Considered options

- Listing the exact codes in the schema: rejected, the source-integrity checks (RL003, RL1xx) would make an older copy of the schema refuse a newer report.
- Refusing undeclared fields in the schema: rejected, any added field would need a new version. The end-to-end tests check instead that the CLI prints no field the schema does not declare.

## Consequences

- The schema does not document the codes; the public Findings page does.
- A field that the current version always prints, but a later one may not, is optional from the start (`rule` and `file` in the check report).
