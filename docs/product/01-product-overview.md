# Rulelink — Product overview

> What the product is, who it is for and why it exists. This file restates the product brief (_Product brief — Business rule traceability_, 31 October 2026) in English; the brief remains the reference for product decisions.
> See also: [Features and releases](02-features-and-releases.md), [Global architecture](../architecture/00-global-architecture.md).

## 1. Summary

Rulelink (final name, confirmed) keeps a team from losing the business knowledge buried in its code. It answers two questions:

1. **Which business rule does this code implement?**
2. **Which rules no longer have any code?**

**Promise:** every business rule stays attached to its code, even when the team changes.

The MVP is deliberately simple: a configured source of truth, a rule identifier in the code, and a deterministic check in CI. AI only intervenes through skills run by the developer's own agent, with human validation.

**Product bet:** maintaining the link between rules and code costs less than rebuilding the knowledge once it is gone. Because the project is run by one person, alongside other work, the MVP is limited to a CLI, a rule format, a JSON source, a CI check and two skills.

## 2. Origin of the need

The need comes from the field, from both sides:

| Situation                                                                                                 | What it shows                                      |
| --------------------------------------------------------------------------------------------------------- | -------------------------------------------------- |
| A recent build already works with a manual flow: Jira → JSON → Confluence                                 | The flow is valuable and worth tooling             |
| Another maintained application where nobody knows any more what was built, and Jira tickets are very thin | The cost of losing the link between rules and code |

In the existing flow, the PO records rules in Jira and the team extracts them; a local JSON file gathers every rule of the project; an export produces a single Confluence page, easier for the PO to read, showing the impacted module, the scope and the application. **The JSON file is the real source of truth: Jira feeds it and Confluence publishes it.**

## 3. Target users

The first problem to solve is the loss of business knowledge, not agent hallucination. A reliable rule base then also frames the agents' work.

| Priority | Target                                                  | What they get                                                                  | Release |
| -------- | ------------------------------------------------------- | ------------------------------------------------------------------------------ | ------- |
| 1        | Developer working with AI agents on an existing project | Installs it alone in the repository; rules available to the agent; check in CI | MVP     |
| 2        | Product Owner / Manager                                 | A readable reference page, without changing tools                              | V1      |

The first user is always the same: a developer able to install the tool without a purchase or an approval.

### 3.1 Three entry points

| Context                     | Where Rulelink starts                                                                                               |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Legacy code                 | Bootstrapping through the retro-documentation skill. The most demanding test, and the closest to the pain described |
| Active project (greenfield) | New rules, annotated as development goes, with agents                                                               |
| Business applications       | The existing rule repository, imported from Jira or a file                                                          |

## 4. Product principles

| Principle                                                  | Consequence                                                                                                                    |
| ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| The rule is the central object, with a public format       | Every connector translates to one model; checks become possible                                                                |
| One source of truth per project, declared in configuration | No synchronisation conflicts; imports create proposed rules, publications are generated and never edited                       |
| The link is a comment in the code: `// @rule RULE-042`     | Works in any language; one file can cite several rules, one rule can appear in several places, tests included                  |
| The check proves the link, not the correctness             | Stated explicitly to users; tests remain the best check of a rule, and Rulelink links them to rules rather than replacing them |
| Warnings do not block CI by default                        | Progressive adoption on existing code; each team can raise them to errors                                                      |
| AI only through the developer's agent                      | No AI infrastructure to host; human validation of every proposed rule                                                          |

## 5. Rule model (product view)

| Field       | Required | Purpose                                                                                     |
| ----------- | -------- | ------------------------------------------------------------------------------------------- |
| Identifier  | Yes      | Stable and never reused, for example `RULE-042`                                             |
| Title       | Yes      | Short name, readable by the PO                                                              |
| Statement   | Yes      | Expected behaviour, in business language                                                    |
| Status      | Yes      | `proposed`, `validated` or `obsolete`                                                       |
| Application | No       | Application concerned                                                                       |
| Module      | No       | Module impacted                                                                             |
| Scope       | No       | Covered cases and excluded cases                                                            |
| Origin      | No       | Link to where the rule comes from: Jira key, other tool, source page or retro-documentation |
| Related     | No       | Identifiers of related rules                                                                |

The technical format and its JSON keys are specified in [architecture/01](../architecture/01-rule-registry.md).

## 6. Deterministic checks (product view)

| Check                  | Case detected                                        | Default level |
| ---------------------- | ---------------------------------------------------- | ------------- |
| Unknown reference      | The code cites an identifier missing from the source | Error         |
| Obsolete rule cited    | The code still cites an abandoned rule               | Error         |
| Duplicate identifier   | Two rules share the same identifier                  | Error         |
| Rule without code      | A validated rule is cited nowhere                    | Warning       |
| Unvalidated rule cited | The code cites a rule that is still proposed         | Warning       |

## 7. Key user journeys

| Journey    | How                                                                                                                       |
| ---------- | ------------------------------------------------------------------------------------------------------------------------- |
| Initialise | `rulelink init` creates the configuration and installs the skills                                                         |
| Bootstrap  | The retro-documentation skill proposes rules with their locations; a human validates, then the skill adds the annotations |
| Develop    | The consultation skill gives the agent the relevant rules and asks it to annotate its code                                |
| Check      | `rulelink check` runs on every pull request                                                                               |
| Understand | `rulelink explain <file>` lists the rules of a file; `rulelink where RULE-042` lists the locations of a rule              |

