# Quality rubric for implementation runs

Used to compare two implementations of the same ticket, and to test changes to the skills (`strategic-programming`, `input-boundaries`, `cli-implement`). Score each line yes / no / partly, and keep the evidence.

## Interface design

- One operation hides the order of steps (no temporal coupling).
- A failure changes nothing, including in-memory counters.
- The application layer is thin; parsing and rules live in the model.
- Inputs are typed at the edge (`Origin`, `RuleId`), not `String`.

## Boundaries

- Paths from configuration cannot leave the project.
- Closed settings reject unknown values; required sections are required.
- Text is trimmed before storing; lengths are measured on the stored value.
- Arithmetic on file values uses `checked_*`.
- Duplicates, self-references and reused Identifiers are refused.

## Errors

- Invalid input, invalid configuration and unreadable Source are separate classes with separate exit codes.
- A corrupted Source is never overwritten.

## Readability

- Every literal that carries a domain rule has a name (`NEGATION`, `TRAILING_PUNCTUATION`), not an inline `'!'` or a string list.
- Every guard that is not obvious says why in one line (`NUL` marks a binary file).
- A heuristic documents its known limits next to the code and in the architecture note.
- No function needs scrolling to read; `clippy.toml` fails the build above 40 lines.

## Hygiene

- Dependencies are declared in the workspace (`cargo xtask arch` passes).
- Unit tests sit with the types; end-to-end tests cover exit codes and untouched files.
- The commit and PR titles match what the diff contains.
- A change to the grammar or to a behaviour of the domain updates `GLOSSARY.md` and the architecture note it contradicts.
- Deferred findings name the dependent tickets they affect.

## How to use it

Run the same ticket with the old and the new skills, score both, and keep the skill change only if the score improves. Last calibration: ticket 02, branch `feature/02-create-rules-identifiers` against `origin/claude/implement-cli-issue-2-ay0jjo`.
