use std::fmt::Display;

use chrono::{Local, NaiveTime, Timelike};

use crate::color::TermStyle;

pub struct TimeFmt {
    value: Option<NaiveTime>,
}

impl TimeFmt {
    pub fn now() -> Self {
        let now = Local::now().time();
        Self { value: Some(now) }
    }

    pub fn new(value: NaiveTime) -> Self {
        Self { value: Some(value) }
    }

    pub fn option(value: Option<NaiveTime>) -> Self {
        Self { value }
    }
}

impl Display for TimeFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(time) = self.value else {
            return write!(f, "...");
        };
        write!(f, "{}", time.format("%H:%M"))
    }
}

pub struct TimeEditFmt {
    value: NaiveTime,
}

impl TimeEditFmt {
    pub fn new(value: NaiveTime) -> Self {
        Self { value }
    }
}

impl Display for TimeEditFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.hour())?;
        let minutes = self.value.minute();
        if minutes == 0 {
            return Ok(());
        }
        if minutes < 10 {
            write!(f, "0")?;
        }
        write!(f, "{}", minutes)
    }
}

impl TermStyle for TimeFmt {}
