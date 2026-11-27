//! The only place the CLI writes (docs/architecture/00, section 9.4). A command says what
//! kind of message it has; where each kind goes is decided here only.

use std::fmt::Display;
use std::io::Write;

use rulelink_app::{Error, Warning};

/// Writes what a command has to say.
pub struct Presenter<'a> {
    out: &'a mut dyn Write,
    err: &'a mut dyn Write,
}

impl<'a> Presenter<'a> {
    /// A presenter writing logs to `out`, warnings and errors to `err`.
    pub fn new(out: &'a mut dyn Write, err: &'a mut dyn Write) -> Self {
        Self { out, err }
    }

    /// What a command reports, followed by a line break. A Finding is logged whatever its
    /// severity: it is the result of `check`, not a failure of the command.
    pub fn log(&mut self, message: impl Display) -> Result<(), Error> {
        writeln!(self.out, "{message}").map_err(|e| unwritable(&e))
    }

    /// Something about `rulelink.toml` that does not stop the command.
    pub fn warn(&mut self, warning: &Warning) -> Result<(), Error> {
        writeln!(self.err, "rulelink: warning: rulelink.toml: {warning}")
            .map_err(|e| unwritable(&e))
    }

    /// Why `name` failed.
    pub fn error(&mut self, name: &str, failure: &dyn Display) {
        // The exit code still tells the failure if the error output cannot be written.
        let _ = writeln!(self.err, "{name}: {failure}");
    }
}

fn unwritable(cause: &std::io::Error) -> Error {
    Error::Internal(format!("cannot write the output: {cause}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logs_and_diagnostics_go_to_their_own_output() {
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let mut presenter = Presenter::new(&mut out, &mut err);
        presenter.log("result").unwrap_or_else(|e| panic!("{e}"));
        presenter
            .warn(&Warning::new("a.b", "`a.b` is deprecated"))
            .unwrap_or_else(|e| panic!("{e}"));
        presenter.error("rulelink", &"broken");
        assert_eq!(String::from_utf8_lossy(&out), "result\n");
        assert_eq!(
            String::from_utf8_lossy(&err),
            "rulelink: warning: rulelink.toml: `a.b` is deprecated\nrulelink: broken\n"
        );
    }
}
