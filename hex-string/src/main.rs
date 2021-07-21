mod hex;

use hex::HexString;

fn main() {
    let my_name = "Liangcheng Juves";
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
    let my_name = "Liangcheng Juves";
    let hex = HexString.from(my_name.as_bytes());
    assert_eq!("4c69616e676368656e67204a75766573", hex);
}

#[test]
fn test_hexstring_parse() {
    let hex = "4c69616e676368656e67204a75766573";
    let bytes = HexString.parse(hex);
    assert_eq!("Liangcheng Juves".to_string().as_bytes(), bytes);
}