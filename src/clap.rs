use std::io::{stdout, IsTerminal};

use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{BundleFmt, DataFmt, TermFmt};

pub trait TermStrategiesExt {
    fn term_strategies(self) -> Command;
}

impl TermStrategiesExt for Command {
    fn term_strategies(self) -> Command {
        self.args([
            Arg::new("plain")
                .long("plain")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["interactive", "json", "csv"])
                .help("force plain output"),
            Arg::new("interactive")
                .long("interactive")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["plain", "json", "csv"])
                .help("force interactive output"),
            Arg::new("json")
                .long("json")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["plain", "interactive", "csv"])
                .help("force json output"),
            Arg::new("csv")
                .long("csv")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["plain", "interactive", "json"])
                .help("force csv output"),
        ])
    }
}

pub trait TermStrategyExt<Data, Bundle>
where
    Data: DataFmt,
    Bundle: BundleFmt,
{
    fn term_strategy(&self, bundle: Bundle) -> TermFmt<Data, Bundle>;
}

impl<Data, Bundle> TermStrategyExt<Data, Bundle> for ArgMatches
where
    Data: DataFmt,
    Bundle: BundleFmt<Data = Data>,
{
    fn term_strategy(&self, bundle: Bundle) -> TermFmt<Data, Bundle> {
        if self.get_flag("plain") {
            TermFmt::plain()
        } else if self.get_flag("interactive") {
            TermFmt::interactive()
        } else if self.get_flag("json") {
            TermFmt::json(bundle)
        } else if self.get_flag("csv") {
            TermFmt::csv(bundle)
        } else if is_stdout_interactive() {
            TermFmt::interactive()
        } else {
            TermFmt::plain()
        }
    }
}

fn is_stdout_interactive() -> bool {
    stdout().is_terminal()
}
