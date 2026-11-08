#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeOfDay {
    hour: u8,
    minute: u8,
    second: u8,
}

impl TimeOfDay {
    pub const fn new(
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Option<Self> {
        if hour >= 24 || minute >= 60 || second >= 60 {
            return None;
        }

        Some(Self {
            hour,
            minute,
            second,
        })
    }

    pub const fn hour(self) -> u8 {
        self.hour
    }

    pub const fn minute(self) -> u8 {
        self.minute
    }

    pub const fn second(self) -> u8 {
        self.second
    }

    pub const fn seconds_since_midnight(self) -> u32 {
        (self.hour as u32 * 3_600)
            + (self.minute as u32 * 60)
            + self.second as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_time_is_constructed() {
        let time = TimeOfDay::new(13, 45, 30).unwrap();

        assert_eq!(time.seconds_since_midnight(), 49_530);
    }

    #[test]
    fn invalid_time_is_rejected() {
        assert!(TimeOfDay::new(24, 0, 0).is_none());
        assert!(TimeOfDay::new(12, 60, 0).is_none());
        assert!(TimeOfDay::new(12, 0, 60).is_none());
    }
}