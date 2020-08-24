fn format_duration(seconds: u64) -> String {
    const SECOND: u64 = 1;
    const MINUTE: u64 = 60 * SECOND;
    const HOUR: u64 = MINUTE * 60;
    const DAY: u64 = HOUR * 24;
    const YEAR: u64 = DAY * 365;
    let units = [
        ("year", YEAR),
        ("day", DAY),
        ("hour", HOUR),
        ("minute", MINUTE),
        ("second", SECOND),
    ];
    if seconds == 0 {
        return String::from("now");
    }
    let mut remainder = seconds;
    let mut message = vec![];
    for (name, time) in units.iter() {
        let units = remainder / time;
        match remainder / time {
            0 => (),
            1 => message.push(format!("1 {}", name)),
            n => message.push(format!("{} {}s", n, name)),
        }
        remainder -= time * units;
    }

    message
        .join(" and ")
        .replacen(" and ", ", ", message.len().saturating_sub(2))
    // Complete this function
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    }
}
