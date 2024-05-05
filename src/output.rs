use std::io::{stderr, stdout, IsTerminal};

pub trait TermOutput {
    fn termout(
        self,
        interactive: impl Fn(&Self) -> String,
        unstyled: impl Fn(&Self) -> String,
    ) -> Self;

    fn termout_interactive(self, interactive: impl Fn(&Self) -> String) -> Self;

    fn termout_unstyled(self, unstyled: impl Fn(&Self) -> String) -> Self;
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
}

pub trait TermError<Value, Error> {
    fn termerr(
        self,
        interactive: impl Fn(&Error) -> String,
        unstyled: impl Fn(&Error) -> String,
    ) -> Option<Value>;

    fn termerr_interactive(self, interactive: impl Fn(&Error) -> String) -> Option<Value>;

    fn termerr_unstyled(self, unstyled: impl Fn(&Error) -> String) -> Option<Value>;
}

impl<Value, Error> TermError<Value, Error> for Result<Value, Error> {
    fn termerr(
        self,
        interactive: impl Fn(&Error) -> String,
        unstyled: impl Fn(&Error) -> String,
    ) -> Option<Value> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                if is_stderr_interactive() {
                    eprintln!("{}", interactive(&error));
                } else {
                    eprintln!("{}", unstyled(&error));
                }
                None
            }
        }
    }

    fn termerr_interactive(self, interactive: impl Fn(&Error) -> String) -> Option<Value> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                if is_stderr_interactive() {
                    eprintln!("{}", interactive(&error));
                }
                None
            }
        }
    }

    fn termerr_unstyled(self, unstyled: impl Fn(&Error) -> String) -> Option<Value> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                if !is_stderr_interactive() {
                    eprintln!("{}", unstyled(&error));
                }
                None
            }
        }
    }
}

fn is_stdout_interactive() -> bool {
    stdout().is_terminal()
}

fn is_stderr_interactive() -> bool {
    stderr().is_terminal()
}
