---
name: checkmate-retrodoc
description: Use to retro-document the business rules of a module or legacy code that has none. Infer rules, write a proposals file, import it, and wait for human validation.
---

# checkmate-retrodoc

1. Read the module and infer candidate rules.
2. Write a proposals file; `checkmate import file --dry-run`, then `checkmate import file`.
3. Never approve rules yourself: `rules approve` is reserved to the human.
4. After validation: `checkmate annotate`, `checkmate check`.
