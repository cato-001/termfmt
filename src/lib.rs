pub use {
    clap::{TermFmtExt, TermFmtsExt},
    color::{Bg, Fg, Style, TermStyle},
    command::{CommandOutputError, CommandStatusError},
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
