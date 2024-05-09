use std::fmt::Display;

#[cfg(feature = "chrono")]
mod logger;

use clap::{Arg, ArgAction, Command};
#[cfg(feature = "chrono")]
pub use logger::{LogLevel, LoggerTermFmt};

pub use {interactive::InteractiveTermFmt, plain::PlainTermFmt};

mod interactive;
mod plain;

pub trait TermFmtStrategie {
    fn debug(&self, value: impl Display);
    fn info(&self, value: impl Display);
    fn warning(&self, value: impl Display);
    fn error(&self, value: impl Display);

    fn headline(&self, value: impl Display);
    fn text(&self, value: impl Display);
    fn action(&self, value: impl Display);
}

pub trait CommandTermStrategiesExt {
    fn term_strategies(self) -> Command;
}

impl CommandTermStrategiesExt for Command {
    fn term_strategies(self) -> Command {
        self.args([
            Arg::new("interactive")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["csv", "plain", "logger"])
                .help("force interactive output"),
            Arg::new("csv")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["interactive", "plain", "logger"])
                .help("force csv output"),
            Arg::new("plain")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["interactive", "csv", "logger"])
                .help("force plain output"),
            Arg::new("logger")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["interactive", "csv", "plain"])
                .help("force logger output"),
        ])
    }
}
