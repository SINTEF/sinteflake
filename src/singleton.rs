use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::error::SINTEFlakeError;
use crate::sinteflake::SINTEFlake;

static SINTEFLAKE: Lazy<Mutex<SINTEFlake>> =
    Lazy::new(|| Mutex::new(SINTEFlake::new().expect("Failed to create SINTEFlake instance")));

/// Sets the instance ID for the global SINTEFlake instance.
/// Returns an error if the mutex is poisoned or if the ID is invalid.
pub fn set_instance_id(id: u16) -> Result<(), SINTEFlakeError> {
    let mut instance = SINTEFLAKE.lock().map_err(|_| SINTEFlakeError::MutexError)?;
    instance.set_instance_id(id)
}

/// Update the time for the global SINTEFlake instance.
/// Returns an error if the mutex is poisoned or if the time update fails.
pub fn update_time() -> Result<(), SINTEFlakeError> {
    let mut instance = SINTEFLAKE.lock().map_err(|_| SINTEFlakeError::MutexError)?;
    instance.update_time()
}

/// Generates the next unique ID using the global SINTEFlake instance.
/// Returns an error if the mutex is poisoned or if ID generation fails.
pub fn next_id() -> Result<u64, SINTEFlakeError> {
    let mut instance = SINTEFLAKE.lock().map_err(|_| SINTEFlakeError::MutexError)?;
    instance.next_id()
}

/// Generates the next unique ID with a hash using the global SINTEFlake instance.
/// Returns an error if the mutex is poisoned or if ID generation fails.
pub fn next_id_with_hash(data: &[u8]) -> Result<u64, SINTEFlakeError> {
    let mut instance = SINTEFLAKE.lock().map_err(|_| SINTEFlakeError::MutexError)?;
    instance.next_id_with_hash(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let id_a = next_id().unwrap();
        let id_b = next_id().unwrap();
        assert_ne!(id_a, id_b);
    }

    #[test]
    fn test_with_hash() {
        let data = [1, 2, 3];
        let id_a = next_id_with_hash(&data).unwrap();
        let id_b = next_id_with_hash(&data).unwrap();
        assert_ne!(id_a, id_b);
    }

    #[test]
    fn test_set_instance_id() {
        set_instance_id(42).unwrap();
        let id_a = next_id().unwrap();
        let id_b = next_id().unwrap();
        assert_ne!(id_a, id_b);
    }

    #[test]
    fn test_update_time() {
        update_time().unwrap();
        let id_a = next_id().unwrap();
        update_time().unwrap();
        let id_b = next_id().unwrap();
        assert_ne!(id_a, id_b);
    }
}
