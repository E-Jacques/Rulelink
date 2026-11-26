# End-to-end tests

Crate `rulelink-e2e` in `tests/e2e/`. It drives the built binary, as a user would.

- **One file per command**: `tests/e2e/tests/<command>.rs`. Shared helpers live in `tests/e2e/src/lib.rs` (`rulelink_bin()` builds and locates the binary).
- **Isolated**: each test works in a `tempfile` directory. Never touch the repository or `examples/`.
- **Assert what a user sees**: stdout, stderr, exit code, and that untouched files stay untouched.
- **Replay the docs**: `quickstart.rs` runs the Quickstart page. A documented example gets a test that runs it as written.
- **Portable**: CI runs on Linux, macOS and Windows. No hard-coded separators, no shell-only syntax.
- **Run**: `cargo nextest run -p rulelink-e2e` (moon task `e2e`). CI runs `cargo test -p rulelink-e2e`.
- **Not here**: logic covered by a unit test or doctest. e2e covers wiring, exit codes and files.
