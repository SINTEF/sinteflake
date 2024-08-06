use siphasher::sip::SipHasher24;

pub(crate) fn hash(array: &[u8], key: &[u8; 16]) -> u16 {
    let hasher = SipHasher24::new_with_key(key);
    let hash_64 = hasher.hash(array);

    // keep only the last 12 bits
    const MASK: u64 = 0x0000_0000_0000_0FFF;

    (hash_64 & MASK) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fibonnaci sequence
    // {1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987}
    // modulo 256
    // {1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 121, 98, 219}

    const TEST_KEY: [u8; 16] = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 121, 98, 219];

    #[test]
    fn test_hash_with_default_key() {
        let input = b"Hello, world!";
        assert_eq!(hash(input, &TEST_KEY), 669);
    }

    #[test]
    fn test_hash_with_custom_key() {
        let input = b"Hello, world!";
        let custom_key = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        assert_eq!(hash(input, &custom_key), 3635);
    }

    #[test]
    fn test_hash_empty_input() {
        let input = b"";
        assert_eq!(hash(input, &TEST_KEY), 2265);
    }

    #[test]
    fn test_hash_long_input() {
        let input = b"This is a longer input string to test the hash function with more data";
        assert_eq!(hash(input, &TEST_KEY), 1330);
    }

    #[test]
    fn test_hash_different_inputs_produce_different_hashes() {
        let input1 = b"Input 1";
        let input2 = b"Input 2";
        assert_ne!(hash(input1, &TEST_KEY), hash(input2, &TEST_KEY));
    }

    #[test]
    fn test_hash_same_input_different_keys() {
        let input = b"Same input";
        let key1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let key2 = [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_ne!(hash(input, &key1), hash(input, &key2));
    }

    #[test]
    fn test_collisions_should_be_likely_by_design() {
        let good_input = b"Hello, world!";
        let good_hash = hash(good_input, &TEST_KEY);

        // for loop to find the collision, but should be with i = 565
        for i in 0..65535_u16 {
            let other_input = i.to_be_bytes();
            let bad_hash = hash(&other_input, &TEST_KEY);

            if bad_hash == good_hash {
                return;
            }
        }

        panic!("No collision found, this is unexpected");
    }
}
