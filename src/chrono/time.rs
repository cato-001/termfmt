use std::fmt::Display;

use chrono::NaiveTime;

use crate::color::TermStyle;
use crate::TermFmt;

pub struct TimeFmt {
    value: Option<NaiveTime>,
}

impl TermFmt<NaiveTime> for TimeFmt {
    fn termfmt(value: NaiveTime) -> Self {
        let value = Some(value);
        Self { value }
    }
}

impl TermFmt<Option<NaiveTime>> for TimeFmt {
    fn termfmt(value: Option<NaiveTime>) -> Self {
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

impl TermStyle for TimeFmt {}
