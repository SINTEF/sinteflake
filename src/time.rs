use time::OffsetDateTime;

use crate::error::SINTEFlakeError;

pub(crate) fn get_current_timestamp(epoch: OffsetDateTime) -> Result<u32, SINTEFlakeError> {
    let current_time = OffsetDateTime::now_utc();
    if current_time < epoch {
        return Err(SINTEFlakeError::EpochInFuture);
    }
    let duration = current_time - epoch;
    let whole_seconds = duration.whole_seconds();

    if whole_seconds > 0x3fffffff8 {
        return Err(SINTEFlakeError::TimestampOverflow);
    }

    // divide by 8 because we are interested in 8 seconds intervals,
    // the number should be max 31 bits at this point
    Ok((whole_seconds >> 3) as u32)
}
#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;

    const EPOCH_2024: i64 = 1719792000; // January 1, 2024 00:00:00 UTC
    const EPOCH_2000: i64 = 946684800; // January 1, 2000 00:00:00 UTC
    const EPOCH_2100: i64 = 4102444800; // January 1, 2100 00:00:00 UTC

    #[test]
    fn test_get_current_timestamp_with_2024_epoch() {
        let epoch = OffsetDateTime::from_unix_timestamp(EPOCH_2024).unwrap();
        let timestamp = get_current_timestamp(epoch).unwrap();
        assert!(timestamp > 0, "Timestamp should be greater than 0");
    }

    #[test]
    fn test_get_current_timestamp_with_2000_epoch() {
        let epoch = OffsetDateTime::from_unix_timestamp(EPOCH_2000).unwrap();
        let timestamp = get_current_timestamp(epoch).unwrap();
        assert!(timestamp > 0, "Timestamp should be greater than 0");
    }

    #[test]
    fn test_get_current_timestamp_with_2100_epoch() {
        let epoch = OffsetDateTime::from_unix_timestamp(EPOCH_2100).unwrap();
        let timestamp_result = get_current_timestamp(epoch);
        assert!(timestamp_result.is_err(), "Timestamp should be an error");
    }

    #[test]
    fn test_timestamp_increases_over_time() {
        let epoch = OffsetDateTime::from_unix_timestamp(EPOCH_2024).unwrap();
        let timestamp1 = get_current_timestamp(epoch).unwrap();
        //thread::sleep(Duration::from_secs(9)); // Sleep for 9 seconds
        let mut epoch = OffsetDateTime::from_unix_timestamp(EPOCH_2024).unwrap();
        epoch -= time::Duration::seconds(9);
        let timestamp2 = get_current_timestamp(epoch).unwrap();
        assert!(
            timestamp2 > timestamp1,
            "Timestamp should increase over time"
        );
    }

    #[test]
    fn test_overflow_scenario() {
        let current_time = OffsetDateTime::now_utc();
        let epoch = current_time - time::Duration::seconds(0x3fffffff8);
        // 0x7FFFFFFF * 8 = 0x3FFFFFFF8
        let timestamp = get_current_timestamp(epoch).unwrap();
        assert_eq!(
            timestamp, 2147483647,
            "Max timestamp value should be 2147483647 (31bits)"
        );

        let epoch = current_time - time::Duration::seconds(0x400000000);
        let timestamp_result = get_current_timestamp(epoch);
        assert!(timestamp_result.is_err(), "Timestamp should be an error");
    }

    #[test]
    fn test_consistency_with_different_calls() {
        let epoch = OffsetDateTime::from_unix_timestamp(EPOCH_2024).unwrap();
        let timestamp1 = get_current_timestamp(epoch).unwrap();
        let timestamp2 = get_current_timestamp(epoch).unwrap();
        assert_eq!(
            timestamp1, timestamp2,
            "Consecutive calls should return the same timestamp"
        );
    }
}
