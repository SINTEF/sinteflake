use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::error::SINTEFlakeError;
use crate::sinteflake::SINTEFlake;

static SINTEFLAKE: Lazy<Mutex<SINTEFlake>> =
    Lazy::new(|| Mutex::new(SINTEFlake::new().expect("Failed to create SINTEFlake instance")));

/// Sets the instance ID for the global async SINTEFlake instance.
/// Returns an error if the ID is invalid.
pub async fn set_instance_id_async(id: u16) -> Result<(), SINTEFlakeError> {
    let mut instance = SINTEFLAKE.lock().await;
    instance.set_instance_id(id)
}

/// Update the time for the global async SINTEFlake instance.
/// Returns an error if the time update fails.
pub async fn update_time_async() -> Result<(), SINTEFlakeError> {
    let mut instance = SINTEFLAKE.lock().await;
    instance.update_time()
}

/// Generates the next unique ID using the global async SINTEFlake instance.
/// Returns an error if ID generation fails.
pub async fn next_id_async() -> Result<u64, SINTEFlakeError> {
    let mut instance = SINTEFLAKE.lock().await;
    instance.next_id()
}

/// Generates the next unique ID with a hash using the global async SINTEFlake instance.
/// Returns an error if ID generation fails.
pub async fn next_id_with_hash_async(data: &[u8]) -> Result<u64, SINTEFlakeError> {
    let mut instance = SINTEFLAKE.lock().await;
    instance.next_id_with_hash(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic() {
        let id_a = next_id_async().await.unwrap();
        let id_b = next_id_async().await.unwrap();
        assert_ne!(id_a, id_b);
    }

    #[tokio::test]
    async fn test_with_hash() {
        let data = [1, 2, 3];
        let id_a = next_id_with_hash_async(&data).await.unwrap();
        let id_b = next_id_with_hash_async(&data).await.unwrap();
        assert_ne!(id_a, id_b);
    }

    #[tokio::test]
    async fn test_set_instance_id() {
        set_instance_id_async(42).await.unwrap();
        let id_a = next_id_async().await.unwrap();
        let id_b = next_id_async().await.unwrap();
        assert_ne!(id_a, id_b);
    }

    #[tokio::test]
    async fn test_update_time() {
        update_time_async().await.unwrap();
        let id_a = next_id_async().await.unwrap();
        update_time_async().await.unwrap();
        let id_b = next_id_async().await.unwrap();
        assert_ne!(id_a, id_b);
    }
}
