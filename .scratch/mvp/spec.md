# Spec: Rulelink MVP

- Status: open
- Labels: ready-for-agent

## Problem Statement

A developer working with AI agents on an existing project has no way to know which business **Rule** a piece of code implements, or which **validated** Rules no longer have any code. Business knowledge lives in thin Jira tickets and in people's heads; when the team changes, it is lost. Agents editing the code have no access to the Rules, so they break or ignore them. Existing tools either need a purchase or approval, or sit in CI platforms the developer does not control.

## Solution

A local, npm-installed CLI that the developer sets up alone in a repository, with no CI or installer integration (ADR-0001). It provides:

- a public Rule format (`"format": 1`) with a JSON Schema, stored in a single JSON **Source** written in **Canonical form** (ADR-0003);
- a Rule lifecycle (`proposed`, `validated`, `obsolete`, `rejected`) managed through `rules` commands, with permanent **Identifiers** and **Retired identifiers**;
- **Annotations** (`@rule <ID>` comments) found by a multi-language lexical scan;
- `check`, which reports **Findings** (human and JSON reports), plus `check --changed` for the agent loop;
- `explain`, `where`, `annotate` and `context` to move between Rules and code;
- a file **Import** that creates `proposed` Rules only;
- `init` and `doctor` for setup;
- two skills (consultation and retro-documentation) installed by hand.

All technical elements (keys, commands, messages) are English; only Rule values may be in another language (ADR-0002).

The MVP is done when: at least 50% of retro-documented Rules are accepted without rewriting; three external developers go from `npx rulelink init` to a first clean `check` in under 15 minutes, including one hand-written CI workflow running `npx rulelink check`; and agents with the skills installed get at least 8 of 10 tasks right (right Rules consulted, code annotated).

## User Stories

