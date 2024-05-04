use std::fmt::Display;

use chrono::NaiveTime;

use crate::color::TermStyle;

pub struct TimeFmt {
    value: Option<NaiveTime>,
}

impl Display for TimeFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(time) = self.value else {
            return write!(f, "...");
        };
        write!(f, "{}", time.format("%H:%M"))
    }
}

impl TermStyle for TimeFmt {}
