const ALPHABET: [char; 32] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', //////
    '2', '3', '4', '5', '6', '7',
];

/// Number of bytes per group
///
/// When encoding, every 5 eight bits is converted to every 8 five bits
const NUMBER_OF_BYTES_PER_GROUP: usize = 5;

#[allow(dead_code)]
pub fn encode(src: &[u8]) -> Vec<u8> {
    let src_len = src.len();

    let mut cpy = src.iter().cloned().collect::<Vec<u8>>();
    (|src_len| {
        let rder = src_len % NUMBER_OF_BYTES_PER_GROUP;
        if rder != 0 {
            for _ in 0..(NUMBER_OF_BYTES_PER_GROUP - rder) {
                cpy.push(b'\0');
            }
        }
    })(src_len);

    let mut cpyi = 0usize;

    let mut dst = Vec::<u8>::new();

    while cpyi < cpy.len() {
        let cpyi_1 = cpyi + 1;
        let cpyi_2 = cpyi + 2;
        let cpyi_3 = cpyi + 3;
        let cpyi_4 = cpyi + 4;

        // Alphabet unsigned indexes
        let albeti_0 = cpy[cpyi] as usize;
        let albeti_1 = cpy[cpyi_1] as usize;
        let albeti_2 = cpy[cpyi_2] as usize;
        let albeti_3 = cpy[cpyi_3] as usize;
        let albeti_4 = cpy[cpyi_4] as usize;

        dst.push(ALPHABET[albeti_0 >> 3] as u8);
        dst.push(ALPHABET[(albeti_0 & 0x7) << 2 | albeti_1 >> 6] as u8);
        dst.push(match b'\0' != cpy[cpyi_1] && cpyi_1 != src_len {
            true => ALPHABET[(albeti_1 & 0x3e) >> 1] as u8,
            _ => b'=',
        });
        dst.push(match b'\0' != cpy[cpyi_1] && cpyi_1 != src_len {
            true => ALPHABET[(albeti_1 & 0x1) << 4 | (albeti_2 & 0xf0) >> 4] as u8,
            _ => b'=',
        });
        dst.push(match b'\0' != cpy[cpyi_2] && cpyi_2 != src_len + 1 {
            true => ALPHABET[(albeti_2 & 0xf) << 1 | (albeti_3 & 0x80) >> 7] as u8,
            _ => b'=',
        });
        dst.push(match b'\0' != cpy[cpyi_3] && cpyi_3 != src_len + 2 {
            true => ALPHABET[(albeti_3 & 0x7c) >> 2] as u8,
            _ => b'=',
        });
        dst.push(match b'\0' != cpy[cpyi_3] && cpyi_3 != src_len + 2 {
            true => ALPHABET[(albeti_3 & 0x3) << 3 | (albeti_4 & 0xe0) >> 5] as u8,
            _ => b'=',
        });
        dst.push(match b'\0' != cpy[cpyi_4] && cpyi_4 != src_len + 3 {
            true => ALPHABET[albeti_4 & 0x1f] as u8,
            _ => b'=',
        });

        cpyi += NUMBER_OF_BYTES_PER_GROUP;
    }

    dst
}

#[allow(dead_code)]
pub fn decode(src: &[u8]) -> Vec<u8> {
    // Find the index of every eight from the Base32 encoding table
    let find_albeti = |elems: &[u8]| -> (i8, i8, i8, i8, i8, i8, i8, i8) {
        // Define every eight as a pair of indexes
        let mut albeti = (-1i8, -1i8, -1i8, -1i8, -1i8, -1i8, -1i8, -1i8);
        let mut count = 0u8;

        for i in 0..ALPHABET.len() {
            // Single element in Base32 encoding table
            let ale = ALPHABET[i] as u8;

            for elemi in 0..elems.len() {
                if ale == elems[elemi] {
                    let i_i = i as i8;
                    match elemi {
                        0 => albeti.0 = i_i,
                        1 => albeti.1 = i_i,
                        2 => albeti.2 = i_i,
                        3 => albeti.3 = i_i,
                        4 => albeti.4 = i_i,
                        5 => albeti.5 = i_i,
                        6 => albeti.6 = i_i,
                        7 => albeti.7 = i_i,
                        _ => panic!("error"),
                    }
                    match count {
                        7 => return albeti, /* Eight indexes found */
                        _ => count += 1,
                    }
                }
            }
        }

        albeti
    };

    let mut srci = 0usize;

    let mut dst = Vec::<u8>::new();

    loop {
        if srci == src.len() {
            break;
        }

        // Find the index of every eight elements from the Base32 encoding table
        let (albeti_0, albeti_1, albeti_2, albeti_3, albeti_4, albeti_5, albeti_6, albeti_7) =
            find_albeti(&src[srci..=(srci + 7)]);

        // [74, 86, 81, 87, 52, 89, 76, 79]
        // [77, 97, 110]

        dst.push((albeti_0 << 3 | albeti_1 >> 2) as u8);

        match albeti_2 != -1 && albeti_3 != -1 {
            false => break,
            _ => dst.push((albeti_1 << 6 | albeti_2 << 1 | albeti_3 >> 4) as u8),
        }

        match albeti_4 {
            -1 => break,
            _ => dst.push((albeti_3 << 4 | albeti_4 >> 1) as u8),
        }

        match albeti_5 != -1 && albeti_6 != -1 {
            false => break,
            _ => dst.push((albeti_4 << 7 | albeti_5 << 2 | albeti_6 >> 3) as u8),
        }

        match albeti_7 {
            -1 => break,
            _ => dst.push((albeti_6 << 5 | albeti_7) as u8),
        }

        srci += 8;
    }
    dst
}
