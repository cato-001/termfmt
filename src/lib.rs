pub use {
    color::TermStyle,
    command::{CommandOutputError, CommandStatusError},
    fmt::{BundleFmt, DataFmt, TermFmt},
    output::{termarrow, termerr, terminfo},
};

pub mod chrono;

mod clap;
mod color;
mod command;
mod fmt;
mod output;
