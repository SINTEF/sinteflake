use bitvec::prelude::*;

pub(crate) fn permute_31_bits(input: &BitArray<[u32; 1], Lsb0>) -> BitArray<[u32; 1], Lsb0> {
    const PERMUTATION: [usize; 31] = [
        4, 16, 22, 21, 2, 5, 20, 12, 13, 6, 24, 25, 17, 8, 23, 0, 28, 3, 19, 18, 14, 1, 15, 27, 29,
        9, 10, 11, 26, 30, 7,
    ];

    let mut result = BitArray::<[u32; 1], Lsb0>::new([0]);

    for (new_position, &old_position) in PERMUTATION.iter().enumerate() {
        //result.set(new_position, input[old_position]);
        if input[old_position] {
            result.set(new_position, true);
        }
    }

    result
}

pub fn permute_u32_31_bits(input: u32) -> u32 {
    let input = BitArray::<[u32; 1], Lsb0>::new([input]);
    let result = permute_31_bits(&input);
    result.as_raw_slice()[0]
}

pub fn permute_u8(input: u8) -> u8 {
    const PERMUTATION: [usize; 8] = [5, 7, 6, 0, 2, 1, 3, 4];

    let input = BitArray::<[u8; 1], Lsb0>::new([input]);
    let mut result = BitArray::<[u8; 1], Lsb0>::new([0]);

    for (new_position, &old_position) in PERMUTATION.iter().enumerate() {
        if input[old_position] {
            result.set(new_position, true);
        }
    }

    result.as_raw_slice()[0]
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_permutation_table_31bits() {
        // this function create a nothing up my sleeve permutation table
        // using the digits of Pi.
        // We work in base 32 for simplicity, but we actually build a permutation table of 31 bits.

        // pi digits after the comma in base 32
        // 3.4gvm1245kc4d640ph8n06s3j80ie1256fj3k085rt9hr2edi4kaa11sosd04rnnpa6djp
        // https://www.wolframalpha.com/input?i=pi+in+base+32

        let text = "4gvml245kc4d64oph8n06s3j8ii0ie1256fj3k085rt9hr2edi4kaa11sosd04rnnpa6djpkt466pg5c56rsiv2grkvo9ldlml3gi5si2ratj2bpvcdt2c8bkqcdvddc5vun5mug3bfrfe71lvmmk9juiqt7p425u4m7v694k6ckfcshdjrgg0fisa2otv0mcdki1m3hat76j92ovqhv94ptfo6pat4fea7bcm3hhf6lh0gl9bn7ml543n15kmdljgodae9au9g17hehm0higo45u3542u8on3djhrsef7eb0o1q3076p7geheo1t2huqsanfgdt655ieu5f5vd5ao2sc3j5a9fjl9aqn52n92c64ov82h05bihpd8lam45mmj65od0h87kct8akgqnnosn9iepus50hcdnroahbl72lqt0o67rcsn1u2qdof4oulvbbkcrc4j7louhiae0ih5c6estoui4odd5rjbu4nvk1mph8469m3m09pjti3ach91uaoo2tti035rs4bleuj1blm7e2c8o2tdihn213h4v83ksmlj2gurbfue1v8ghp5o5k90l4ggg08qe8u159s7srbogscq22urkmp6j71ie63aujh3o6kkd0qbc58br8io7qea5ba4pq6rnf1dm16uhrsit3ns2gfrtil651u5ihqedf05r6diip7q1463k8hjn8c6a5dufr8vc4kn1jn2qunrg6uteogn0i0sq03929ulm1daj4tktac8r3uto63fvdusi2jc13qdugqsid02gi93dgvqmj97os16o7adpcj04p3dtibl3pr3rehnnnsfv506lmf563n5rcs2ug9g06nb0qijtm82fm1h2ubifc46ba4hhmhurflsv6oktl2csr5qprabm6urfsa4fpmc4l5j682ha4ltfbq2dusf809nhj9bumc3p80scisitjo35qglq5p1q0vkgbbssrjkvrrdanjg5t39g342mmk40ccg1ce9smf7p5vrthv";
        let mut visited_bits: HashSet<u8> = HashSet::new();
        let mut permutation_table: Vec<u8> = Vec::new();

        // convert base32 to number
        for c in text.chars() {
            let value = match c {
                '0'..='9' => c as u8 - b'0',
                'a'..='v' => c as u8 - b'a' + 10,
                _ => unreachable!(),
            };
            if visited_bits.insert(value) {
                // we actually don't insert the last bit in the permutation table because we workon 31 bits
                // we worked so far on 32bits for simplicity
                if value != 31 {
                    permutation_table.push(value);
                }
            }
        }

        assert_eq!(permutation_table.len(), 31);
        assert_eq!(
            permutation_table,
            vec![
                4, 16, 22, 21, 2, 5, 20, 12, 13, 6, 24, 25, 17, 8, 23, 0, 28, 3, 19, 18, 14, 1, 15,
                27, 29, 9, 10, 11, 26, 30, 7
            ]
        );
    }

    #[test]
    fn test_permutation_table_8bits() {
        // Same concept as the previous test, but for 8 bits and with e
        // 2.55760521305053551246527734254
        // https://www.wolframalpha.com/input?i=e+in+base+8

        let text = "55760521305053551246527734254";
        let mut visited_bits: HashSet<u8> = HashSet::new();
        let mut permutation_table: Vec<u8> = Vec::new();

        // convert base32 to number
        for c in text.chars() {
            let value = match c {
                '0'..='7' => c as u8 - b'0',
                _ => unreachable!(),
            };
            if visited_bits.insert(value) {
                permutation_table.push(value);
            }
        }

        assert_eq!(permutation_table.len(), 8);
        assert_eq!(permutation_table, vec![5, 7, 6, 0, 2, 1, 3, 4]);
    }

    #[test]
    fn test_permutation_31_bits() {
        let input = BitArray::<[u32; 1], Lsb0>::new([0b1010101010101010101010101010101]);
        let permuted = permute_31_bits(&input);
        let raw = permuted.as_raw_slice()[0];
        assert_eq!(raw, 0b0110100000110011010011011010111);
    }

    #[test]
    fn test_permutation_u32_31_bits() {
        assert_eq!(permute_u32_31_bits(1), 32768);
        assert_eq!(permute_u32_31_bits(123456789), 475315287);
    }

    #[test]
    fn test_permutation_u8() {
        assert_eq!(permute_u8(1), 8);
        assert_eq!(permute_u8(123), 237);
    }
}
