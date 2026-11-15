# 12: `annotate` (idempotent)

**What to build:** `annotate RULE-042 <location>` inserts a correctly formatted Annotation in the right comment style for the target file's language. Repeated calls do not duplicate it, so agents can call it safely.

**Blocked by:** 06 (Annotation scan and `where`)

**Status:** ready-for-agent

- [ ] Inserts a `@rule <ID>` comment using the file's comment style
- [ ] The inserted Annotation is found by the scan and by `where`
- [ ] A second identical call leaves the file byte-identical and exits successfully
- [ ] Unknown or retired Identifiers, and unknown locations or unsupported file types, are refused clearly
