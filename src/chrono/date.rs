use std::fmt::Display;

use chrono::{Datelike, Days, Local, NaiveDate};

use crate::color::TermStyle;
use crate::TermFmt;

pub struct DateFmt {
    value: Option<NaiveDate>,
}

impl TermFmt<DateFmt> for NaiveDate {
    fn termfmt(self) -> DateFmt {
        DateFmt { value: Some(self) }
    }
}

impl TermFmt<DateFmt> for Option<NaiveDate> {
    fn termfmt(self) -> DateFmt {
        DateFmt { value: self }
    }
}

impl Display for DateFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(value) = self.value else {
            return write!(f, "...");
        };
        let today = Local::now().date_naive();
        if value == today {
            return write!(f, "Today");
        }
        if let Some(yesterday) = today.checked_sub_days(Days::new(1)) {
            if value == yesterday {
                return write!(f, "Yesterday");
            }
        };
        if let Some(week) = today.checked_sub_days(Days::new(7)) {
            if value > week {
                return write!(f, "{}", weekday(&value));
            }
        };
        if let Some(tomorrow) = today.checked_add_days(Days::new(1)) {
            if value == tomorrow {
                return write!(f, "Tomorrow");
            }
        };
        write!(f, "{}", value.format("%Y.%m.%d"))
    }
}

impl TermStyle for DateFmt {}

fn weekday(date: &NaiveDate) -> &'static str {
    match date.weekday() {
        chrono::Weekday::Mon => "Monday",
        chrono::Weekday::Tue => "Tuesday",
        chrono::Weekday::Wed => "Wednesday",
        chrono::Weekday::Thu => "Thursday",
        chrono::Weekday::Fri => "Friday",
        chrono::Weekday::Sat => "Saturday",
        chrono::Weekday::Sun => "Sunday",
    }
}
