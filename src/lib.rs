use std::fmt::Display;

pub use {color::TermStyle, output::TermOutput};

pub trait TermFmt<Value>: Display {
    fn termfmt(value: Value) -> Self;
}

#[cfg(feature = "chrono")]
pub mod chrono;

#[cfg(feature = "command")]
pub mod command;

mod color;
mod output;
