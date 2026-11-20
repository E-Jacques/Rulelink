# 02: Create Rules and allocate Identifiers

**What to build:** A `rules` command creates a Rule that starts as `proposed`, with a tool-allocated Identifier. Allocation is monotonic per prefix and consults Retired identifiers, including those of Rules removed from the Source outside the tool, so an Identifier is never reused.

**Blocked by:** 01 (Walking skeleton: `init` and the Source)

**Status:** ready-for-agent

- [x] Creating a Rule writes it as `proposed` with the next Identifier (e.g. `RULE-001`, `RULE-002`)
- [x] Output is always written in Canonical form
- [x] The Rule is validated against the schema; invalid input is refused with a clear message
- [x] An Identifier that was retired, or whose Rule was deleted by hand, is never allocated again
- [x] Rule values may be in any language; keys and messages stay English
