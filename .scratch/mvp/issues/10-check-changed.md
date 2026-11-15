# 10: `check --changed`

**What to build:** `check --changed` reports only Findings about files changed according to Git and the Rules they touch, keeping the agent loop fast on large repositories. It is independent of any CI platform.

**Blocked by:** 07 (`check` core)

**Status:** ready-for-agent

- [ ] On a Git fixture, only Findings for changed files (staged, unstaged and untracked) and the Rules they cite are reported
- [ ] Findings unrelated to the changed files are omitted
- [ ] Works with both `human` and `json` formats
- [ ] Clear message and behaviour outside a Git repository
- [ ] Exit code reflects only the reported Findings
