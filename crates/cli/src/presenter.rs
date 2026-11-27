//! The only place the CLI writes: results on the output, warnings and errors on the error
//! output (docs/architecture/00, section 9.4).

use std::fmt::Display;
use std::io::Write;

use rulelink_app::{Error, Warning};

/// Writes what a command has to say.
pub struct Presenter<'a> {
    out: &'a mut dyn Write,
    err: &'a mut dyn Write,
}

impl<'a> Presenter<'a> {
    /// A presenter writing results to `out`, everything else to `err`.
    pub fn new(out: &'a mut dyn Write, err: &'a mut dyn Write) -> Self {
        Self { out, err }
    }

    /// One line of a result.
    pub fn line(&mut self, line: impl Display) -> Result<(), Error> {
        writeln!(self.out, "{line}").map_err(|e| unwritable(&e))
    }

    /// A result that already ends its lines.
    pub fn text(&mut self, text: &str) -> Result<(), Error> {
        write!(self.out, "{text}").map_err(|e| unwritable(&e))
    }

    /// Something about `rulelink.toml` that does not stop the command.
    pub fn configuration_warning(&mut self, warning: &Warning) -> Result<(), Error> {
        writeln!(self.err, "rulelink: warning: rulelink.toml: {warning}")
            .map_err(|e| unwritable(&e))
    }

    /// Why `name` failed.
    pub fn failure(&mut self, name: &str, failure: &dyn Display) {
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
    fn results_and_diagnostics_go_to_their_own_output() {
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let mut presenter = Presenter::new(&mut out, &mut err);
        presenter.line("result").unwrap_or_else(|e| panic!("{e}"));
        presenter
            .configuration_warning(&Warning::new("a.b", "`a.b` is deprecated"))
            .unwrap_or_else(|e| panic!("{e}"));
        presenter.failure("rulelink", &"broken");
        assert_eq!(String::from_utf8_lossy(&out), "result\n");
        assert_eq!(
            String::from_utf8_lossy(&err),
            "rulelink: warning: rulelink.toml: `a.b` is deprecated\nrulelink: broken\n"
        );
    }
}
