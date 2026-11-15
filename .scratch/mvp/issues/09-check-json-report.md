# 09: `check --format json` with a published report schema

**What to build:** `check --format json` emits Findings in a stable machine-readable report that agents and skills parse. Its schema is a published contract consumed by the skills.

**Blocked by:** 07 (`check` core)

**Status:** ready-for-agent

- [ ] JSON output contains every Finding with severity, code, message and location
- [ ] Output validates against a published JSON report schema, tested through the CLI
- [ ] Schema is generated and verified by the existing schema consistency task
- [ ] Exit codes match the human format
- [ ] Only `human` and `json` formats are accepted; others are refused clearly
