use std::io::{stdout, IsTerminal};

use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{BundleFmt, TermFmt};

pub trait TermFmtsExt {
    fn termfmts(self) -> Command;
}

impl TermFmtsExt for Command {
    fn termfmts(self) -> Command {
        self.args([
            Arg::new("plain")
                .long("plain")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["interactive", "json"])
                .help("force plain output"),
            Arg::new("interactive")
                .long("interactive")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["plain", "json"])
                .help("force interactive output"),
            Arg::new("json")
                .long("json")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["plain", "interactive"])
                .help("force json output"),
        ])
    }
}

pub trait TermFmtExt<Bundle>
where
    Bundle: BundleFmt,
{
    fn termfmt(&self, config: &Bundle::Config) -> TermFmt<Bundle>
    where
        Bundle::Config: Clone;
}

impl<Bundle> TermFmtExt<Bundle> for ArgMatches
where
    Bundle: BundleFmt,
    Bundle::Config: Clone,
{
    fn termfmt(&self, config: &Bundle::Config) -> TermFmt<Bundle>
    where
        Bundle::Config: Clone,
    {
        if self.get_flag("plain") {
            TermFmt::new_plain()
        } else if self.get_flag("interactive") {
            TermFmt::new_interactive()
        } else if self.get_flag("json") {
            TermFmt::new_json(Bundle::new(config.clone()))
        } else if is_stdout_interactive() {
            TermFmt::new_interactive()
        } else {
            TermFmt::new_plain()
        }
    }
}

fn is_stdout_interactive() -> bool {
    stdout().is_terminal()
}
