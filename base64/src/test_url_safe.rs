#[allow(unused_imports)]
use crate::base64;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::Result;
#[allow(unused_imports)]
use std::io::Write;

#[test]
fn test_base64_encode_url_safe() -> Result<()> {
    let url = String::from("https://www.google.com/search?q=Rust&source=hp&ei=WzT0YMmgDOer5NoPpLyEoAY&iflsig=AINFCbYAAAAAYPRCa1InDdoQP5VAM4U0_du5t3X-cMyD&oq=Rust&gs_lcp=Cgdnd3Mtd2l6EANQ5B5YoSNg8SZoAXAAeACAAQCIAQCSAQCYAQCgAQGqAQdnd3Mtd2l6sAEA&sclient=gws-wiz&ved=0ahUKEwiJupKf5OzxAhXnFVkFHSQeAWQQ4dUDCAo&uact=5");

    let url_bytes = url.as_bytes();

    let chk_ret_str = String::from("aHR0cHM6Ly93d3cuZ29vZ2xlLmNvbS9zZWFyY2g_cT1SdXN0JnNvdXJjZT1ocCZlaT1XelQwWU1tZ0RPZXI1Tm9QcEx5RW9BWSZpZmxzaWc9QUlORkNiWUFBQUFBWVBSQ2ExSW5EZG9RUDVWQU00VTBfZHU1dDNYLWNNeUQmb3E9UnVzdCZnc19sY3A9Q2dkbmQzTXRkMmw2RUFOUTVCNVlvU05nOFNab0FYQUFlQUNBQVFDSUFRQ1NBUUNZQVFDZ0FRR3FBUWRuZDNNdGQybDZzQUVBJnNjbGllbnQ9Z3dzLXdpeiZ2ZWQ9MGFoVUtFd2lKdXBLZjVPenhBaFhuRlZrRkhTUWVBV1FRNGRVRENBbyZ1YWN0PTU=");

    let ret = base64::encode(&url_bytes, true, false, false);

    assert_eq!(ret, chk_ret_str.as_bytes());

    Ok(())
}

#[test]
fn test_base64_decode_url_safe() -> Result<()> {
    let url = String::from("https://www.google.com/search?q=Rust&source=hp&ei=WzT0YMmgDOer5NoPpLyEoAY&iflsig=AINFCbYAAAAAYPRCa1InDdoQP5VAM4U0_du5t3X-cMyD&oq=Rust&gs_lcp=Cgdnd3Mtd2l6EANQ5B5YoSNg8SZoAXAAeACAAQCIAQCSAQCYAQCgAQGqAQdnd3Mtd2l6sAEA&sclient=gws-wiz&ved=0ahUKEwiJupKf5OzxAhXnFVkFHSQeAWQQ4dUDCAo&uact=5");

    let url_bytes = url.as_bytes();

    let chk_ret_str = String::from("aHR0cHM6Ly93d3cuZ29vZ2xlLmNvbS9zZWFyY2g_cT1SdXN0JnNvdXJjZT1ocCZlaT1XelQwWU1tZ0RPZXI1Tm9QcEx5RW9BWSZpZmxzaWc9QUlORkNiWUFBQUFBWVBSQ2ExSW5EZG9RUDVWQU00VTBfZHU1dDNYLWNNeUQmb3E9UnVzdCZnc19sY3A9Q2dkbmQzTXRkMmw2RUFOUTVCNVlvU05nOFNab0FYQUFlQUNBQVFDSUFRQ1NBUUNZQVFDZ0FRR3FBUWRuZDNNdGQybDZzQUVBJnNjbGllbnQ9Z3dzLXdpeiZ2ZWQ9MGFoVUtFd2lKdXBLZjVPenhBaFhuRlZrRkhTUWVBV1FRNGRVRENBbyZ1YWN0PTU=");

    let ret = base64::decode(&chk_ret_str, true);

    assert_eq!(ret, url_bytes);

    Ok(())
}
