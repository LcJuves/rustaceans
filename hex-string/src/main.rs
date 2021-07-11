fn main() {
    let my_name = "Liangcheng Juves".to_string();
    let hex = HexString.from(my_name.as_bytes());
    println!("Convert \"{}\" to hex string is {}", my_name, hex);

    let bytes = HexString.parse(&hex);
    println!("Convert \"{}\" to bytes is {:?}", hex, bytes);
    println!(
        "Prased string is \"{}\"",
        String::from_utf8_lossy(bytes.as_slice())
    );
}

#[test]
fn test_hexstring_from() {
    let my_name = "Liangcheng Juves".to_string();
    let hex = HexString.from(my_name.as_bytes());
    assert_eq!("4c69616e676368656e67204a75766573", hex);
}

#[test]
fn test_hexstring_parse() {
    let hex = "4c69616e676368656e67204a75766573";
    let bytes = HexString.parse(hex);
    assert_eq!("Liangcheng Juves".to_string().as_bytes(), bytes);
}

struct HexString;

impl HexString {
    pub const RADIX: u32 = 16;

    pub fn from(&self, bytes: &[u8]) -> String {
        let char_from_digit = |num: u32| char::from_digit(num, HexString::RADIX).unwrap();
        let mut ret = String::new();
        for byte in bytes {
            let num = *byte as u32;
            ret.push(char_from_digit(num >> 4 & 0xF));
            ret.push(char_from_digit(num & 0xF));
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
            let mut digit = char_to_digit(2 * i);
            digit <<= 4;
            digit += char_to_digit(2 * i + 1);
            ret.push(digit as u8);
        }
        ret
    }
}
