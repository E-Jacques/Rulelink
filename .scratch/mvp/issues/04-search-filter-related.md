# 04: Search, filter and `related`

**What to build:** Developers can search Rules by keyword (accent- and case-insensitive, so French or other-language values are findable), filter by status to list what needs review, and record connections between Rules with a `related` field. Cycle detection is out of scope.

**Blocked by:** 02 (Create Rules and allocate Identifiers)

**Status:** ready-for-agent

- [ ] Keyword search matches regardless of case and accents
- [ ] Filtering by status lists only matching Rules
- [ ] A Rule can carry `related` Identifiers, stored in Canonical form and covered by the schema
- [ ] `related` pointing at an unknown Identifier is handled clearly (refused or reported)