1. As a developer, I want to run `npx rulelink init` in my repository, so that I get a configuration and an empty Source without any approval or purchase.
2. As a developer, I want `init` to detect an existing setup and refuse to overwrite it, so that I never lose my Rules.
3. As a developer, I want `doctor` to diagnose my configuration, Source and environment, so that I can fix setup problems quickly.
4. As a developer, I want to configure the Identifier prefix (default `RULE`), so that Identifiers match my project's conventions.
5. As a developer, I want Rules validated against a published JSON Schema, so that editors and other tools can check the Source.
6. As a developer, I want a Rule to have a stable Identifier such as `RULE-042`, so that I can cite it in code forever.
7. As a developer, I want Identifiers to be allocated by the tool and never reused, so that an old Annotation can never silently point at a different Rule.
8. As a developer, I want to create a Rule with `rules` commands, so that I do not hand-edit JSON for routine work.
9. As a developer, I want new Rules to start as `proposed`, so that nothing is trusted before a human reviews it.
10. As a reviewer, I want to `approve` a Rule so it becomes `validated`, so that the team knows it is expected to have code.
11. As a reviewer, I want to `reject` a proposed Rule, so that it stays in the Source and is not proposed again.
12. As a maintainer, I want to mark a validated Rule `obsolete` (optionally with `superseded_by`), so that its retirement is recorded and code citing it is flagged.
13. As a maintainer, I want the Identifier of a rejected or obsolete Rule to be retired, so that it is never allocated again.
14. As a human reviewer, I want `approve` to be reserved to humans by skill text, so that an agent never validates its own proposals.
15. As a developer, I want to search Rules by keyword, accent- and case-insensitively, so that French or other-language values are easy to find.
16. As a developer, I want basic filtering (for example by status), so that I can list what needs review.
17. As a developer, I want a `related` field on Rules, so that I can record connections between Rules (cycle detection comes later).
18. As a developer, I want the Source always written in Canonical form, so that two people editing different Rules produce small, non-conflicting diffs.
19. As a developer, I want `rules format` to rewrite a file into Canonical form, so that I can normalise after resolving a merge conflict.
20. As a developer, I want `check` to warn (`RL106`) when the Source is not canonical, so that format drift is visible.
21. As a developer, I want source-integrity findings (`RL100`–`RL106`) for malformed or inconsistent Source files, so that a broken Source is detected before the code is checked.
22. As a developer, I want `check` to raise an error (`RL001`) when code cites an Identifier absent from the Source, so that typos and phantom Rules are caught.
23. As a developer, I want a warning (`RL002`) when code cites an `obsolete` Rule, with `superseded_by` suggested, so that I can migrate to the replacement step by step. A team can raise it to an error.
24. As a developer, I want an error (`RL003`) when two Rules share an Identifier, so that merges cannot corrupt the Source.
25. As a developer, I want a warning (`RL004`) for each `validated` Rule cited nowhere, so that I see which Rules have no code.
26. As a developer, I want a warning (`RL005`) when code cites a `proposed` Rule, so that unreviewed Rules do not go unnoticed.
27. As a developer, I want an error (`RL009`) when code cites a `rejected` or otherwise retired Identifier, so that dead references are removed.
28. As a team lead, I want warnings not to fail CI by default and errors to fail it, so that adoption on legacy code is progressive.
29. As a team lead, I want to raise chosen Findings to errors in configuration, so that my team can tighten the check over time.
30. As a developer, I want a human-readable `check` report with severity, code, message and location, so that I can act on it.
31. As an agent, I want `check --format json`, so that I can parse Findings reliably.
32. As an agent, I want `check --changed` to only report Findings about changed files and the Rules they touch, so that the loop is fast on large repositories.
33. As a developer, I want a distinct non-zero exit code on errors, so that a CI step fails correctly.
34. As a developer, I want to run `npx rulelink check` in a workflow I write by hand, so that I get CI enforcement without a maintained Action.
35. As a developer, I want the scan to find `@rule` comments in many languages and comment styles, so that the link works whatever the stack.
36. As a developer, I want one file to cite several Rules and one Rule to appear in several places, tests included, so that the link reflects reality.
37. As a developer, I want the scan to respect ignore files and configured exclusions, so that vendored or generated code is not scanned.
38. As a developer, I want `explain RULE-042` to show the Rule and every place that cites it, so that I understand what the code is for.
39. As a developer, I want `where RULE-042` to list code locations, so that I can jump to the implementation.
40. As a developer, I want `explain` and `where` to behave clearly for unknown, retired or uncited Identifiers, so that I get a useful answer instead of silence.
41. As a developer, I want `annotate RULE-042 <location>` to insert a correctly formatted Annotation in the right comment style, so that I do not hand-write it.
42. As an agent, I want `annotate` to be idempotent, so that repeated calls do not duplicate Annotations.
43. As an agent, I want `context <path>` to return the Rules relevant to a file or area, so that I consult them before editing.
44. As a developer, I want `context` to have a machine-readable output, so that skills can consume it.
45. As a developer, I want to import Rules from a JSON file, so that an existing Rule list (for example from Jira) can seed the Source.
46. As a developer, I want imported Rules to always land as `proposed`, so that an Import can never produce a `validated` Rule.
47. As a developer, I want imported text treated as untrusted data, so that a hostile Imported item cannot inject instructions or break the Source.
48. As a developer, I want re-importing not to duplicate Rules or reuse Identifiers, so that Import is safe to repeat.
49. As a developer, I want a consultation skill, so that my agent reads the relevant Rules before changing code and annotates what it writes.
50. As a developer, I want a retro-documentation skill, so that my agent proposes Rules from legacy code that I can review.
51. As a developer, I want both skills installed by hand with `npx skills add` or APM, so that no installer is required.
52. As a developer, I want the skills' reference files (annotation grammar, statuses) to match the real CLI behaviour, so that agents are not misled.
53. As a developer, I want to install via `npx rulelink` or `cargo install`, so that I choose my channel.
54. As a contributor, I want release archives and the npm package built by dist from a tag, so that releases are reproducible.
55. As a user on Linux, macOS or Windows, I want the same behaviour on each, so that my team can mix platforms.
56. As a non-English team, I want Rule values in my language, so that rules read naturally while the tool stays English.

## Implementation Decisions

