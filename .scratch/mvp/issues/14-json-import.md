# 14: JSON file Import

**What to build:** A developer imports Rules from a JSON file (e.g. a list exported from Jira) to seed the Source. Imported items always land as `proposed` Rules, never `validated`. Imported text is untrusted data. Re-importing neither duplicates Rules nor reuses Identifiers.

**Blocked by:** 03 (Rule lifecycle)

**Status:** ready-for-agent

- [ ] Every imported Rule is `proposed`, even if the input claims another status
- [ ] Hostile input (instruction-like text, control characters, malformed JSON, oversized fields) cannot break the Source or inject behaviour; invalid items are reported and skipped or refused
- [ ] Re-running the same Import creates no duplicates and allocates no reused Identifiers
- [ ] Identifiers are allocated by the tool, never taken from the input
- [ ] The Source stays in Canonical form
