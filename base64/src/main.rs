mod base64;
#[allow(unused_imports)]
mod test_no_wrap;
#[allow(unused_imports)]
mod test_url_safe;
#[allow(unused_imports)]
mod test_wrap;

use std::io::Result;

fn main() -> Result<()> {
    println!("{:08b}", -127i8);
    println!("{:08b}", 0xff);
    println!("{}", -127i8 as u8);
    Ok(())
}
