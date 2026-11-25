# The MVP ships shell and PowerShell installers next to npm

[ADR-0001](0001-mvp-scope-cut.md) limited the MVP to one install channel, npm, and moved the shell and PowerShell installers to V1. This ADR brings them back into the MVP. The release pipeline now publishes `rulelink-cli-installer.sh` and `rulelink-cli-installer.ps1` on each GitHub Release, built by dist, in addition to the archives and the npm package.

The public documentation promises a one-line install per operating system. Requiring Node to install a Rust binary is a barrier for Linux, macOS and Windows users who have no Node toolchain. dist already builds the archives, so the two installers are two lines of configuration (`installers = ["shell", "powershell", "npm"]`) and no new workflow code: the cost that justified the cut in ADR-0001 no longer applies.

## Considered options

- Keep npm only (ADR-0001): rejected, it makes the installation page depend on Node.
- Add Homebrew and apt too: rejected, each needs a maintained tap or repository; they stay out of scope.

## Consequences

- The shell and PowerShell installers move from V1 to the MVP in the release lifecycle documentation. Docker, Homebrew, Scoop and winget stay in V1.
- The `smoke-test` workflow checks that both installers are present, parse and name the archives built in the same run. It cannot run them end to end before the GitHub Release exists.
- `rulelink self update` for script-installed binaries stays in V1.
