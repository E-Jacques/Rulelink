# Plan 03: Rule lifecycle (`rules approve | reject | obsolete`, issue #10)

## Interface

```rust
// model
impl RuleSet {
    pub fn approve(&mut self, id: &RuleId) -> Result<(), Error>;                       // proposed -> validated
    pub fn reject(&mut self, id: &RuleId) -> Result<(), Error>;                        // proposed -> rejected, retires id
    pub fn obsolete(&mut self, id: &RuleId, superseded_by: Option<&RuleId>) -> Result<(), Error>; // validated -> obsolete, retires id
}
impl Display for RuleStatus                                                            // "proposed", ...
// app
pub fn rules::{approve, reject}(root: &Path, id: &RuleId) -> Result<(), Error>;
pub fn rules::obsolete(root: &Path, id: &RuleId, superseded_by: Option<&RuleId>) -> Result<(), Error>;
```

One sentence: each call moves one Rule along the lifecycle or fails without touching the Source.

## Misuse

- There is no `set_status(id, status)`: the caller cannot request a transition the lifecycle does not allow, nor change status without retiring the Identifier.
- Retiring is part of `reject` / `obsolete`: a caller cannot reject and forget to retire.
- Every method checks everything first, then mutates; the file is written last (atomic), so a refused command changes nothing.
- `superseded_by` can only be given to `obsolete`; it must exist and differ from `id`.

## Ownership

| Decision                                                | Owner                                  |
| ------------------------------------------------------- | -------------------------------------- |
| Allowed transitions, retiring, `superseded_by` validity | model (`RuleSet`)                      |
| Read-modify-write of the Source                         | app (`with_source`, shared with `add`) |
| Error class (refused value/transition = input, exit 2)  | app                                    |
| Arguments, printing the new status                      | cli                                    |

## Input boundaries

| Input             | Line checked                                                                                                               |
| ----------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `<id>`            | exact match with an existing Rule; otherwise `RuleNotFound` naming the value. No normalisation (case-sensitive, as stored) |
| `--superseded-by` | exists in the Source, differs from `<id>`; only on `obsolete`                                                              |
| Current status    | one named error per refusal, naming the Rule, its status and the required one                                              |
| Source / config   | as in `rules add` (same helper)                                                                                            |
| Failure           | nothing written; atomic rename                                                                                             |
| Error class       | not found / refused transition / bad replacement -> `Input` (exit 2); config 2; Source 3                                   |

## Smells

- Shotgun surgery risk: three near-identical transitions. One private `transition` helper in the model.
- Duplicate read-modify-write in app (review #8 of issue 02): extracted now, as it has four callers.
- Retired list could get duplicates if the Source was hand-edited: `retire` is idempotent.
- Wrong error class: new model errors map to `Input`, not `Source`.

## Reuse

`read_source` / `write_source`, `Config::load`, `RuleSet::retired`, `AppError::Input`, the `rules_add` handler shape, the e2e `Repo` harness.

## Dependent tickets

| Ticket                    | Reuses                                                              | Missing            |
| ------------------------- | ------------------------------------------------------------------- | ------------------ |
| #11 search/filter/related | `RuleStatus` Display, `with_source`                                 | filters (theirs)   |
| #12 format                | `with_source`                                                       | `--check` (theirs) |
| #14 check core            | status semantics (validated expected to have code), `superseded_by` | findings (theirs)  |

Known limit, inherited by #14: `--superseded-by` only has to exist; it may be a rejected, obsolete or proposed Rule, and a cycle is possible. RL002 must tolerate it. `<id>` is trimmed and refused when blank (`RuleId::parse`); hand-edited duplicates are not detected.

Deferred: `--reason`, un-rejecting, `validated -> proposed`, `--dry-run`; concurrent modification (known gap, review #6 of issue 02). Hand-edited duplicate Identifiers in the Source are not detected here (review #5); the first match wins.
