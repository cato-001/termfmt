use std::fmt::{self, Display, Formatter};
use std::io::{stderr, stdout, IsTerminal};

pub trait TermOutput {
    fn termout(
        self,
        interactive: impl Fn(Self, &mut Formatter) -> fmt::Result,
        unstyled: impl Fn(Self, &mut Formatter) -> fmt::Result,
    ) -> Self;

    fn termout_interactive(self, interactive: impl Fn(Self, &mut Formatter) -> fmt::Result)
        -> Self;

    fn termout_unstyled(self, unstyled: impl Fn(Self, &mut Formatter) -> fmt::Result) -> Self;

    fn termerr(
        self,
        interactive: impl Fn(Self, &mut Formatter) -> fmt::Result,
        unstyled: impl Fn(Self, &mut Formatter) -> fmt::Result,
    ) -> Self;

    fn termerr_interactive(self, interactive: impl Fn(Self, &mut Formatter) -> fmt::Result)
        -> Self;

    fn termerr_unstyled(self, unstyled: impl Fn(Self, &mut Formatter) -> fmt::Result) -> Self;
}

impl<Value> TermOutput for Value
where
    Value: Copy,
{
    fn termout(
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

    fn termout_interactive(
        self,
        interactive: impl Fn(Self, &mut Formatter) -> fmt::Result,
    ) -> Self {
        if is_stdout_interactive() {
            println!("{}", FnFmt::new(interactive, self));
        }
        self
    }

    fn termout_unstyled(self, unstyled: impl Fn(Self, &mut Formatter) -> fmt::Result) -> Self {
        if !is_stdout_interactive() {
            println!("{}", FnFmt::new(unstyled, self));
        }
        self
    }

    fn termerr(
        self,
        interactive: impl Fn(Self, &mut Formatter) -> fmt::Result,
        unstyled: impl Fn(Self, &mut Formatter) -> fmt::Result,
    ) -> Self {
        if is_stderr_interactive() {
            eprintln!("{}", FnFmt::new(interactive, self));
        } else {
            eprintln!("{}", FnFmt::new(unstyled, self));
        }
        self
    }

    fn termerr_interactive(
        self,
        interactive: impl Fn(Self, &mut Formatter) -> fmt::Result,
    ) -> Self {
        if is_stderr_interactive() {
            eprintln!("{}", FnFmt::new(interactive, self));
        }
        self
    }

    fn termerr_unstyled(self, unstyled: impl Fn(Self, &mut Formatter) -> fmt::Result) -> Self {
        if !is_stderr_interactive() {
            eprintln!("{}", FnFmt::new(unstyled, self));
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
