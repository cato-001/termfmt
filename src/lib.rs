use std::fmt::Display;

pub use {color::TermStyle, output::TermOutput};

pub trait TermFmt<Value>
where
    Value: Display,
{
    fn termfmt(self) -> Value;
}

#[cfg(feature = "chrono")]
pub mod chrono;

#[cfg(feature = "command")]
pub mod command;

mod color;
mod output;
