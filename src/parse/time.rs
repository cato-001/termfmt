use std::ops::Not;

use chrono::NaiveTime;

// Valid time formats are:
// - 10:32 => 10:32
// - 1032  => 10:32
// - 9:20  => 09:20
// - 920   => 09:20
// - 12    => 12:00
// - 7     => 07:00
pub fn parse_time(input: &str) -> Option<(&str, NaiveTime)> {
    let (time, input) = input
        .find(|char| matches!(char, '0'..='9' | ':').not())
        .map(|index| input.split_at(index))
        .unwrap_or((input, ""));
    if time.len() == 0 {
        return None;
    }
    if let Some((hour, minutes)) = time.split_once(':') {
        let hour = hour.parse().ok()?;
        let minutes = minutes.parse().ok()?;
        return NaiveTime::from_hms_opt(hour, minutes, 0).map(|time| (input, time));
    }
    if time.len() > 4 {
        return None;
    }
    if time.len() <= 2 {
        let hour = time.parse().ok()?;
        return NaiveTime::from_hms_opt(hour, 0, 0).map(|time| (input, time));
    }
    let (hour, minutes) = time.split_at(time.len() - 2);
    let hour = hour.parse().ok()?;
    let minutes = minutes.parse().ok()?;
    NaiveTime::from_hms_opt(hour, minutes, 0).map(|time| (input, time))
}

#[cfg(test)]
mod tests {
    use chrono::NaiveTime;

    use crate::parse::time::parse_time;

    #[test]
    fn test_valid_inputs() {
        assert_eq!(
            parse_time("10:32text"),
            Some(("text", NaiveTime::from_hms_opt(10, 32, 0).unwrap())),
            "could not deserialize '10:32'"
        );
        assert_eq!(
            parse_time("1032text"),
            Some(("text", NaiveTime::from_hms_opt(10, 32, 0).unwrap())),
            "could not deserialize '1032'"
        );
        assert_eq!(
            parse_time("9:20text"),
            Some(("text", NaiveTime::from_hms_opt(9, 20, 0).unwrap())),
            "could not deserialize '1032'"
        );
        assert_eq!(
            parse_time("920text"),
            Some(("text", NaiveTime::from_hms_opt(9, 20, 0).unwrap())),
            "could not deserialize '920'"
        );
        assert_eq!(
            parse_time("12text"),
            Some(("text", NaiveTime::from_hms_opt(12, 0, 0).unwrap())),
            "could not deserialize '12'"
        );
        assert_eq!(
            parse_time("7text"),
            Some(("text", NaiveTime::from_hms_opt(7, 0, 0).unwrap())),
            "could not deserialize '7'"
        );
    }

    #[test]
    fn test_invalid_inputs() {
        assert_eq!(
            parse_time("12000text"),
            None,
            "could deserialize '12000', but should not"
        );
        assert_eq!(
            parse_time("1:200text"),
            None,
            "could deserialize '1:200', but should not"
        );
        assert_eq!(
            parse_time("120:0text"),
            None,
            "could deserialize '120:0', but should not"
        );
        assert_eq!(
            parse_time("text"),
            None,
            "could deserialize '', but should not"
        );
    }
}
