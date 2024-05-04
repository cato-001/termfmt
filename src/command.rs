use std::fmt::{Debug, Display};
use std::io;
use std::process::{Command, Output};

use crate::TermFmt;

pub struct CommandFmt<'a> {
    value: Option<&'a Command>,
}

impl<'a> TermFmt<CommandFmt<'a>> for &'a Command {
    fn termfmt(self) -> CommandFmt<'a> {
        CommandFmt { value: Some(self) }
    }
}

impl<'a> TermFmt<CommandFmt<'a>> for Option<&'a Command> {
    fn termfmt(self) -> CommandFmt<'a> {
        CommandFmt { value: self }
    }
}

impl<'a> Display for CommandFmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(command) = self.value else {
            return write!(f, "...");
        };
        command.fmt(f)
    }
}

pub struct OutputErrFmt {
    value: io::Result<Output>,
}

impl TermFmt<OutputErrFmt> for io::Result<Output> {
    fn termfmt(self) -> OutputErrFmt {
        OutputErrFmt { value: self }
    }
}

impl Display for OutputErrFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Ok(result) => todo!(),
            Err(error) => todo!(),
        }
    }
}
