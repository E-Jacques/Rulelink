# 08: Source-integrity Findings CM100–CM106

**What to build:** `check` detects malformed or inconsistent Source files before checking code, using the MVP source-integrity Findings CM100–CM106, including CM003 (two Rules share an Identifier, error) and CM106 (Source not in Canonical form, warning). A broken Source is reported clearly rather than producing misleading code Findings.

**Blocked by:** 05 (`rules format` and Canonical-form integrity), 07 (`check` core)

**Status:** ready-for-agent

- [ ] Each of CM100–CM106 (as defined in the checks design doc) is raised by a minimal fixture
- [ ] CM003 fires on duplicate Identifiers, e.g. after a bad merge
- [ ] CM106 fires on a non-canonical Source and disappears after `rules format`
- [ ] A broken Source stops code Findings and reports the integrity problem
- [ ] Severities and exit codes follow the same rules as 07
