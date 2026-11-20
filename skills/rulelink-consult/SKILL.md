---
name: rulelink-consult
description: Use when changing business code in a repository that contains rulelink.toml. Load the relevant rules before, respect them during, annotate and check after.
---

# rulelink-consult

1. Before: `rulelink context`, then `rulelink rules show <id>` for the relevant rules.
2. During: respect the validated rules.
3. After: `rulelink annotate`, `rulelink check --changed`, then summarise the rules applied.
