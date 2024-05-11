pub use {
    color::TermStyle,
    fmt::{BundleFmt, DataFmt, TermFmt},
    output::{termarrow, termerr, terminfo},
};

#[cfg(feature = "command")]
pub mod command;

pub mod chrono;

mod clap;
mod color;
mod fmt;
mod output;
