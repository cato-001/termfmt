pub use {
    clap::{TermFmtExt, TermFmtsExt},
    color::TermStyle,
    command::{CommandOutputError, CommandStatusError},
    fmt::{BundleFmt, DataFmt, TermFmt},
    output::{termarrow, termerr, termh1, termh2, termh3, terminfo},
};

pub mod chrono;

mod clap;
mod color;
mod command;
mod fmt;
mod output;
