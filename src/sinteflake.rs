use crate::bits::construct_identifier;
use crate::error::SINTEFlakeError;
use crate::hash;
use crate::permute::{permute_u32_31_bits, permute_u8};
use crate::time::get_current_timestamp;
use ::time::OffsetDateTime;

/// SINTEFlake is a 64-bit ID generator inspired by Twitter's Snowflake and Sony's Sonyflake.
/// It generates unique identifiers that start with a hash or a pseudo-random number instead of a timestamp.
pub struct SINTEFlake {
    instance_id: u16,

    hash_key: [u8; 16],

    counter_key: u8,

    epoch: OffsetDateTime,

    collisions_map: [u16; 16384], // 2^14

    current_timestamp_bits: u32,

    ids_count_at_current_timestamp: u64,
}

impl SINTEFlake {
    /// Creates a new SINTEFlake instance with default settings.
    ///
    /// # Returns
    /// - `Result<Self, SINTEFlakeError>`: A new SINTEFlake instance or an error if creation fails.
    ///
    /// # Errors
    /// Returns an error if the initial time update fails.
    pub fn new() -> Result<Self, SINTEFlakeError> {
        let mut instance = SINTEFlake {
            instance_id: 0,

            // hash_key: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            // pi digits after the comma in base 16
            // https://www.wolframalpha.com/input?i=pi+in+base+16
            // 3.243f6a8885a308d313198a2e03707344
            hash_key: [
                0x24, 0x3f, 0x6a, 0x88, 0x85, 0xa3, 0x08, 0xd3, 0x13, 0x19, 0x8a, 0x2e, 0x03, 0x70,
                0x73, 0x44,
            ],

            // the counter is XORed with this value
            counter_key: 42,

            // 2024-07-01T00:00:00Z
            epoch: OffsetDateTime::from_unix_timestamp(1719792000)
                .expect("Invalid timestamp, shouldn't happen #1719792000"),

            collisions_map: [0; 16384],

            current_timestamp_bits: 0,

            ids_count_at_current_timestamp: 0,
        };

        instance.update_time()?;

        Ok(instance)
    }

    /// Creates a custom SINTEFlake instance with specified settings.
    ///
    /// # Arguments
    /// * `instance_id` - A 14-bit unsigned integer representing the instance ID.
    /// * `hash_key` - A 16-byte array used as the key for hashing.
    /// * `counter_key` - An 8-bit unsigned integer used to XOR the counter.
    /// * `epoch` - The epoch time from which to measure timestamps.
    ///
    /// # Returns
    /// - `Result<Self, SINTEFlakeError>`: A new SINTEFlake instance or an error if creation fails.
    ///
    /// # Errors
    /// Returns an error if the instance_id is too high (>16383) or if the initial time update fails.
    pub fn custom(
        instance_id: u16,
        hash_key: [u8; 16],
        counter_key: u8,
        epoch: OffsetDateTime,
    ) -> Result<Self, SINTEFlakeError> {
        if instance_id > 16383 {
            return Err(SINTEFlakeError::InstanceIDTooHigh);
        }
        let mut instance = SINTEFlake {
            instance_id,
            hash_key,
            counter_key,
            epoch,
            collisions_map: [0; 16384],
            current_timestamp_bits: 0,
            ids_count_at_current_timestamp: 0,
        };

        instance.update_time()?;

        Ok(instance)
    }

    /// Sets the instance ID for this SINTEFlake instance.
    ///
    /// # Arguments
    /// * `instance_id` - A 14-bit unsigned integer representing the new instance ID.
    ///
    /// # Returns
    /// - `Result<(), SINTEFlakeError>`: Ok if successful, or an error if the instance_id is too high.
    ///
    /// # Errors
    /// Returns an error if the instance_id is greater than 16383.
    pub fn set_instance_id(&mut self, instance_id: u16) -> Result<(), SINTEFlakeError> {
        if instance_id > 16383 {
            return Err(SINTEFlakeError::InstanceIDTooHigh);
        }
        self.instance_id = instance_id;
        Ok(())
    }

    /// Updates the internal timestamp of the SINTEFlake instance.
    ///
    /// # Returns
    /// - `Result<(), SINTEFlakeError>`: Ok if successful, or an error if the time update fails.
    ///
    /// # Errors
    /// Returns an error if unable to get the current timestamp.
    pub fn update_time(&mut self) -> Result<(), SINTEFlakeError> {
        let current_timestamp = get_current_timestamp(self.epoch)?;
        let permuted_timestamp = permute_u32_31_bits(current_timestamp);
        if permuted_timestamp != self.current_timestamp_bits {
            // not clear because we want to start
            // from a clean memory allocation
            self.collisions_map = [0; 16384];
            self.current_timestamp_bits = permuted_timestamp;
            self.ids_count_at_current_timestamp = 0;
        }
        Ok(())
    }

    /// Generates the next unique ID.
    ///
    /// # Returns
    /// - `Result<u64, SINTEFlakeError>`: A new unique 64-bit ID, or an error if generation fails.
    ///
    /// # Errors
    /// Returns an error if there's a counter overflow.
    pub fn next_id(&mut self) -> Result<u64, SINTEFlakeError> {
        self.next_id_with_hash(&self.ids_count_at_current_timestamp.to_be_bytes())
    }

