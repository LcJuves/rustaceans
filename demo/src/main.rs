mod comb_sum;
mod process_bar;
mod stream;

use std::io::Result;
// use stream::{open_cargo_toml, read_stream, ReadListenerImpl};
// use std::thread::sleep;
// use std::time::Duration;

fn main() -> Result<()> {
    ////////////////////////////////////////////////

    // let mut cargo_toml = open_cargo_toml()?;
    // read_stream(&mut cargo_toml, &mut ReadListenerImpl::new())?;

    ////////////////////////////////////////////////

    // for perc in 0..=100 {
    //     process_bar::draw(perc)?;
    //     sleep(Duration::from_micros(60_000));
    // }
    // println!();

    ////////////////////////////////////////////////
    let keys: [f64; 7] = [0f64, 1f64, 2f64, 3f64, 4f64, 5f64, 6f64];
    println!("{:?}", comb_sum::compute(&keys, 9f64));

    ////////////////////////////////////////////////
    Ok(())
}
