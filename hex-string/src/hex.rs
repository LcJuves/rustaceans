pub struct HexString;

impl HexString {
    pub const RADIX: u32 = 16;

    pub fn from(&self, bytes: &[u8]) -> String {
        let char_from_digit = |num: u32| char::from_digit(num, HexString::RADIX).unwrap();
        let mut ret = String::new();
        for byte in bytes {
            let num = *byte as u32;
            ret.push(char_from_digit(num >> 4 & 0xf));
            ret.push(char_from_digit(num & 0xf));
        }
        ret
    }

    pub fn parse(&self, hex: &str) -> Vec<u8> {
        let char_to_digit = |n: usize| {
            hex.chars()
                .nth(n)
                .unwrap()
                .to_digit(HexString::RADIX)
                .unwrap()
        };
        let mut ret = Vec::<u8>::new();
        for i in 0..(hex.len() / 2) {
            let digi = 2 * i;
            let mut digit = char_to_digit(digi);
            digit <<= 4;
            digit += char_to_digit(digi + 1);
            ret.push(digit as u8);
        }
        ret
    }
}
