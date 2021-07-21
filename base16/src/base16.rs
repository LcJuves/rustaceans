const ALPHABET: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', //////
    'A', 'B', 'C', 'D', 'E', 'F',
];

#[allow(dead_code)]
pub fn encode(src: &[u8]) -> Vec<u8> {
    let mut dst = Vec::<u8>::new();
    for e in src {
        let albeti_0 = (e >> 4) as usize;
        let albeti_1 = (e & 0xf) as usize;

        dst.push(ALPHABET[albeti_0] as u8);
        dst.push(ALPHABET[albeti_1] as u8);
    }
    dst
}

#[allow(dead_code)]
pub fn decode(src: &[u8]) -> Vec<u8> {
    let mut dst = Vec::<u8>::new();
    const ALBET_LEN: usize = ALPHABET.len();

    // Find the index of every two from the Base16 encoding table
    let find_albeti = |elems: &[u8]| -> (u8, u8) {
        // Define every four as a pair of indexes
        let mut albeti = (0u8, 0u8);
        let mut count = 0u8;

        for i in 0..ALPHABET.len() {
            // Single element in Base16 encoding table
            let ale = ALPHABET[i] as u8;

            for elemi in 0..elems.len() {
                let mut curr_elem = elems[elemi];
                if curr_elem > b'Z' {
                    curr_elem = curr_elem - 32;
                }
                if ale == curr_elem {
                    let i_i = i as u8;
                    match elemi {
                        0 => albeti.0 = i_i,
                        1 => albeti.1 = i_i,
                        _ => panic!("error"),
                    }
                    match count {
                        1 => return albeti, /* Two indexes found */
                        _ => count += 1,
                    }
                }
            }
        }

        albeti
    };

    for i in 0..(src.len() / 2) {
        let srci = 2 * i;
        // Find the index of every two elements from the Base16 encoding table
        let (albeti_0, albeti_1) = find_albeti(&src[srci..=(srci + 1)]);
        let mut diget = albeti_0 << 4;
        diget |= albeti_1;
        dst.push(diget as u8);
    }

    dst
}
