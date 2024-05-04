use std::fmt::{Debug, Display};
use std::io;
use std::process::{Command, Output};

use crate::TermFmt;

pub struct CommandFmt<'a> {
    value: Option<&'a Command>,
}

impl<'a> TermFmt<&'a Command> for CommandFmt<'a> {
    fn termfmt(value: &'a Command) -> Self {
        let value = Some(value);
        Self { value }
    }
}

impl<'a> TermFmt<Option<&'a Command>> for CommandFmt<'a> {
    fn termfmt(value: Option<&'a Command>) -> Self {
        Self { value }
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

impl TermFmt<io::Result<Output>> for OutputErrFmt {
    fn termfmt(value: io::Result<Output>) -> Self {
        Self { value }
    }
}

impl Display for OutputErrFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Ok(result) => ,
            Err(error) => todo!(),
        }
    }
}
