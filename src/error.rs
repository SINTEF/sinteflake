use thiserror::Error;

#[derive(Error, Debug)]
pub enum SINTEFlakeError {
    #[error("Epoch should be in the past")]
    EpochInFuture,

    #[error("Timestamp overflow")]
    TimestampOverflow,

    #[error("Counter overflow, do you remember to call update_time()?")]
    CounterOverflow,

    #[error("Mutex error")]
    MutexError,

    #[error("Instance ID too high, max 10 bits")]
    InstanceIDTooHigh,
}
