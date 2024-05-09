use std::fmt::Display;

use crate::TermStyle;

use super::TermFmtStrategy;

pub struct InteractiveTermFmt;

impl TermFmtStrategy for InteractiveTermFmt {
    fn debug(&self, value: impl Display) {
        println!("\n{} {}", "DEBUG".fg_bright_cyan().bold(), value)
    }

    fn info(&self, value: impl Display) {
        println!("\n{} {}", "INFO".fg_green().bold(), value);
    }

    fn warning(&self, value: impl Display) {
        println!("\n{} {}", "WARNING".fg_yellow().bold(), value);
    }

    fn error(&self, value: impl Display) {
        eprintln!("\n{} {}", "ERROR".fg_red().bold(), value);
    }

    fn headline(&self, value: impl Display) {
        println!("\n{}", value);
    }

    fn text(&self, value: impl Display) {
        println!("{}", value);
    }

    fn action(&self, value: impl Display) {
        println!("{} {}", "->".fg_blue().bold(), value);
    }

    fn start(&self) {
        println!();
    }

    fn end(&self) {
        println!();
    }
}