use std::fmt::Display;
use std::io::{stderr, stdout, IsTerminal};

pub trait TermOutput {
    fn termout<InteractiveResult, UnstyledResult>(
        self,
        interactive: impl Fn(&Self) -> InteractiveResult,
        unstyled: impl Fn(&Self) -> UnstyledResult,
    ) -> Self
    where
        InteractiveResult: Display,
        UnstyledResult: Display;

    fn termout_interactive<Result>(self, interactive: impl Fn(&Self) -> Result) -> Self
    where
        Result: Display;

    fn termout_unstyled<Result>(self, unstyled: impl Fn(&Self) -> Result) -> Self
    where
        Result: Display;

    fn termerr<InteractiveResult, UnstyledResult>(
        self,
        interactive: impl Fn(&Self) -> InteractiveResult,
        unstyled: impl Fn(&Self) -> UnstyledResult,
    ) -> Self
    where
        InteractiveResult: Display,
        UnstyledResult: Display;

    fn termerr_interactive<Result>(self, interactive: impl Fn(&Self) -> Result) -> Self
    where
        Result: Display;

    fn termerr_unstyled<Result>(self, unstyled: impl Fn(&Self) -> Result) -> Self
    where
        Result: Display;
}

impl<Value> TermOutput for Value {
    fn termout<InteractiveResult, UnstyledResult>(
        self,
        interactive: impl Fn(&Self) -> InteractiveResult,
        unstyled: impl Fn(&Self) -> UnstyledResult,
    ) -> Self
    where
        InteractiveResult: Display,
        UnstyledResult: Display,
    {
        if is_stdout_interactive() {
            println!("{}", interactive(&self));
        } else {
            println!("{}", unstyled(&self))
        }
        self
    }

    fn termout_interactive<Result>(self, interactive: impl Fn(&Self) -> Result) -> Self
    where
        Result: Display,
    {
        if is_stdout_interactive() {
            println!("{}", interactive(&self));
        }
        self
    }

    fn termout_unstyled<Result>(self, unstyled: impl Fn(&Self) -> Result) -> Self
    where
        Result: Display,
    {
        if !is_stdout_interactive() {
            println!("{}", unstyled(&self));
        }
        self
    }

    fn termerr<InteractiveResult, UnstyledResult>(
        self,
        interactive: impl Fn(&Self) -> InteractiveResult,
        unstyled: impl Fn(&Self) -> UnstyledResult,
    ) -> Self
    where
        InteractiveResult: Display,
        UnstyledResult: Display,
    {
        if is_stderr_interactive() {
            eprintln!("{}", interactive(&self));
        } else {
            eprintln!("{}", unstyled(&self));
        }
        self
    }

    fn termerr_interactive<Result>(self, interactive: impl Fn(&Self) -> Result) -> Self
    where
        Result: Display,
    {
        if is_stderr_interactive() {
            eprintln!("{}", interactive(&self));
        }
        self
    }

    fn termerr_unstyled<Result>(self, unstyled: impl Fn(&Self) -> Result) -> Self
    where
        Result: Display,
    {
        if !is_stderr_interactive() {
            eprintln!("{}", unstyled(&self));
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