    fn shuffle_hash_counter(&self, counter: u8) -> u8 {
        permute_u8(counter ^ self.counter_key)
    }

    /// Generates the next unique ID using the provided data for hashing.
    ///
    /// # Arguments
    /// * `data` - A byte slice used to generate the hash part of the ID.
    ///
    /// # Returns
    /// - `Result<u64, SINTEFlakeError>`: A new unique 64-bit ID, or an error if generation fails.
    ///
    /// # Errors
    /// Returns an error if there's a counter overflow.
    pub fn next_id_with_hash(&mut self, data: &[u8]) -> Result<u64, SINTEFlakeError> {
        let mut hash = hash::hash(data, &self.hash_key);
        let mut counter = 0;

        loop {
            let hash_counter = self.collisions_map[hash as usize];
            // if the hash counter has overflowed
            if hash_counter == 256 {
                // we give ourselves 10 tries to find a new hash
                // with enough space
                if counter == 10 {
                    return Err(SINTEFlakeError::CounterOverflow);
                }
                counter += 1;
                // we just increment the hash by one
                hash = (hash + 1) % 16384;
                continue;
            }
            self.collisions_map[hash as usize] += 1;

            let timestamp = self.current_timestamp_bits;
            let instance_id = 0; // no instance id
            let shuffled_counter = self.shuffle_hash_counter(hash_counter as u8);
            self.ids_count_at_current_timestamp += 1;
            return Ok(construct_identifier(
                hash,
                timestamp,
                instance_id,
                shuffled_counter,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut instance = SINTEFlake::new().unwrap();
        let id_a = instance.next_id().unwrap();
        let id_b = instance.next_id().unwrap();
        assert_ne!(id_a, id_b);
    }

    #[test]
    fn test_with_hash() {
        let mut instance = SINTEFlake::new().unwrap();
        let data = [1, 2, 3];
        let id_a = instance.next_id_with_hash(&data).unwrap();
        let id_b = instance.next_id_with_hash(&data).unwrap();
        assert_ne!(id_a, id_b);
    }

    #[test]
    fn test_2048_collisions() {
        let mut instance = SINTEFlake::new().unwrap();
        let mut id_a = instance.next_id().unwrap();
        for _ in 0..2048 {
            let id_b = instance.next_id().unwrap();
            assert_ne!(id_a, id_b);
            id_a = id_b;
        }
    }

    #[test]
    fn test_too_many_collisions() {
        let mut instance = SINTEFlake::new().unwrap();
        let data = [1, 2, 3];
        let mut id_a = instance.next_id_with_hash(&data).unwrap();
        for _ in 0..2815 {
            let id_b = instance.next_id_with_hash(&data).unwrap();
            assert_ne!(id_a, id_b);
            id_a = id_b;
        }
        assert!(instance.next_id_with_hash(&data).is_err());
    }

    #[test]
    fn test_custom() {
        let mut normal_instance = SINTEFlake::new().unwrap();

        let mut custom_instance_a = SINTEFlake::custom(
            0,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            123,
            OffsetDateTime::from_unix_timestamp(1719792000).unwrap(),
        )
        .unwrap();
        let mut custom_instance_b = SINTEFlake::custom(
            0,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17],
            123,
            OffsetDateTime::from_unix_timestamp(1719792000).unwrap(),
        )
        .unwrap();
        let mut custom_instance_c = SINTEFlake::custom(
            0,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            124,
            OffsetDateTime::from_unix_timestamp(1719792000).unwrap(),
        )
        .unwrap();
        let mut custom_instance_d = SINTEFlake::custom(
            0,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            123,
            OffsetDateTime::from_unix_timestamp(1719792008).unwrap(),
        )
        .unwrap();

        let id_a = normal_instance.next_id().unwrap();
        let id_b = custom_instance_a.next_id().unwrap();
        let id_c = custom_instance_b.next_id().unwrap();
        let id_d = custom_instance_c.next_id().unwrap();
        let id_e = custom_instance_d.next_id().unwrap();

        // test that all ids are different
        assert_ne!(id_a, id_b);
        assert_ne!(id_a, id_c);
        assert_ne!(id_a, id_d);
        assert_ne!(id_a, id_e);
        assert_ne!(id_b, id_c);
        assert_ne!(id_b, id_d);
        assert_ne!(id_b, id_e);
        assert_ne!(id_c, id_d);
        assert_ne!(id_c, id_e);
        assert_ne!(id_d, id_e);
    }

    #[test]
    fn test_set_instance_id() {
        let mut instance = SINTEFlake::new().unwrap();
        let id_a = instance.next_id().unwrap();
        assert!(instance.set_instance_id(16384).is_err());
        assert!(instance.set_instance_id(16383).is_ok());
        let id_b = instance.next_id().unwrap();
        assert_ne!(id_a, id_b);
    }

    #[test]
    fn test_custom_instance_id() {
        let mut instance = SINTEFlake::custom(
            16383,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            123,
            OffsetDateTime::from_unix_timestamp(1719792000).unwrap(),
        )
        .unwrap();
        let id_a = instance.next_id().unwrap();
        let id_b = instance.next_id().unwrap();
        assert_ne!(id_a, id_b);

        assert!(SINTEFlake::custom(
            16384,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            123,
            OffsetDateTime::from_unix_timestamp(1719792000).unwrap(),
        )
        .is_err());
    }
}
