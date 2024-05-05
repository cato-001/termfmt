use std::io::{stderr, stdout, IsTerminal};

pub trait TermOutput {
    fn termout(
        self,
        interactive: impl Fn(&Self) -> String,
        unstyled: impl Fn(&Self) -> String,
    ) -> Self;

    fn termout_interactive(self, interactive: impl Fn(&Self) -> String) -> Self;

    fn termout_unstyled(self, unstyled: impl Fn(&Self) -> String) -> Self;

    fn termerr(
        self,
        interactive: impl Fn(&Self) -> String,
        unstyled: impl Fn(&Self) -> String,
    ) -> Self;

    fn termerr_interactive(self, interactive: impl Fn(&Self) -> String) -> Self;

    fn termerr_unstyled(self, unstyled: impl Fn(&Self) -> String) -> Self;
}

impl<Value> TermOutput for Value {
    fn termout(
        self,
        interactive: impl Fn(&Self) -> String,
        unstyled: impl Fn(&Self) -> String,
    ) -> Self {
        if is_stdout_interactive() {
            println!("{}", interactive(&self));
        } else {
            println!("{}", unstyled(&self))
        }
        self
    }

    fn termout_interactive(self, interactive: impl Fn(&Self) -> String) -> Self {
        if is_stdout_interactive() {
            println!("{}", interactive(&self));
        }
        self
    }

    fn termout_unstyled(self, unstyled: impl Fn(&Self) -> String) -> Self {
        if !is_stdout_interactive() {
            println!("{}", unstyled(&self));
        }
        self
    }

    fn termerr(
        self,
        interactive: impl Fn(&Self) -> String,
        unstyled: impl Fn(&Self) -> String,
    ) -> Self {
        if is_stderr_interactive() {
            eprintln!("{}", interactive(&self));
        } else {
            eprintln!("{}", unstyled(&self));
        }
        self
    }

    fn termerr_interactive(self, interactive: impl Fn(&Self) -> String) -> Self {
        if is_stderr_interactive() {
            eprintln!("{}", interactive(&self));
        }
        self
    }

    fn termerr_unstyled(self, unstyled: impl Fn(&Self) -> String) -> Self {
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
