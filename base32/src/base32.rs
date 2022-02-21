const ALPHABET: [char; 32] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', //////
    '2', '3', '4', '5', '6', '7',
];

/// Number of bytes per group
///
/// When encoding, every 5 eight bits is converted to every 8 five bits
const NUMBER_OF_BYTES_PER_GROUP: usize = 5;

/// Number of encoded bytes per group
///
/// When decoding, every 8 five bits is converted to every 5 eight bits
const NUMBER_OF_ENCODED_BYTES_PER_GROUP: usize = 8;

#[allow(dead_code)]
pub fn encode(src: &[u8]) -> Vec<u8> {
    let src_len = src.len();

    let mut cpy = src.iter().cloned().collect::<Vec<u8>>();
    (|src_len| {
        let remainder = src_len % NUMBER_OF_BYTES_PER_GROUP;
        if remainder != 0 {
            for _ in 0..(NUMBER_OF_BYTES_PER_GROUP - remainder) {
                cpy.push(b'\0');
            }
        }
    })(src_len);

    let mut cpy_i = 0usize;

    let mut dst = Vec::<u8>::new();

    while cpy_i < cpy.len() {
        let cpy_i_1 = cpy_i + 1;
        let cpy_i_2 = cpy_i + 2;
        let cpy_i_3 = cpy_i + 3;
        let cpy_i_4 = cpy_i + 4;

        // Alphabet unsigned indexes
        let albet_i_0 = cpy[cpy_i] as usize;
        let albet_i_1 = cpy[cpy_i_1] as usize;
        let albet_i_2 = cpy[cpy_i_2] as usize;
        let albet_i_3 = cpy[cpy_i_3] as usize;
        let albet_i_4 = cpy[cpy_i_4] as usize;

        dst.push(ALPHABET[albet_i_0 >> 3] as u8);
        dst.push(ALPHABET[(albet_i_0 & 0x7) << 2 | albet_i_1 >> 6] as u8);
        dst.push(match b'\0' != cpy[cpy_i_1] && cpy_i_1 != src_len {
            true => ALPHABET[(albet_i_1 & 0x3e) >> 1] as u8,
            _ => b'=',
        });
        dst.push(match b'\0' != cpy[cpy_i_1] && cpy_i_1 != src_len {
            true => ALPHABET[(albet_i_1 & 0x1) << 4 | (albet_i_2 & 0xf0) >> 4] as u8,
            _ => b'=',
        });
        dst.push(match b'\0' != cpy[cpy_i_2] && cpy_i_2 != src_len + 1 {
            true => ALPHABET[(albet_i_2 & 0xf) << 1 | (albet_i_3 & 0x80) >> 7] as u8,
            _ => b'=',
        });
        dst.push(match b'\0' != cpy[cpy_i_3] && cpy_i_3 != src_len + 2 {
            true => ALPHABET[(albet_i_3 & 0x7c) >> 2] as u8,
            _ => b'=',
        });
        dst.push(match b'\0' != cpy[cpy_i_3] && cpy_i_3 != src_len + 2 {
            true => ALPHABET[(albet_i_3 & 0x3) << 3 | (albet_i_4 & 0xe0) >> 5] as u8,
            _ => b'=',
        });
        dst.push(match b'\0' != cpy[cpy_i_4] && cpy_i_4 != src_len + 3 {
            true => ALPHABET[albet_i_4 & 0x1f] as u8,
            _ => b'=',
        });

        cpy_i += NUMBER_OF_BYTES_PER_GROUP;
    }

    dst
}

#[allow(dead_code)]
pub fn decode(src: &[u8]) -> Vec<u8> {
    // Find the index of every eight from the Base32 encoding table
    let find_albet_i = |elems: &[u8]| -> (i8, i8, i8, i8, i8, i8, i8, i8) {
        // Define every eight as a pair of indexes
        let mut albet_i = (-1i8, -1i8, -1i8, -1i8, -1i8, -1i8, -1i8, -1i8);
        let mut count = 0u8;

        for i in 0..(ALPHABET.len()) {
            // Single element in Base32 encoding table
            let ale = ALPHABET[i] as u8;

            for elemi in 0..(elems.len()) {
                if ale == elems[elemi] {
                    let i_i = i as i8;
                    match elemi {
                        0 => albet_i.0 = i_i,
                        1 => albet_i.1 = i_i,
                        2 => albet_i.2 = i_i,
                        3 => albet_i.3 = i_i,
                        4 => albet_i.4 = i_i,
                        5 => albet_i.5 = i_i,
                        6 => albet_i.6 = i_i,
                        7 => albet_i.7 = i_i,
                        _ => panic!("error"),
                    }
                    match count {
                        7 => return albet_i, /* Eight indexes found */
                        _ => count += 1,
                    }
                }
            }
        }

        albet_i
    };

    let mut src_i = 0usize;

    let mut dst = Vec::<u8>::new();

    loop {
        if src_i == src.len() {
            break;
        }

        // Find the index of every eight elements from the Base32 encoding table
        let (
            albet_i_0,
            albet_i_1,
            albet_i_2,
            albet_i_3,
            albet_i_4,
            albet_i_5,
            albet_i_6,
            albet_i_7,
        ) = find_albet_i(&src[src_i..(src_i + NUMBER_OF_ENCODED_BYTES_PER_GROUP)]);

        dst.push((albet_i_0 << 3 | albet_i_1 >> 2) as u8);

        match albet_i_2 != -1 && albet_i_3 != -1 {
            false => break,
            _ => dst.push((albet_i_1 << 6 | albet_i_2 << 1 | albet_i_3 >> 4) as u8),
        }

        match albet_i_4 {
            -1 => break,
            _ => dst.push((albet_i_3 << 4 | albet_i_4 >> 1) as u8),
        }

        match albet_i_5 != -1 && albet_i_6 != -1 {
            false => break,
            _ => dst.push((albet_i_4 << 7 | albet_i_5 << 2 | albet_i_6 >> 3) as u8),
        }

        match albet_i_7 {
            -1 => break,
            _ => dst.push((albet_i_6 << 5 | albet_i_7) as u8),
        }

        src_i += NUMBER_OF_ENCODED_BYTES_PER_GROUP;
    }
    dst
}
