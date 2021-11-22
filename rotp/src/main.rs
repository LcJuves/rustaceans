mod generator;
use generator::*;

fn main() {
    let secret = rb32::decode("FHCIDHYW3N46EJBIQWOSP4VURTYIJ3W7".as_bytes());
    use std::time::{SystemTime, UNIX_EPOCH};
    let time_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("time_millis >>> {}", time_millis);
    let time = (time_millis / 1000 / 30) as u64;
    println!("time >>> {}", time);
    let digits = 6;
    let otp = generator::gen_otp(&secret, time, digits, HmacShaAlgorithm::SHA1);
    println!("otp >>> {}", otp);
}