## 8. Needs coverage

| Target    | Need                                                      | Answer                                                     | Release |
| --------- | --------------------------------------------------------- | ---------------------------------------------------------- | ------- |
| Developer | Identify the rule behind a piece of code                  | `explain` and annotations                                  | MVP     |
| Developer | Give agents a frame                                       | Consultation skill                                         | MVP     |
| Developer | Find a rule in the code                                   | `where`                                                    | MVP     |
| Developer | Avoid hallucinations                                      | Skill and `check`: the risk decreases without disappearing | MVP     |
| PO        | Find a rule and its scope                                 | Confluence publication with module, scope and application  | V1      |
| PO        | Know whether a rule still exists and follow its lifecycle | Status, `check` result and code links in the publication   | V1      |
| PO        | Write specifications and aim for completeness             | Generation and review skills                               | V2      |
| PO        | Spot inconsistencies and related rules                    | `related` field and review skill                           | V2      |

## 9. Alternatives and positioning

Linking by comment is not new. The difference must come from the complete flow: a source of the team's choice, a PO-readable output, AI agents and legacy bootstrapping.

| Alternative   | What it does                                                                       | Room left for Rulelink                                                     |
| ------------- | ---------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| OpenFastTrace | Open-source requirement tracing in Java (GPL-3.0), with tags in the code           | No Jira, no PO view, no AI agents                                          |
| Spec Kit      | Specification process for AI agents (MIT)                                          | No rule repository linked to code                                          |
| OpenSpec      | Markdown specifications for agents (MIT), also aimed at existing code              | Organised around changes, no rule ↔ code link                              |
| Kiro          | IDE that generates requirements, design and tasks                                  | No explicit link to code; imposes its environment                          |
| Cucumber      | Executable specifications checked against the system                               | Stronger on correctness, but every scenario must be written and maintained |
| Swimm         | Commercial extraction of legacy rules, deterministic then AI, validated by experts | Aimed at large modernisations, not a daily open-source tool                |

## 10. Openness

Everything a developer uses in their repository is open source (MIT): rule format, CLI, `check`, CI action, skills, adapters, Jira import, Confluence publishing, VS Code extension.

What makes the project useful to the community:

- an open, versioned format that a team can adopt even without the tool;
- a documented adapter contract, so a connector contribution stays small, with a reference example;
- no account required: the CLI and `check` work offline;
- a ready-to-use CI action: adoption takes a few lines of configuration;
- an example repository showing a complete bootstrap on existing code;
- portable skills that work with the main agents, not just one.

## 11. Success criteria

The first test is retro-documentation on code whose knowledge has been lost: it validates the riskiest scenario and can be run before the CLI exists. Thresholds are proposals to adjust, not measurements.

| Test                | Protocol                                                                                         | Proposed threshold                                                                                                     | If the threshold is missed                               |
| ------------------- | ------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- |
| Retro-documentation | Extract the rules of one module, then have them reviewed by the person who knows the domain best | At least 50% of rules accepted without rewriting                                                                       | Rework the skill, or reduce the promise to a writing aid |
| Internal use        | Replace the manual flow on the recent build, with `check` in CI for 4 weeks                      | CI kept, and at least 80% of validated rules linked to code                                                            | Find what slows annotation down before adding features   |
| Agents              | Run 10 real tasks with the consultation skill                                                    | At least 8 tasks with the right rules consulted and the code annotated                                                 | Strengthen the skill, or add a stricter annotation check |
| Installation        | Have 3 external developers install the tool unaided                                              | Under 15 minutes from `npx rulelink init` to a first clean `check`, including one run in a CI workflow written by hand | Simplify `init` and the documentation                    |

**Main indicator:** number of repositories where `check` has run in CI for at least 30 days. It measures real use, unlike GitHub stars or installs.

## 12. Product decisions and open questions

| Topic                                                     | Decision                                                                                                                                                  |
| --------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Model                                                     | Open source (MIT)                                                                                                                                         |
| First target                                              | Developer working with AI agents                                                                                                                          |
| Source of truth                                           | A single interface, declared in configuration                                                                                                             |
| Link with code                                            | Comment in the code                                                                                                                                       |
| MVP check                                                 | Deterministic only                                                                                                                                        |
| Legacy bootstrap                                          | Retro-documentation skill with human validation                                                                                                           |
| Name                                                      | Rulelink (confirmed)                                                                                                                                      |
| Rule identifier with a Jira source                        | Follows the project's standard: the ticket key or an in-house identifier; the link to the ticket goes in `origin`                                         |
| Recognising rules in a Jira ticket                        | Configurable regex per project; the default pattern "RULE XXXX_N : my business rule" extracts the code and the statement; a ticket can hold several rules |
| Validation of proposed rules when the PO does not use Git | Developers add rules locally and an export gives the PO an extract                                                                                        |
| Several repositories sharing one source                   | Possible, especially with a remote source (V2)                                                                                                            |
| Licence                                                   | MIT                                                                                                                                                       |
| CLI language                                              | Rust, for a single binary without a runtime (verdict in [architecture/00](../architecture/00-global-architecture.md), section 2)                          |
| Agents supported by the skills                            | All, through community tooling (`npx skills`, APM)                                                                                                        |
| Code base for the retro-documentation test                | Not the old application: another code base is needed                                                                                                      |
