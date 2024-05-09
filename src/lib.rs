pub use {
    color::TermStyle,
    data::{ArrayFmt, DataDisplay},
    output::{termarrow, termerr, terminfo, termout, TermError, TermOutput},
};

#[cfg(feature = "command")]
pub mod command;

#[cfg(feature = "layout")]
pub mod layout;

pub mod chrono;
pub mod strategies;

mod color;
mod data;
mod fmt;
mod output;
