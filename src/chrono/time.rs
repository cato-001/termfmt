use std::fmt::Display;

use chrono::NaiveTime;

use crate::color::TermStyle;
use crate::TermFmt;

pub struct TimeFmt {
    value: Option<NaiveTime>,
}

impl TermFmt<TimeFmt> for NaiveTime {
    fn termfmt(self) -> TimeFmt {
        TimeFmt { value: Some(self) }
    }
}

impl TermFmt<TimeFmt> for Option<NaiveTime> {
    fn termfmt(self) -> TimeFmt {
        TimeFmt { value: self }
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

impl TermStyle for TimeFmt {}
