pub use {color::TermStyle, output::TermOutput};

#[cfg(feature = "chrono")]
pub mod chrono;

#[cfg(feature = "command")]
pub mod command;

mod color;
mod output;
