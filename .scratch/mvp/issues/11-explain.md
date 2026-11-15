# 11: `explain`

**What to build:** `explain RULE-042` shows the Rule (status, text, related, `superseded_by`) and every place in code that cites it, so a developer understands what the code is for. It behaves clearly for unknown, retired and uncited Identifiers.

**Blocked by:** 06 (Annotation scan and `where`)

**Status:** ready-for-agent

- [ ] Known Rule: shows its fields and all citing locations
- [ ] Unknown Identifier: clear "not found" answer and non-zero exit
- [ ] Retired Identifier: states it is retired (rejected or obsolete) and any `superseded_by`
- [ ] Uncited Rule: states that no code cites it
