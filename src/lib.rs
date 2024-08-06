//! # SINTEFlake
//!
//! SINTEFlake is a 64-bit ID generator inspired by Twitter's Snowflake and Sony's Sonyflake.
//! It generates identifiers that start with a hash or a pseudo-random number instead of a timestamp.
//!
//! ## Features
//!
//! - Generates 64-bit IDs with distinct values
//! - Allows custom instance IDs for distributed systems
//! - Provides hash-based ID generation
//! - Supports both synchronous and asynchronous environments
//!
//! ## Structure
//!
//! A SINTEFlake ID is composed of:
//!
//! - 14 bits for a hash or a random number
//! - 31 bits for a timestamp with an 8-second resolution
//! - 10 bits for an instance identifier
//! - 8 bits for a sequence number
//!
//! ## Usage
//!
//! ```rust
//! use sinteflake::{next_id, next_id_with_hash, set_instance_id, update_time};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   // Set the instance ID first if necessary, is 0 by default
//!   set_instance_id(42)?;
//!
//!   // Before created a batch of IDs, it's good to update the time
//!   // and you are good for 8 seconds.
//!   update_time()?;
//!
//!   let id_a = next_id()?;
//!   let id_b = next_id()?;
//!
//!   let id_c = next_id_with_hash(&[1, 2, 3])?;
//!   let id_d = next_id_with_hash(&[1, 2, 3])?;
//!
//!   Ok(())
//! }
//! ```
//!
//! ## Async Usage:
//!
//! ```toml
//! [dependencies]
//! sinteflake = { version = "0.1", features = ["async"] }
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! ```rust
//! #[cfg(feature = "async")]
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   use sinteflake::{next_id_async, next_id_with_hash_async, set_instance_id_async, update_time_async};
//!
//!   set_instance_id_async(42).await?;
//!
//!   // Before created a batch of IDs, it's good to update the time
//!   // and you are good for 8 seconds.
//!   update_time_async().await?;
//!
//!   let id = next_id_async().await?;
//!   let id = next_id_with_hash_async(&[1, 2, 3]).await?;
//!
//!   Ok(())
//! }
//! // (ignore this, it's to let the example pass the tests)
//! #[cfg(not(feature = "async"))] fn main() {}
//! ```
//!
//! ⚠️ Please note that the `set_instance_id_async` is not setting the instance ID of the non async version.
//! Similarly, `update_time_async` is not updating the time of the non async version.
//!
//! ## Custom Settings
//!
//! You can create a custom SINTEFlake instance with your own settings:
//!
//! ```rust
//! use sinteflake::sinteflake::SINTEFlake;
//! use time::OffsetDateTime;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   let mut instance = SINTEFlake::custom(
//!     42,                                                      // instance_id
//!     [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], // hash_key
//!     123,                                                     // counter hash key
//!     OffsetDateTime::from_unix_timestamp(1719792000)?,        // epoch
//!   )?;
//!
//!   let id_a = instance.next_id()?;
//!
//!   Ok(())
//! }
//! ```
//!
//! ## Note
//!
//! SINTEFlake IDs are not cryptographically secure and should not be used for security-sensitive applications.
//! For most use cases, UUIDs are recommended over SINTEFlake IDs.

pub mod bits;
pub mod error;
pub mod hash;
pub mod permute;
pub mod sinteflake;
pub mod time;

mod singleton;

#[cfg(feature = "async")]
mod tokio_singleton;

pub use singleton::*;

#[cfg(feature = "async")]
pub use tokio_singleton::*;

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
}
