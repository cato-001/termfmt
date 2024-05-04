use std::fmt::Display;

use chrono::TimeDelta;

use crate::color::TermStyle;
use crate::TermFmt;

pub struct DeltaFmt {
    value: Option<TimeDelta>,
}

impl TermFmt<DeltaFmt> for TimeDelta {
    fn termfmt(self) -> DeltaFmt {
        DeltaFmt { value: Some(self) }
    }
}

impl TermFmt<DeltaFmt> for Option<TimeDelta> {
    fn termfmt(self) -> DeltaFmt {
        DeltaFmt { value: self }
    }
}

impl Display for DeltaFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(span) = self.value else {
            return write!(f, "running ...");
        };
        let hours = span.num_hours();
        let minutes = span.num_minutes() % 60;
        let seconds = span.num_seconds() % 60;
        match (hours, minutes, seconds) {
            (0, 0, seconds) => write!(f, "{}s", seconds),
            (0, minutes, 0) => write!(f, "{}m", minutes),
            (hours, 0, _) => write!(f, "{}h", hours),
            (0, minutes, seconds) => write!(f, "{}m {}s", minutes, seconds),
            (hours, minutes, _) => write!(f, "{}h {}m", hours, minutes),
        }
    }
}

impl TermStyle for TimeDelta {}
