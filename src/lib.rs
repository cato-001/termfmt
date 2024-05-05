pub use {
    color::TermStyle,
    output::{termarrow, termerr, terminfo, termout, TermError, TermOutput},
};

#[cfg(feature = "chrono")]
pub mod chrono;

#[cfg(feature = "command")]
pub mod command;

#[cfg(feature = "layout")]
pub mod layout;

mod color;
mod output;
