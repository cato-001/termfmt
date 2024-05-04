use std::fmt::{Debug, Display};
use std::io;
use std::process::{Command, Output};

pub struct CommandFmt<'a> {
    value: Option<&'a Command>,
}

impl<'a> CommandFmt<'a> {
    pub fn new(value: &'a Command) -> Self {
        Self { value: Some(value) }
    }

    pub fn option(value: Option<&'a Command>) -> Self {
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

impl Display for OutputErrFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Ok(result) => todo!(),
            Err(error) => todo!(),
        }
    }
}
