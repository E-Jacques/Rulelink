# Rulelink

Rulelink links business rules to the code that implements them, and reports which rules have no code.

## Language

**Rule**:
An atomic, testable statement of business behaviour with a stable identifier (e.g. `RULE-042`): a single verifiable criterion, either satisfied or not. The only object the registry manages.
_Avoid_: Requirement, user story, ticket, business rule (when ambiguous)

**Imported item**:
External data (a Jira issue, a CSV row, a GitHub issue) from which candidate Rules are extracted. Never a Rule itself, and treated as untrusted text.
_Avoid_: Ticket, issue (as a synonym for Rule)

**Source**:
The single place where Rules are authored and persisted (a JSON file in the MVP).
_Avoid_: Source of truth (in prose), database, rules.json (as a concept)

**Import**:
A one-shot pull of Imported items into the Source as `proposed` Rules. Never produces a `validated` Rule.
_Avoid_: Sync, synchronisation

**Publication**:
A generated, read-only rendering of the Rules (e.g. a Confluence page). Never read back as a Source.
_Avoid_: Export (for a Confluence page), sync

**Canonical form**:
The one line-oriented, sorted layout in which the Source file is always written, so independent edits touch different lines.

**Identifier**:
The permanent name of a Rule (e.g. `RULE-042`): an upper-case prefix (a letter, then upper-case letters, digits or `_`), one hyphen, then upper-case letters, digits or `_`. `RULE-042`, `BR-7` and `PRICING_V2-A1` are valid; `rule-042`, `RULE042`, `RULE-` and `1RULE-042` are not. Immutable: it is never changed, reassigned or reused, even after the Rule is rejected or obsolete.
_Avoid_: Code, key, number

## Rule lifecycle

**proposed**:
A Rule awaiting human review.

**validated**:
A Rule a human has approved. Only validated Rules are expected to have code.

**obsolete**:
A previously validated Rule that no longer applies. Its identifier is retired.

**rejected**:
A proposed Rule a human declined. It stays in the Source so it is not proposed again, and its identifier is retired.

**Retired identifier**:
An identifier of an obsolete or rejected Rule. It is never allocated again.

## Linking

**Annotation**:
A `@rule <ID>` comment in code that states which Rule the nearby code implements. The `@rule` must sit in a comment and be followed by a space, then one or more Identifiers separated by spaces or commas; the list stops at the first token that is not an Identifier. See `docs/public/concepts/specify-rules-in-code.md` for what is valid and its limits.
_Avoid_: Tag, link (as a noun for the comment)

**Finding**:
One result of `check`, with a severity, about a Rule or an Annotation.
_Avoid_: Error, violation, issue

## Documentation

**Public documentation**:
The documentation written for the people who use Rulelink (developers, product owners, functional roles), as opposed to the people who build it. It describes only what the current version does; anything announced appears only on the Roadmap.
_Avoid_: User manual, docs (alone, when the audience matters)

**Contributor documentation**:
The product, architecture and repository documents written for the people who build Rulelink.
_Avoid_: Internal docs, dossier

**Product**:
A distinct deliverable that users install or use: the CLI, or an editor extension. Release notes and the Roadmap are organised by Product, never by crate.
_Avoid_: Component, module, crate (when speaking to users)

**Release notes**:
The changes a Product brings between two versions, written for its users and generated from commit messages. Detail per crate belongs to the contributor-facing developer pages, not to Release notes.
_Avoid_: Changelog (for the user-facing text)

**Roadmap**:
The list of what each Product offers now, what comes next and what comes later, without dates.
_Avoid_: Backlog, plan
