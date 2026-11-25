//! Maintainer automation: `cargo xtask <arch|schemas|skills-lint|docs-check>`.

mod arch;
mod docs;
mod schemas;
mod skills;

use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let result = match args.next().as_deref() {
        Some("arch") => arch::run(),
        Some("schemas") => schemas::run(args.next().as_deref() == Some("--check")),
        Some("skills-lint") => skills::run(),
        Some("docs-check") => docs::run(),
        _ => Err("usage: cargo xtask <arch|schemas [--check]|skills-lint|docs-check>".to_string()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("xtask: {msg}");
            ExitCode::FAILURE
        }
    }
}
