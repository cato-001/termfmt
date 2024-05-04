use std::fmt::{Debug, Display};
use std::process::Command;

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
