# Library

Reusable crates: `model`, `registry`, `link`, `check`, `import`, `config`, `adapter-sdk`.

- **Doc**: rustdoc on every public item, with an example. Update `docs/public/developers/crates.md` if the crate list changes.
- **Role**: the public API is the contract. Keep it small and typed. Hide steps behind one deep operation.
- **Feedback loop**: the doc example as a doctest, then unit tests next to the types.
- **Extra gates**: `moon run :api-docs` (warnings denied). Mark a breaking change in the commit footer.
- **Check**: a new public item has a caller. No caller, no export.
- **Commit scope**: the crate name (`model`, `link`, …).
