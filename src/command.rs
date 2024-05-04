use std::fmt::{Debug, Display};
use std::io;
use std::process::{Command, Output};

pub struct CommandFmt {
    value: Option<Command>,
}

impl CommandFmt {
    pub fn new(value: Command) -> Self {
        Self { value: Some(value) }
    }

    pub fn option(value: Option<Command>) -> Self {
        Self { value }
    }
}

impl Display for CommandFmt {
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
