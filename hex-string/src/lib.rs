mod hex;

pub use crate::hex::*;

#[test]
fn test_hexstring_from() {
    let my_name = "Liangcheng Juves";
    let hex = hex::HexString::from(my_name.as_bytes());
    assert_eq!("4c69616e676368656e67204a75766573", hex);
}

#[test]
fn test_hexstring_parse() {
    let hex = "4c69616e676368656e67204a75766573";
    let bytes = hex::HexString::parse(hex);
    assert_eq!("Liangcheng Juves".to_string().as_bytes(), bytes);
}
