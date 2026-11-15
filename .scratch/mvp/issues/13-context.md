# 13: `context <path>`

**What to build:** `context <path>` returns the Rules relevant to a file or area, in a form suited to agents so they consult Rules before editing. It has a human output and a machine-readable JSON output that skills can consume.

**Blocked by:** 06 (Annotation scan and `where`), 09 (`check --format json` with a published report schema)

**Status:** ready-for-agent

- [ ] For a file or directory, returns the Rules cited there with status and text
- [ ] JSON output is stable, documented and schema-validated
- [ ] A path with no Annotations returns an empty result, not an error
- [ ] Retired or proposed Rules are flagged as such in the output
- [ ] Unknown path is refused clearly
