# 01: Walking skeleton: `init` and the Source

**What to build:** `npx checkmate init` in a repository creates a configuration (with the Identifier prefix, default `RULE`) and an empty Source written in Canonical form. It refuses to overwrite an existing setup. This slice also lays down the Rule model v1 (`"format": 1`), the generated JSON Schema verified in CI, the Canonical form writer, and the CLI-process test harness that runs the real binary against fixture repos in temp directories.

**Blocked by:** None (can start immediately).

**Status:** ready-for-agent

- [ ] `init` in an empty repo creates the config and an empty Source; the Source is in Canonical form
- [ ] The Identifier prefix is configurable and defaults to `RULE`
- [ ] `init` on an existing setup refuses and leaves all files untouched
- [ ] A JSON Schema for the Rule model is generated and checked in CI
- [ ] CLI tests run the real binary on a temp fixture repo and assert stdout, stderr, exit code and files
- [ ] Behaviour is identical on Linux, macOS and Windows
