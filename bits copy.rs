/// Constructs a 64-bit identifier from the given components.
///
/// # Arguments
///
/// * `hash` - A 16-bit hash or random number (only 12 bits will be used).
/// * `timestamp` - A 32-bit timestamp (only 31 bits will be used).
/// * `instance_id` - A 16-bit instance identifier (only 10 bits will be used).
/// * `sequence` - A 16-bit sequence number (only 10 bits will be used).
///
/// # Returns
///
/// A u64 containing the combined identifier.
pub fn construct_identifier(hash: u16, timestamp: u32, instance_id: u16, sequence: u16) -> u64 {
    // Ensure we only use the specified number of bits for each component
    let hash = (hash & 0xFFF) as u64; // 12 bits
    let timestamp = (timestamp & 0x7FFFFFFF) as u64; // 31 bits
    let instance_id = (instance_id & 0x3FF) as u64; // 10 bits
    let sequence = (sequence & 0x3FF) as u64; // 10 bits

    // Combine the components using bitwise operations
    (hash << 51) | (timestamp << 20) | (instance_id << 10) | sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_construction() {
        let result = construct_identifier(0x07FF, 0x3FFFFFFF, 0x01FF, 0x01FF);
        // 011111111111 0111111111111111111111111111111 0111111111 0111111111
        assert_eq!(result, 0x3FFBFFFFFFF7FDFF, "Basic construction failed");
    }

    #[test]
    fn test_zero_values() {
        let result = construct_identifier(0, 0, 0, 0);
        assert_eq!(result, 0, "All zero input should produce zero output");
    }

    #[test]
    fn test_max_values() {
        let result = construct_identifier(0xFFFF, 0xFFFFFFFF, 0xFFFF, 0xFFFF);
        assert_eq!(
            result, 0x7FFFFFFFFFFFFFFF,
            "Max values should be truncated correctly"
        );
    }

    #[test]
    fn test_bit_overflow() {
        let hash = 0x1FFF; // 13 bits, one more than allowed
        let timestamp = 0xFFFFFFFF; // 32 bits, one more than allowed
        let instance_id = 0x7FF; // 11 bits, one more than allowed
        let sequence = 0x7FF; // 11 bits, one more than allowed

        let result = construct_identifier(hash, timestamp, instance_id, sequence);
        assert_eq!(
            result, 0x7FFFFFFFFFFFFFFF,
            "Overflow bits should be ignored"
        );
    }

    #[test]
    fn test_individual_components() {
        let result = construct_identifier(0x0FFF, 0, 0, 0);
        assert_eq!(result, 0x7FF8000000000000, "Hash component incorrect");

        let result = construct_identifier(0, 0x7FFFFFFF, 0, 0);
        assert_eq!(result, 0x00007FFFFFFF00000, "Timestamp component incorrect");

        let result = construct_identifier(0, 0, 0x03FF, 0);
        assert_eq!(result, 0x0000000000FFC00, "Instance ID component incorrect");

        let result = construct_identifier(0, 0, 0, 0x03FF);
        assert_eq!(result, 0x00000000000003FF, "Sequence component incorrect");
    }

    #[test]
    fn test_random_values() {
        let result = construct_identifier(0x0ABC, 0x12345678, 0x0123, 0x0456);
        assert_eq!(
            result, 6188266143980227670,
            "Random value construction failed"
        );

        let result = construct_identifier(0x0ABD, 0x12345679, 0x0124, 0x0457);
        assert_eq!(
            result, 6190517943794962519,
            "Random value construction failed"
        );
    }
}
