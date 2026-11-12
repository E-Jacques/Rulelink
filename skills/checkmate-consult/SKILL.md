---
name: checkmate-consult
description: Use when changing business code in a repository that contains checkmate.toml. Load the relevant rules before, respect them during, annotate and check after.
---

# checkmate-consult

1. Before: `checkmate context`, then `checkmate rules show <id>` for the relevant rules.
2. During: respect the validated rules.
3. After: `checkmate annotate`, `checkmate check --changed`, then summarise the rules applied.
