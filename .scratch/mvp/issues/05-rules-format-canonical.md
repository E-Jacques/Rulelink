# 05: `rules format` and Canonical-form integrity

**What to build:** `rules format` rewrites a Source file into Canonical form (one property or array element per line, natural Identifier order, schema key order, no alignment), so a developer can normalise after resolving a merge conflict. Two people editing different Rules must produce small diffs that merge cleanly in Git.

**Blocked by:** 02 (Create Rules and allocate Identifiers)

**Status:** ready-for-agent

- [ ] `rules format` is idempotent (second run changes nothing)
- [ ] A hand-mangled but valid Source is rewritten to the exact Canonical form
- [ ] Identifiers are ordered naturally (`RULE-2` before `RULE-10`)
- [ ] A Git fixture with two independent edits to different Rules merges without conflict
- [ ] Round-trip (read, then write) of a canonical file is byte-identical
