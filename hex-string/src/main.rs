use std::char;

fn main() {
    let my_name = "Liangcheng Juves".to_string();
    let bytes = my_name.as_bytes();
    println!("{}", HexString::from(bytes));
}

struct HexString;

impl HexString {
    pub fn from(bytes: &[u8]) -> String {
        let mut result = String::from("");
        let radix: u32 = 16;
        for byte in bytes {
            let num: u32 = *byte as u32;
            result.push(char::from_digit(num >> 4 & 0xF, radix).unwrap());
            result.push(char::from_digit(num & 0xF, radix).unwrap());
        }
        result
    }

    pub fn parse(@hex: String) -> &'static [u8] {
        let result: [u8; hex.len() / 2];
        result
    }
}
