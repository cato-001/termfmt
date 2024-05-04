use std::fmt::{self, Display, Formatter};
use std::io::{stderr, stdout, IsTerminal};

pub trait TermOutput {
    fn stdout(
        self,
        interactive: impl Fn(Self, &mut Formatter) -> fmt::Result,
        unstyled: impl Fn(Self, &mut Formatter) -> fmt::Result,
    ) -> Self;

    fn stdout_interactive(self, interactive: impl Fn(Self, &mut Formatter) -> fmt::Result) -> Self;

    fn stdout_unstyled(self, unstyled: impl Fn(Self, &mut Formatter) -> fmt::Result) -> Self;
}

struct FnFmt<Function, Value> {
    function: Function,
    value: Value,
}

impl<Function, Value> FnFmt<Function, Value>
where
    Function: Fn(Value, &mut Formatter) -> fmt::Result,
    Value: Copy,
{
    pub fn new(function: Function, value: Value) -> Self {
        Self { function, value }
    }
}

impl<Function, Value> Display for FnFmt<Function, Value>
where
    Function: Fn(Value, &mut Formatter) -> fmt::Result,
    Value: Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (self.function)(self.value, f)
    }
}

impl<Value> TermOutput for Value
where
    Value: Copy,
{
    fn stdout(
        self,
        interactive: impl Fn(Self, &mut Formatter) -> fmt::Result,
        unstyled: impl Fn(Self, &mut Formatter) -> fmt::Result,
    ) -> Self {
        if is_stdout_interactive() {
            println!("{}", FnFmt::new(interactive, self));
        } else {
            println!("{}", FnFmt::new(unstyled, self));
        }
        self
    }

    fn stdout_interactive(self, interactive: impl Fn(Self, &mut Formatter) -> fmt::Result) -> Self {
        if is_stdout_interactive() {
            println!("{}", FnFmt::new(interactive, self));
        }
        self
    }

    fn stdout_unstyled(self, unstyled: impl Fn(Self, &mut Formatter) -> fmt::Result) -> Self {
        if !is_stdout_interactive() {
            println!("{}", FnFmt::new(unstyled, self));
        }
        self
    }
}

fn is_stdout_interactive() -> bool {
    stdout().is_terminal()
}

fn is_stderr_interactive() -> bool {
    stderr().is_terminal()
}
