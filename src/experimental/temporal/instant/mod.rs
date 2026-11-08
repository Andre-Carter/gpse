use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub fn unix_timestamp_seconds(
    instant: SystemTime,
) -> Result<u64, SystemTimeError> {
    let elapsed = instant.duration_since(UNIX_EPOCH)?;

    Ok(elapsed.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_epoch_has_zero_timestamp() {
        let timestamp =
            unix_timestamp_seconds(UNIX_EPOCH).unwrap();

        assert_eq!(timestamp, 0);
    }

    #[test]
    fn timestamp_counts_whole_seconds() {
        let instant = UNIX_EPOCH
            + std::time::Duration::from_secs(1_000);

        assert_eq!(
            unix_timestamp_seconds(instant).unwrap(),
            1_000
        );
    }
}