# 15: Skills: `rulelink-consult` and `rulelink-retrodoc`

**What to build:** Two skills installable by hand (`npx skills add` or APM): `rulelink-consult` makes an agent read relevant Rules before changing code and annotate what it writes; `rulelink-retrodoc` makes an agent propose Rules from legacy code for human review. Skill text reserves `approve` to humans. Reference files (annotation grammar, statuses, report format) must match the real CLI.

**Blocked by:** 09 (`check --format json`), 11 (`explain`), 12 (`annotate`), 13 (`context`), 14 (JSON file Import)

**Status:** ready-for-agent

- [ ] Both skills exist with instructions that use only real, shipped commands and flags
- [ ] Reference files match actual CLI behaviour and the skills consistency task passes
- [ ] Skill text states that agents never run `approve`
- [ ] Installation works by hand with no installer
- [ ] The manual evaluation protocol (8 of 10 agent tasks) is documented; running it is not part of this ticket
