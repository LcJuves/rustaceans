const NORMAL_ALPHABET: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', //////
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', //////
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
];

const URL_SAFE_ALPHABET: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', //////
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', //////
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_',
];

/// Number of bytes per group
///
/// When encoding, every 3 eight bits is converted to every 4 six bits
const NUMBER_OF_BYTES_PER_GROUP: usize = 3;

/// Number of encoded bytes per group
///
/// When decoding, every 4 six bits is converted to every 3 eight bits
const NUMBER_OF_ENCODED_BYTES_PER_GROUP: usize = 4;

#[allow(dead_code)]
pub fn encode(src: &[u8], url_safe: bool, no_padding: bool, wrap: bool) -> Vec<u8> {
    let alphabet = match url_safe {
        true => URL_SAFE_ALPHABET,
        _ => NORMAL_ALPHABET,
    };

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

    let mut wrap_flag = 0usize;

    let pad_val = |no_padding: bool| if !no_padding { b'=' } else { b'\0' };

    let mut dst = Vec::<u8>::new();

    while cpyi < cpy.len() {
        let cpyi_1 = cpyi + 1;
        let cpyi_2 = cpyi + 2;

        // Alphabet unsigned indexes
        let albeti_0 = cpy[cpyi] as usize;
        let albeti_1 = cpy[cpyi_1] as usize;
        let albeti_2 = cpy[cpyi_2] as usize;

        dst.push(alphabet[albeti_0 >> 2] as u8);
        dst.push(alphabet[(albeti_0 & 0x3) << 4 | albeti_1 >> 4] as u8);
        dst.push(match b'\0' != cpy[cpyi_1] && cpyi_1 != src_len {
            true => alphabet[(albeti_1 & 0xf) << 2 | albeti_2 >> 6] as u8,
            _ => pad_val(no_padding),
        });
        dst.push(match b'\0' != cpy[cpyi_2] && cpyi_2 != src_len + 1 {
            true => alphabet[albeti_2 & 0x3f] as u8,
            _ => pad_val(no_padding),
        });

        // If wrap is true, the line will wrap every 76 characters
        if wrap {
            match wrap_flag {
                72 => {
                    dst.push(b'\n');
                    wrap_flag = 0;
                }
                _ => wrap_flag += NUMBER_OF_ENCODED_BYTES_PER_GROUP,
            }
        }

        cpyi += NUMBER_OF_BYTES_PER_GROUP;
    }
    dst
}

/// Remove useless characters in Base64 strings
fn remove_unused_chars(string: &str) -> Vec<u8> {
    let ret = string.clone();
    let ret = ret.replace("\r", "");
    let ret = ret.replace("\n", "");
    let ret = ret.replace("\t", "");
    let ret = ret.replace(" ", "");
    let back_val = ret.as_bytes().iter().cloned().collect::<Vec<u8>>();
    back_val
}

#[allow(dead_code)]
pub fn decode(string: &str, url_safe: bool) -> Vec<u8> {
    let alphabet = match url_safe {
        true => URL_SAFE_ALPHABET,
        _ => NORMAL_ALPHABET,
    };

    // Find the index of every four from the Base64 encoding table
    let find_albeti = |elems: &[u8]| -> (i8, i8, i8, i8) {
        // Define every four as a pair of indexes
        let mut albeti = (-1i8, -1i8, -1i8, -1i8);
        let mut count = 0u8;

        for i in 0..alphabet.len() {
            // Single element in Base64 encoding table
            let ale = alphabet[i] as u8;

            for elemi in 0..elems.len() {
                if ale == elems[elemi] {
                    let i_i = i as i8;
                    match elemi {
                        0 => albeti.0 = i_i,
                        1 => albeti.1 = i_i,
                        2 => albeti.2 = i_i,
                        3 => albeti.3 = i_i,
                        _ => panic!("error"),
                    }
                    match count {
                        3 => return albeti, /* Four indexes found */
                        _ => count += 1,
                    }
                }
            }
        }

        albeti
    };

    let mut srci = 0usize;

    let src = remove_unused_chars(string);
    let mut dst = Vec::<u8>::new();

    loop {
        if srci == src.len() {
            break;
        }

        // Find the index of every four elements from the Base64 encoding table
        let (albeti_0, albeti_1, albeti_2, albeti_3) =
            find_albeti(&src[srci..=(srci + (NUMBER_OF_ENCODED_BYTES_PER_GROUP - 1))]);

        dst.push((albeti_0 << 2 | albeti_1 >> 4) as u8);

        match albeti_2 {
            -1 => break,
            _ => dst.push((albeti_1 << 4 | albeti_2 >> 2) as u8),
        }

        match albeti_3 {
            -1 => break,
            _ => dst.push((albeti_2 << 6 | albeti_3) as u8),
        }

        srci += NUMBER_OF_ENCODED_BYTES_PER_GROUP;
    }
    dst
}
