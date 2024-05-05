use crate::TermStyle;
use std::fmt::Display;

pub use lines::Lines;

mod lines;

pub struct UppercaseFmt<'a> {
    value: &'a str,
}

impl<'a> UppercaseFmt<'a> {
    pub fn new(value: &'a str) -> Self {
        Self { value }
    }
}

impl<'a> Display for UppercaseFmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for char in self.value.chars() {
            write!(f, "{}", char.to_ascii_uppercase())?;
        }
        Ok(())
    }
}

impl<'a> TermStyle for UppercaseFmt<'a> {}

pub struct ArrowFmt<Value> {
    value: Value,
}

impl<Value> ArrowFmt<Value>
where
    Value: Display,
{
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

impl<Value> Display for ArrowFmt<Value>
where
    Value: TermStyle + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-> {}", self.value.clone().reset_style())
    }
}

impl<Value: TermStyle + Clone> TermStyle for ArrowFmt<Value> {}
