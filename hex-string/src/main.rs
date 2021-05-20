use std::char;

fn main() {
    let my_name = "Liangcheng Juves".to_string();
    let hex = HexString.from(my_name.as_bytes());
    assert_eq!("4c69616e676368656e67204a75766573", hex);
    println!("Convert my_name to hex string is {}", hex);
    // let bytes = HexString.parse(hex);
    // println!("Convert hex string to my_name is {:?}", bytes);
}

struct HexString;

impl HexString {
    pub fn from(&self, bytes: &[u8]) -> String {
        let mut result = String::from("");
        let radix: u32 = 16;
        for byte in bytes {
            let num: u32 = *byte as u32;
            result.push(char::from_digit(num >> 4 & 0xF, radix).unwrap());
            result.push(char::from_digit(num & 0xF, radix).unwrap());
        }
        result
    }

    // pub fn parse(&self, |hex| hex: String) -> Vec<u8> {
    //     const result_arrlen = hex.len() / 2;
    //     let result: Vec<u8> = Vec::new();
    //     for i in 0..result_arrlen {
    //         let digit: u32;
    //         digit = hex.chars().nth(2 * i).to_digit(16);
    //         digit <<= 4;
    //         digit += hex.chars().nth(2 * i + 1).to_digit(16);
    //         result.push(digit)
    //     }
    //     result
    // }
}
