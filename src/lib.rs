use std::collections::HashMap;

use ::clap::{ArgMatches, Command};
pub use {
    clap::{TermFmtExt, TermFmtsExt},
    color::{Bg, Fg, Style, TermStyle},
    command::{CommandOutputError, CommandStatusError, TermCommandDefinition},
    fmt::{BundleFmt, DataFmt, TermFmt},
    output::{
        termarrow, termarrow_fg, termerr, termh1, termh2, termh3, termh_fg, terminfo, termprefix1,
        termprefix2, termprefix3,
    },
};

pub mod chrono;

mod clap;
mod color;
mod command;
mod fmt;
mod output;

pub struct TermCommand {
    command: Command,
    definitions: HashMap<String, Box<dyn TermCommandDefinition>>,
}

impl TermCommand {
    pub fn new() -> Self {
        let command = Command::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"));
        Self {
            command,
            definitions: HashMap::new(),
        }
    }

    pub fn configure(mut self, callback: impl Fn(Command) -> Command) -> Self {
        self.command = callback(self.command);
        self
    }

    pub fn add(mut self, definition: impl TermCommandDefinition + 'static) -> Self {
        let command = definition.clap();
        self.command = self.command.clone().subcommand(command.clone());
        self.definitions
            .insert(command.get_name().to_owned(), Box::new(definition));
        self
    }

    pub fn run(self) {
        _ = (self.command.get_matches(),);
    }
}
