# 07: `check` core: CM001–CM005, CM009, human report, exit codes

**What to build:** `check` combines the Source and the scan and reports Findings in a human-readable report (severity, code, message, location). Covered: CM001 (cited Identifier absent), CM002 (cites an obsolete Rule, suggesting `superseded_by`), CM004 (validated Rule cited nowhere), CM005 (cites a proposed Rule), CM009 (cites a rejected or retired Identifier). Errors exit non-zero, warnings do not by default, and chosen Findings can be raised to errors in configuration. The same `npx checkmate check` runs unchanged as a hand-written CI step.

**Blocked by:** 03 (Rule lifecycle), 06 (Annotation scan and `where`)

**Status:** ready-for-agent

- [ ] Each of CM001, CM002, CM004, CM005, CM009 is raised by a minimal fixture and not raised on a clean one
- [ ] CM002 output includes the `superseded_by` suggestion when present
- [ ] Exit code is non-zero (distinct) on any error Finding and zero with warnings only
- [ ] Configuration can raise a chosen Finding's severity to error
- [ ] Human report shows severity, code, message and location
- [ ] A CI-style invocation (`npx checkmate check` in a fresh checkout) passes and fails correctly