- **Release scope** follows ADR-0001 and the features table in the product docs: everything marked MVP is in; V1/V2 items (labels, directory source, adapters, Jira/CSV/GitHub import, publishing, LSP, GitHub/GitLab integration, skills installer, `AGENTS.md` section, shell installers, Docker, SARIF/GitHub/JUnit reporters) are not.
- **Architecture**: hexagonal Rust workspace, one crate per responsibility (model, registry, link, check, import, config, adapter SDK, app, CLI). The CLI is a thin shell over the app crate's command bus; commands are the only public behaviour.
- **Rule model v1** is a public contract (`"format": 1`) with a JSON Schema generated from the model and verified in CI. Fields and keys are English (ADR-0002).
- **Identifier allocation** is monotonic per prefix and consults retired Identifiers, including those of Rules removed outside the tool, so reuse is impossible.
- **Lifecycle transitions** are enforced by the registry: `proposed→validated|rejected`, `validated→obsolete`; anything else is refused. Only humans approve (skill text, not a technical gate in the MVP).
- **Source**: a single JSON file behind the native adapter contract. Writes always go through the Canonical form (one property or array element per line, natural Identifier order, schema key order, no alignment). The adapter contract stays internal in the MVP.
- **Code linking**: a lexical, language-agnostic scan for `@rule <ID>` in comments; no tree-sitter and no enclosing-symbol resolution. `explain`, `where` and `annotate` are built on this scan.
- **Check**: findings `RL001`–`RL005`, `RL009` and source-integrity `RL100`–`RL106` (those marked MVP). Severities are configurable; errors fail the process, warnings do not by default. `RL006`–`RL008` are V1.
- **`check --changed`** limits reporting to files changed according to Git, to serve the agent loop. It is independent of any CI platform.
- **Reports**: `human` and `json` only. The JSON report schema is a contract consumed by the skills.
- **Import**: JSON file only; produces `proposed` Rules, never `validated`; imported text is untrusted.
- **`context`** returns the Rules linked to a path or area in a form suited to agents (human and JSON).
- **Skills**: `rulelink-consult` and `rulelink-retrodoc` live in the skills folder, with reference files kept in sync by a repo task.
- **Configuration**: created by `init`; keys are English; no locale or `--lang` option.
- **Distribution**: npm package plus `cargo install`; GitHub release archives built by dist.

## Testing Decisions

- **Single seam: the CLI process.** Tests run the real `rulelink` binary against fixture repositories in temporary directories and assert on stdout, stderr, exit code and resulting files. Internal crates are not tested through their own interfaces for behaviour already reachable from the CLI.
- A good test asserts external behaviour only: a given repository state and command produce given Findings, output and file contents. It never asserts on internal types, module layout or call order.
- Cover through the seam: every lifecycle transition (allowed and refused), Identifier allocation and non-reuse, Canonical form round-trip (`rules format` is idempotent; two independent edits merge cleanly in Git), each `CMxxx` Finding with a minimal fixture, `--changed` scoping on a Git fixture, `annotate` idempotency, Import always yielding `proposed`, hostile Import input, `explain`/`where`/`context` for known, unknown and retired Identifiers, JSON report shape against its schema, and exit codes.
- Add one end-to-end journey mirroring the exit criterion: `init`, import or create Rules, annotate code, `check` clean, then run the same `check` as a hand-written CI step.
- **Prior art**: the existing end-to-end test crate and the adapter conformance tests are the places this goes; the schema and skills consistency tasks in the xtask crate already verify generated artifacts.
- Validate the skills separately by the exit-criterion protocol (8 of 10 agent tasks), which is a manual evaluation rather than an automated test.

## Out of Scope

- Anything marked V1 or V2 in the features table: labels, filter language, directory source, `source migrate`, external adapters, Jira/CSV/GitHub/GitLab import, Markdown/CSV/JSON export, Confluence publishing, LSP and IDE extensions, baseline, pre-commit hook, self-approval warning, cycle detection, `RL006`–`RL008`, onboarding and other skills, tree-sitter, remote sources.
- GitHub Action, GitLab CI component, SARIF and `github` reporters, workflow generation by `init`.
- The `rulelink skills` installer, managed `AGENTS.md` section, shell/PowerShell installers, Homebrew/Scoop/winget, Docker image.
- Judging whether code correctly implements a Rule: `check` proves the link, not correctness.
- Writing to remote sources.

## Further Notes

- Repository scaffolding (workspace, CI, release pipeline, stubs for every crate) already exists; this spec covers the behaviour to build on it.
- Design detail lives in the product and architecture docs under `docs/`; if this spec and those docs diverge, flag it rather than silently choosing.
- Seam proposed to the user: CLI process only. Confirm or amend before slicing into issues.
