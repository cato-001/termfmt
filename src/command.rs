use std::io;
use std::process::{ExitStatus, Output};

use eyre::eyre;

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
