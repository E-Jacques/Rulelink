---
name: input-boundaries
description: Checklist for code that takes input from outside the process (CLI arguments, configuration, files). Use when planning or reviewing any command that reads user input, `checkmate.toml` or a Source file.
---

# Input boundaries

Everything that crosses into the program is untrusted: CLI arguments, `checkmate.toml`, Source files, paths. Go through this list for each input of the change, in the plan and again in the review. Each line is checked, or marked not applicable with a reason.

## Parse at the edge

- Convert the raw value into the model's type as early as possible (`Origin`, `RuleId`, `IdPrefix`). Do not carry `String` further than the first layer that can parse it.
- The parsing lives with the type (`Origin::parse`), not in `app` or `cli`, so import and other commands reuse it.

## Normalise and bound

- Trim text before storing it, and measure lengths on the stored value.
- Reject empty values in lists and optional fields, not only in required ones.
- Reject duplicates and self-references where the model forbids them.
- Arithmetic on values read from a file uses `checked_*` and returns a named error, not a panic or wraparound.

## Strict configuration

- An unknown value for a closed setting (such as `source.kind`) is an error.
- A missing section the code depends on is an error, unless a default is documented.
- Nothing is silently defaulted.

## Paths

- A path read from configuration is relative, contains no `..`, and stays inside the project.
- Distinguish "not found" from "cannot read" in the error message.

## Failure

- A failed command leaves every file as it was. Mutations to in-memory state, such as counters, happen only after validation succeeds.
- Files are written atomically (temporary file, then rename).
- A corrupted Source is reported and never overwritten.

## Errors

Every error says what is wrong, where, and what to do next. Specific beats short.

- **One type per cause.** An error type holds only the failures of one cause (`InvalidIdPrefix`, `DraftError`, not one `Error` for the whole model). A variant named `InvalidIdPrefix` never wraps a generic error of another crate.
- **No catch-all.** Do not use `#[non_exhaustive]` on the error enums of this workspace: it is for published APIs, and here it forces a `_ =>` arm in the caller that files new variants under the wrong class without a compiler warning. Match every variant by name when converting to `app::Error`.
- **No blanket `From`.** A `From<X> for app::Error` is allowed only when every variant of `X` has the same class (document it on the impl). Otherwise convert with an explicit `match`.
- **Class matches the cause**: invalid input, invalid configuration, unreadable Source, internal error. Each class has its own exit code, and "not implemented" or an unexpected I/O failure is never reported as user input.
- A refused value names the option as the user typed it, a missing or unreadable file names its path, a parse failure gives line and column when the parser has them.
- Messages are in English and tell the user what to do next.
- Errors from adapters keep structure (kind, location) when later Findings will need it, not only a `String`.

## Tests

- Each line above that applies has a test that fails without it.
- Each error class has a test that checks the exit code and that the message names the value or path at fault.
- Unit tests sit with the type; end-to-end tests cover exit codes and untouched files.
