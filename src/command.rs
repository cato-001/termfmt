use std::error::Error;
use std::process::{ExitStatus, Output};

use eyre::eyre;

pub trait CommandOutputError {
    fn output_error(self) -> eyre::Result<Output>;
}

pub trait CommandStatusError {
    fn status_error(self) -> eyre::Result<ExitStatus>;
}

impl<Err> CommandOutputError for Result<Output, Err>
where
    Err: Error + Send + Sync + 'static,
{
    fn output_error(self) -> eyre::Result<Output> {
        let output = self?;
        if !output.status.success() {
            return Err(eyre!("exit status {}", output.status));
        }
        Ok(output)
    }
}

impl<Err> CommandStatusError for Result<ExitStatus, Err>
where
    Err: Error + Send + Sync + 'static,
{
    fn status_error(self) -> eyre::Result<ExitStatus> {
        let status = self?;
        if !status.success() {
            return Err(eyre!("exit status {}", status));
        }
        Ok(status)
    }
}
