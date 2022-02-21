mod base16;

fn main() {
    let my_name = "Liangcheng Juves";
    let ret = base16::encode(my_name.as_bytes());

    let ret_string = String::from_utf8_lossy(&ret);

    println!("Convert \"{}\" to base 16 is {}", my_name, ret_string);

    assert_eq!("4c69616e676368656e67204a75766573", ret_string.to_lowercase());

    let bytes = base16::decode(&ret);
    println!("Convert \"{}\" to bytes is {:?}", ret_string, bytes);
    println!("Parsed string is \"{}\"", String::from_utf8_lossy(bytes.as_slice()));
}

#[test]
fn test_base16_encode() {
    let my_name = "Liangcheng Juves";
    let ret = base16::encode(my_name.as_bytes());

    let ret_string = String::from_utf8_lossy(&ret);

    assert_eq!("4c69616e676368656e67204a75766573", ret_string.to_lowercase());
}

#[test]
fn test_base16_decode() {
    let hex = "4c69616e676368656e67204a75766573";
    let ret0 = base16::decode(hex.as_bytes());
    assert_eq!("Liangcheng Juves".to_string().as_bytes(), ret0);

    let ret1 = base16::decode(hex.to_uppercase().as_bytes());
    assert_eq!("Liangcheng Juves".to_string().as_bytes(), ret1);
}
