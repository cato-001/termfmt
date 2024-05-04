use std::fmt::Display;

pub trait TermFmt<Value>: Display {
    fn termfmt(value: Value) -> Self;
}

#[cfg(feature = "chrono")]
pub mod chrono;

#[cfg(feature = "command")]
pub mod command;

pub mod color;
