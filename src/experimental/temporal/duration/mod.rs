pub const SECONDS_PER_MINUTE: u64 = 60;
pub const MINUTES_PER_HOUR: u64 = 60;
pub const HOURS_PER_DAY: u64 = 24;

pub const SECONDS_PER_HOUR: u64 =
    SECONDS_PER_MINUTE * MINUTES_PER_HOUR;

pub const SECONDS_PER_STANDARD_DAY: u64 =
    SECONDS_PER_HOUR * HOURS_PER_DAY;

pub const fn whole_minutes(seconds: u64) -> u64 {
    seconds / SECONDS_PER_MINUTE
}

pub const fn whole_hours(seconds: u64) -> u64 {
    seconds / SECONDS_PER_HOUR
}

pub const fn whole_standard_days(seconds: u64) -> u64 {
    seconds / SECONDS_PER_STANDARD_DAY
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_constants_are_consistent() {
        assert_eq!(SECONDS_PER_HOUR, 3_600);
        assert_eq!(SECONDS_PER_STANDARD_DAY, 86_400);
    }

    #[test]
    fn seconds_convert_to_whole_units() {
        assert_eq!(whole_minutes(120), 2);
        assert_eq!(whole_hours(7_200), 2);
        assert_eq!(whole_standard_days(172_800), 2);
    }

    #[test]
    fn conversions_discard_remainders() {
        assert_eq!(whole_minutes(125), 2);
        assert_eq!(whole_hours(7_299), 2);
    }
}