mod base32;

pub use crate::base32::*;

#[test]
fn test_base32_encode() {
    let ret0 = base32::encode("Liangcheng Juves".as_bytes());
    assert_eq!(
        String::from_utf8_lossy(&ret0),
        "JRUWC3THMNUGK3THEBFHK5TFOM======"
    );

    let ret1 = base32::encode("Man".as_bytes());
    assert_eq!(String::from_utf8_lossy(&ret1), "JVQW4===");

    let ret2 = base32::encode("Manan".as_bytes());
    assert_eq!(String::from_utf8_lossy(&ret2), "JVQW4YLO");

    let url = "https://www.google.com/search?q=Rust&sxsrf=ALeKk00l7Td5aOgwE00oqRkU_6cOa4-52w%3A1626874984460&source=hp&ei=aCT4YLGiGOnT5NoPlvK2-Ac&iflsig=AINFCbYAAAAAYPgyeOszJycnR33U3lYo0BoAyCgpBU63&oq=Rust&gs_lcp=Cgdnd3Mtd2l6EAMyAggAMgIIADICCAAyAggAMgIIADICCAAyAggAMgIIADICCAAyAggAUPYfWOYkYKUoaABwAHgAgAHzBIgBow-SAQczLTEuMS4ymAEAoAEBqgEHZ3dzLXdpeg&sclient=gws-wiz&ved=0ahUKEwix9qfMpfTxAhXpKVkFHRa5DX8Q4dUDCAc&uact=5";
    let ret_string = "NB2HI4DTHIXS653XO4XGO33PM5WGKLTDN5WS643FMFZGG2B7OE6VE5LTOQTHG6DTOJTD2QKMMVFWWMBQNQ3VIZBVMFHWO52FGAYG64KSNNKV6NTDJ5QTILJVGJ3SKM2BGE3DENRYG42DSOBUGQ3DAJTTN52XEY3FHVUHAJTFNE6WCQ2UGRMUYR3JI5HW4VBVJZXVA3DWJMZC2QLDEZUWM3DTNFTT2QKJJZDEGYSZIFAUCQKBLFIGO6LFJ5ZXUSTZMNXFEMZTKUZWYWLPGBBG6QLZINTXAQSVGYZSM33RHVJHK43UEZTXGX3MMNYD2Q3HMRXGIM2NORSDE3BWIVAU26KBM5TUCTLHJFEUCRCJINBUCQLZIFTWOQKNM5EUSQKEJFBUGQKBPFAWOZ2BJVTUSSKBIREUGQ2BIF4UCZ3HIFKVAWLGK5HVS22ZJNKW6YKBIJ3UCSDHIFTUCSD2IJEWOQTPO4WVGQKRMN5EYVCFOVGVGNDZNVAUKQLPIFCUE4LHIVEFUM3EPJGFQZDQMVTSM43DNRUWK3TUHVTXO4ZNO5UXUJTWMVSD2MDBNBKUWRLXNF4DS4LGJVYGMVDYIFUFQ4CLKZVUMSCSME2UIWBYKE2GIVKEINAWGJTVMFRXIPJV";

    let ret3 = base32::encode(url.as_bytes());
    assert_eq!(String::from_utf8_lossy(&ret3), ret_string);

    let ret4 = base32::encode("  ".as_bytes());
    assert_eq!(String::from_utf8_lossy(&ret4), "EAQA====");

    let ret5 = base32::encode("ABcd".as_bytes());
    assert_eq!(String::from_utf8_lossy(&ret5), "IFBGGZA=");
}

#[test]
fn test_base32_decode() {
    let ret0 = base32::decode("IE======".as_bytes());
    assert_eq!(String::from_utf8_lossy(&ret0), "A");

    let ret1 = base32::decode("IFBGG===".as_bytes());
    assert_eq!(String::from_utf8_lossy(&ret1), "ABc");

    let ret2 = base32::decode("IFBGGZA=".as_bytes());
    assert_eq!(String::from_utf8_lossy(&ret2), "ABcd");

    let url = "https://www.google.com/search?q=Rust&sxsrf=ALeKk00l7Td5aOgwE00oqRkU_6cOa4-52w%3A1626874984460&source=hp&ei=aCT4YLGiGOnT5NoPlvK2-Ac&iflsig=AINFCbYAAAAAYPgyeOszJycnR33U3lYo0BoAyCgpBU63&oq=Rust&gs_lcp=Cgdnd3Mtd2l6EAMyAggAMgIIADICCAAyAggAMgIIADICCAAyAggAMgIIADICCAAyAggAUPYfWOYkYKUoaABwAHgAgAHzBIgBow-SAQczLTEuMS4ymAEAoAEBqgEHZ3dzLXdpeg&sclient=gws-wiz&ved=0ahUKEwix9qfMpfTxAhXpKVkFHRa5DX8Q4dUDCAc&uact=5";
    let ret_string = "NB2HI4DTHIXS653XO4XGO33PM5WGKLTDN5WS643FMFZGG2B7OE6VE5LTOQTHG6DTOJTD2QKMMVFWWMBQNQ3VIZBVMFHWO52FGAYG64KSNNKV6NTDJ5QTILJVGJ3SKM2BGE3DENRYG42DSOBUGQ3DAJTTN52XEY3FHVUHAJTFNE6WCQ2UGRMUYR3JI5HW4VBVJZXVA3DWJMZC2QLDEZUWM3DTNFTT2QKJJZDEGYSZIFAUCQKBLFIGO6LFJ5ZXUSTZMNXFEMZTKUZWYWLPGBBG6QLZINTXAQSVGYZSM33RHVJHK43UEZTXGX3MMNYD2Q3HMRXGIM2NORSDE3BWIVAU26KBM5TUCTLHJFEUCRCJINBUCQLZIFTWOQKNM5EUSQKEJFBUGQKBPFAWOZ2BJVTUSSKBIREUGQ2BIF4UCZ3HIFKVAWLGK5HVS22ZJNKW6YKBIJ3UCSDHIFTUCSD2IJEWOQTPO4WVGQKRMN5EYVCFOVGVGNDZNVAUKQLPIFCUE4LHIVEFUM3EPJGFQZDQMVTSM43DNRUWK3TUHVTXO4ZNO5UXUJTWMVSD2MDBNBKUWRLXNF4DS4LGJVYGMVDYIFUFQ4CLKZVUMSCSME2UIWBYKE2GIVKEINAWGJTVMFRXIPJV";

    let ret3 = base32::decode(ret_string.as_bytes());
    assert_eq!(String::from_utf8_lossy(&ret3), url);

    let ret4 = base32::decode("EAQA====".as_bytes());
    assert_eq!(String::from_utf8_lossy(&ret4), "  ");
}