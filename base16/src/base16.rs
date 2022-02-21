const ALPHABET: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', //////
    'A', 'B', 'C', 'D', 'E', 'F',
];

/// Number of encoded bytes per group
///
/// When decoding, every 2 four bits is converted to every 1 eight bits
const NUMBER_OF_ENCODED_BYTES_PER_GROUP: usize = 2;

#[allow(dead_code)]
pub fn encode(src: &[u8]) -> Vec<u8> {
    let mut dst = Vec::<u8>::new();
    for e in src {
        let albet_i_0 = (e >> 4) as usize;
        let albet_i_1 = (e & 0xf) as usize;

        dst.push(ALPHABET[albet_i_0] as u8);
        dst.push(ALPHABET[albet_i_1] as u8);
    }
    dst
}

#[allow(dead_code)]
pub fn decode(src: &[u8]) -> Vec<u8> {
    let mut dst = Vec::<u8>::new();

    // Find the index of every two from the Base16 encoding table
    let find_albet_i = |elems: &[u8]| -> (u8, u8) {
        // Define every four as a pair of indexes
        let mut albet_i = (0u8, 0u8);
        let mut count = 0u8;

        for i in 0..ALPHABET.len() {
            // Single element in Base16 encoding table
            let ale = ALPHABET[i] as u8;

            for elemi in 0..elems.len() {
                let mut curr_elem = elems[elemi];
                if curr_elem < b'0' || curr_elem > b'z' {
                    panic!("Beyond the decodable range");
                } else if curr_elem > b'Z' {
                    curr_elem = curr_elem - 32;
                }
                if ale == curr_elem {
                    let i_i = i as u8;
                    match elemi {
                        0 => albet_i.0 = i_i,
                        1 => albet_i.1 = i_i,
                        _ => panic!("error"),
                    }
                    match count {
                        1 => return albet_i, /* Two indexes found */
                        _ => count += 1,
                    }
                }
            }
        }

        albet_i
    };

    for i in 0..(src.len() / NUMBER_OF_ENCODED_BYTES_PER_GROUP) {
        let src_i = NUMBER_OF_ENCODED_BYTES_PER_GROUP * i;
        // Find the index of every two elements from the Base16 encoding table
        let (albet_i_0, albet_i_1) =
            find_albet_i(&src[src_i..(src_i + NUMBER_OF_ENCODED_BYTES_PER_GROUP)]);
        let mut digit = albet_i_0 << 4;
        digit |= albet_i_1;
        dst.push(digit as u8);
    }

    dst
}
