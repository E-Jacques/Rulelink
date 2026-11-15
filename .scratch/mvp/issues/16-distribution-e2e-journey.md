# 16: Distribution and cross-platform end-to-end journey

**What to build:** The MVP exit-criterion journey runs end to end and the tool ships: `init`, create or import Rules, annotate code, `check` clean, then the same `check` as a hand-written CI step. Releases are built by dist from a tag, giving the npm package (`npx checkmate`) and `cargo install`, with the same behaviour on Linux, macOS and Windows.

**Blocked by:** 07 (`check` core), 14 (JSON file Import)

**Status:** ready-for-agent

- [ ] One end-to-end test mirrors the journey above and passes via the real binary
- [ ] The same journey passes on Linux, macOS and Windows in CI
- [ ] A tag produces GitHub release archives and the npm package through dist
- [ ] `npx checkmate init` and `cargo install` both yield a working binary
- [ ] No installer, Action or workflow generation is included
