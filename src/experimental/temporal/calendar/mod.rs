pub const GREGORIAN_WEEKDAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

pub const GREGORIAN_MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub const fn is_gregorian_leap_year(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

pub const fn gregorian_days_in_month(year: u32, month: u8) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => {
            if is_gregorian_leap_year(year) {
                Some(29)
            } else {
                Some(28)
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gregorian_leap_year_rules() {
        assert!(is_gregorian_leap_year(2024));
        assert!(!is_gregorian_leap_year(1900));
        assert!(is_gregorian_leap_year(2000));
    }

    #[test]
    fn gregorian_month_lengths() {
        assert_eq!(gregorian_days_in_month(2024, 2), Some(29));
        assert_eq!(gregorian_days_in_month(2023, 2), Some(28));
        assert_eq!(gregorian_days_in_month(2024, 4), Some(30));
        assert_eq!(gregorian_days_in_month(2024, 13), None);
    }
}
