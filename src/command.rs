use std::fmt::{Debug, Display};
use std::io;
use std::process::{Command, ExitStatus, Output};

use eyre::eyre;

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
        let Some(ref command) = self.value else {
            return write!(f, "...");
        };
        command.fmt(f)
    }
}

pub trait CmdOutErr {
    fn cmd_out_err(self) -> eyre::Result<Output>;
}

pub trait CmdStatErr {
    fn cmd_stat_err(self) -> eyre::Result<ExitStatus>;
}

impl CmdOutErr for io::Result<Output> {
    fn cmd_out_err(self) -> eyre::Result<Output> {
        let output = self?;
        if !output.status.success() {
            return Err(eyre!("exit status {}", output.status));
        }
        Ok(output)
    }
}

impl CmdStatErr for io::Result<ExitStatus> {
    fn cmd_stat_err(self) -> eyre::Result<ExitStatus> {
        let status = self?;
        if !status.success() {
            return Err(eyre!("exit status {}", status));
        }
        Ok(status)
    }
}
