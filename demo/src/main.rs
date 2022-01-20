mod comb_sum;
mod gh_qrcode;
mod process_bar;
mod stream;

use std::io::Result;
use stream::{closure_read_stream, open_cargo_toml};
// use std::thread::sleep;
// use std::time::Duration;

fn main() -> Result<()> {
    ////////////////////////////////////////////////

    // let mut cargo_toml = open_cargo_toml()?;
    // read_stream(&mut cargo_toml, &mut ReadListenerImpl::new())?;

    let mut cargo_toml = open_cargo_toml()?;
    let mut read_listener = |bytes: &[u8], finished: bool| {
        if !finished {
            print!("{}", String::from_utf8_lossy(bytes));
        } else {
            const SIGN: &str = "========";
            println!();
            println!("{} finished {}", SIGN, SIGN);
            println!();
            print!("{}", String::from_utf8_lossy(bytes));
        }
    };
    closure_read_stream(&mut cargo_toml, &mut read_listener)?;

    ////////////////////////////////////////////////

    // for perc in 0..=100 {
    //     process_bar::draw(perc)?;
    //     sleep(Duration::from_micros(60_000));
    // }
    // println!();

    ////////////////////////////////////////////////
    // let keys: [f64; 7] = [0f64, 1f64, 2f64, 3f64, 4f64, 5f64, 6f64];
    // println!("{:?}", comb_sum::compute(&keys, 9f64));

    ////////////////////////////////////////////////
    Ok(())
}
