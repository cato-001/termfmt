use std::fmt::Display;
use std::io::{stdout, IsTerminal};

use clap::{Arg, ArgAction, ArgMatches, Command};

pub use {
    interactive::InteractiveTermFmt,
    logger::{LogLevel, LoggerTermFmt},
    plain::PlainTermFmt,
};

mod interactive;
mod logger;
mod plain;

pub trait TermFmtStrategy {
    fn debug(&self, value: impl Display);
    fn info(&self, value: impl Display);
    fn warning(&self, value: impl Display);
    fn error(&self, value: impl Display);

    fn headline(&self, value: impl Display);
    fn text(&self, value: impl Display);
    fn action(&self, value: impl Display);

    fn start(&self) {}
    fn end(&self) {}
}

enum TermFmtStrategyImpl {
    Interactive(InteractiveTermFmt),
    Csv(),
    Plain(PlainTermFmt),
    Logger(),
}

impl TermFmtStrategyImpl {
    fn interactive() -> Self {
        Self::Interactive(InteractiveTermFmt)
    }

    fn csv() -> Self {
        Self::Csv()
    }

    fn plain() -> Self {
        Self::Plain(PlainTermFmt)
    }

    fn logger() -> Self {
        Self::Logger()
    }
}

impl TermFmtStrategy for TermFmtStrategyImpl {
    fn debug(&self, value: impl Display) {
        match self {
            Self::Interactive(strategy) => strategy.debug(value),
            Self::Csv() => todo!(),
            Self::Plain(strategy) => strategy.debug(value),
            Self::Logger() => todo!(),
        }
    }

    fn info(&self, value: impl Display) {
        match self {
            TermFmtStrategyImpl::Interactive(strategy) => strategy.info(value),
            TermFmtStrategyImpl::Csv() => todo!(),
            TermFmtStrategyImpl::Plain(strategy) => strategy.info(value),
            TermFmtStrategyImpl::Logger() => todo!(),
        }
    }

    fn warning(&self, value: impl Display) {
        match self {
            TermFmtStrategyImpl::Interactive(strategy) => strategy.warning(value),
            TermFmtStrategyImpl::Csv() => todo!(),
            TermFmtStrategyImpl::Plain(strategy) => strategy.warning(value),
            TermFmtStrategyImpl::Logger() => todo!(),
        }
    }

    fn error(&self, value: impl Display) {
        match self {
            TermFmtStrategyImpl::Interactive(strategy) => strategy.error(value),
            TermFmtStrategyImpl::Csv() => todo!(),
            TermFmtStrategyImpl::Plain(strategy) => strategy.error(value),
            TermFmtStrategyImpl::Logger() => todo!(),
        }
    }

    fn headline(&self, value: impl Display) {
        match self {
            TermFmtStrategyImpl::Interactive(strategy) => strategy.headline(value),
            TermFmtStrategyImpl::Csv() => todo!(),
            TermFmtStrategyImpl::Plain(strategy) => strategy.headline(value),
            TermFmtStrategyImpl::Logger() => todo!(),
        }
    }

    fn text(&self, value: impl Display) {
        match self {
            TermFmtStrategyImpl::Interactive(strategy) => strategy.text(value),
            TermFmtStrategyImpl::Csv() => todo!(),
            TermFmtStrategyImpl::Plain(strategy) => strategy.text(value),
            TermFmtStrategyImpl::Logger() => todo!(),
        }
    }

    fn action(&self, value: impl Display) {
        match self {
            TermFmtStrategyImpl::Interactive(strategy) => strategy.action(value),
            TermFmtStrategyImpl::Csv() => todo!(),
            TermFmtStrategyImpl::Plain(strategy) => strategy.action(value),
            TermFmtStrategyImpl::Logger() => todo!(),
        }
    }
}

pub trait TermStrategiesExt {
    fn term_strategies(self) -> Command;
}

impl TermStrategiesExt for Command {
    fn term_strategies(self) -> Command {
        self.args([
            Arg::new("interactive")
                .long("interactive")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["csv", "plain", "logger"])
                .help("force interactive output"),
            Arg::new("csv")
                .long("csv")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["interactive", "plain", "logger"])
                .help("force csv output"),
            Arg::new("plain")
                .long("plain")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["interactive", "csv", "logger"])
                .help("force plain output"),
            Arg::new("logger")
                .long("logger")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["interactive", "csv", "plain"])
                .help("force logger output"),
        ])
    }
}

pub trait TermStrategyExt {
    fn term_strategy(&self) -> impl TermFmtStrategy;
}

impl TermStrategyExt for ArgMatches {
    fn term_strategy(&self) -> impl TermFmtStrategy {
        if self.get_flag("interactive") {
            TermFmtStrategyImpl::interactive()
        } else if self.get_flag("csv") {
            TermFmtStrategyImpl::csv()
        } else if self.get_flag("plain") {
            TermFmtStrategyImpl::plain()
        } else if self.get_flag("logger") {
            TermFmtStrategyImpl::logger()
        } else if is_stdout_interactive() {
            TermFmtStrategyImpl::interactive()
        } else {
            TermFmtStrategyImpl::plain()
        }
    }
}

fn is_stdout_interactive() -> bool {
    stdout().is_terminal()
}
