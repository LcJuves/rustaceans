#![allow(dead_code)]
pub struct HexString;

impl HexString {
    pub const RADIX: u32 = 16;

    pub fn from(bytes: &[u8]) -> String {
        let char_from_digit = |num: u32| char::from_digit(num, Self::RADIX).unwrap();
        let mut ret = String::new();
        for byte in bytes {
            let num = *byte as u32;
            ret.push(char_from_digit(num >> 4));
            ret.push(char_from_digit(num & 0xf));
        }
        ret
    }

    pub fn parse(hex: &str) -> Vec<u8> {
        let char_to_digit = |n: usize| hex.chars().nth(n).unwrap().to_digit(Self::RADIX).unwrap();
        let mut ret = Vec::<u8>::new();
        for i in 0..(hex.len() / 2) {
            let digit_idx = 2 * i;
            let mut digit = char_to_digit(digit_idx);
            digit <<= 4;
            digit |= char_to_digit(digit_idx + 1);
            ret.push(digit as u8);
        }
        ret
    }
}
