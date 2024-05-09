use std::fmt::Display;

use crate::chrono::{DateFmt, TimeFmt};

use super::TermFmtStrategy;

#[derive(Default)]
pub struct LoggerTermFmt {
    level: LogLevel,
}

impl LoggerTermFmt {
    pub fn new(level: LogLevel) -> Self {
        Self { level }
    }
}

#[derive(Default)]
pub enum LogLevel {
    #[default]
    Debug,
    Info,
    Warning,
    Error,
}

impl TermFmtStrategy for LoggerTermFmt {
    fn debug(&self, value: impl Display) {
        if matches!(self.level, LogLevel::Debug) {
            let date = DateFmt::today();
            let time = TimeFmt::now();
            println!("DEBUG [{} {}]: {}", date, time, value);
        }
    }

    fn info(&self, value: impl Display) {
        if matches!(self.level, LogLevel::Debug | LogLevel::Info) {
            let date = DateFmt::today();
            let time = TimeFmt::now();
            println!("DEBUG [{} {}]: {}", date, time, value);
        }
    }

    fn warning(&self, value: impl Display) {
        if matches!(
            self.level,
            LogLevel::Debug | LogLevel::Info | LogLevel::Warning
        ) {
            let date = DateFmt::today();
            let time = TimeFmt::now();
            println!("DEBUG [{} {}]: {}", date, time, value);
        }
    }

    fn error(&self, value: impl Display) {
        if matches!(
            self.level,
            LogLevel::Debug | LogLevel::Info | LogLevel::Warning | LogLevel::Error
        ) {
            let date = DateFmt::today();
            let time = TimeFmt::now();
            println!("DEBUG [{} {}]: {}", date, time, value);
        }
    }

    fn headline(&self, _value: impl Display) {}

    fn text(&self, value: impl Display) {
        let date = DateFmt::today();
        let time = TimeFmt::now();
        println!("HINT [{} {}]: {}", date, time, value);
    }

    fn action(&self, value: impl Display) {
        let date = DateFmt::today();
        let time = TimeFmt::now();
        println!("ACTION [{} {}]: {}", date, time, value);
    }
}
