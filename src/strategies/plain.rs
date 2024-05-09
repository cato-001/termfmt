use std::fmt::Display;

use super::TermFmtStrategie;

pub struct PlainTermFmt;

impl TermFmtStrategie for PlainTermFmt {
    fn debug(&self, value: impl Display) {
        println!("{}", value);
    }

    fn info(&self, value: impl Display) {
        println!("{}", value);
    }

    fn warning(&self, value: impl Display) {
        println!("{}", value);
    }

    fn error(&self, value: impl Display) {
        println!("{}", value);
    }

    fn headline(&self, value: impl Display) {
        println!("{}", value);
    }

    fn text(&self, value: impl Display) {
        println!("{}", value);
    }

    fn action(&self, value: impl Display) {
        println!("{}", value);
    }
}
