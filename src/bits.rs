/// Constructs a 64-bit identifier from the given components.
///
/// # Arguments
///
/// * `hash` - A 16-bit hash or random number (only 14 bits will be used).
/// * `timestamp` - A 32-bit timestamp (only 31 bits will be used).
/// * `instance_id` - A 16-bit instance identifier (only 10 bits will be used).
/// * `sequence` - A 8-bit sequence number (only 8 bits will be used).
///
/// # Returns
///
/// A u64 containing the combined identifier.
pub fn construct_identifier(hash: u16, timestamp: u32, instance_id: u16, sequence: u8) -> u64 {
    // Ensure we only use the specified number of bits for each component
    let hash = (hash & 0x3FFF) as u64; // 14 bits
    let timestamp = (timestamp & 0x7FFFFFFF) as u64; // 31 bits
    let instance_id = (instance_id & 0x3FF) as u64; // 10 bits
    let sequence = sequence as u64; // 8 bits

    // Combine the components using bitwise operations
    (hash << 49) | (timestamp << 18) | (instance_id << 8) | sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_construction() {
        let result = construct_identifier(0x01FFF, 0x3FFFFFFF, 0x01FF, 0x07F);
        // 01111111111111 0111111111111111111111111111111 0111111111 01111111
        assert_eq!(result, 0x3FFEFFFFFFFDFF7F, "Basic construction failed");
    }

    #[test]
    fn test_zero_values() {
        let result = construct_identifier(0, 0, 0, 0);
        assert_eq!(result, 0, "All zero input should produce zero output");
    }

    #[test]
    fn test_max_values() {
        let result = construct_identifier(0xFFFF, 0xFFFFFFFF, 0xFFFF, 0xFF);
        assert_eq!(
            result, 0x7FFFFFFFFFFFFFFF,
            "Max values should be truncated correctly"
        );
    }

    #[test]
    fn test_bit_overflow() {
        let hash = 0x7FFF; // 15 bits, one more than allowed
        let timestamp = 0xFFFFFFFF; // 32 bits, one more than allowed
        let instance_id = 0x7FF; // 11 bits, one more than allowed
        let sequence = 0xFF; // 8 bits, cannot overflow

        let result = construct_identifier(hash, timestamp, instance_id, sequence);
        assert_eq!(
            result, 0x7FFFFFFFFFFFFFFF,
            "Overflow bits should be ignored"
        );
    }

    #[test]
    fn test_individual_components() {
        let result = construct_identifier(0x01FFF, 0, 0, 0);
        assert_eq!(result, 0x3FFE000000000000, "Hash component incorrect");

        let result = construct_identifier(0, 0x7FFFFFFF, 0, 0);
        assert_eq!(result, 0x00001FFFFFFFC0000, "Timestamp component incorrect");

        let result = construct_identifier(0, 0, 0x03FF, 0);
        assert_eq!(result, 0x00000000003FF00, "Instance ID component incorrect");

        let result = construct_identifier(0, 0, 0, 0x7F);
        assert_eq!(result, 0x000000000000007F, "Sequence component incorrect");
    }

    #[test]
    fn test_random_values() {
        let result = construct_identifier(0x0ABC, 0x12345678, 0x0123, 0x45);
        assert_eq!(
            result, 1547066535995056965,
            "Random value construction failed"
        );

        let result = construct_identifier(0x0ABD, 0x12345679, 0x0124, 0x45);
        assert_eq!(
            result, 1547629485948740677,
            "Random value construction failed"
        );
    }
}
