# 03: Rule lifecycle: approve, reject, obsolete

**What to build:** `rules approve` turns a `proposed` Rule into `validated`; `rules reject` declines a `proposed` Rule (it stays in the Source); `rules obsolete` retires a `validated` Rule, optionally recording `superseded_by`. Rejected and obsolete Identifiers become Retired identifiers. Any other transition is refused.

**Blocked by:** 02 (Create Rules and allocate Identifiers)

**Status:** ready-for-agent

- [ ] `proposed → validated` and `proposed → rejected` work
- [ ] `validated → obsolete` works, with optional `superseded_by`
- [ ] Every other transition is refused with a non-zero exit and an explanatory message, leaving the Source unchanged
- [ ] Identifiers of rejected and obsolete Rules are retired and never re-allocated
- [ ] Each transition (allowed and refused) is covered by CLI tests
