use std::fmt::Display;

use crate::TermStyle;

pub fn terminfo(value: impl Display) {
    println!("\n{} {}", "INFO".fg_green().bold(), value);
}

pub fn termerr(value: impl Display) {
    eprintln!("\n{} {}", "ERROR".fg_red().bold(), value);
}

pub fn termarrow(value: impl Display) {
    println!("{} {}", "->".fg_blue().bold(), value);
}

pub fn termheader(value: impl TermStyle) {
    println!("\n{}", value.fg_green().bold());
}
