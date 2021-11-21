mod generator;
use generator::*;

fn main() {
    let key = rb32::decode("FHCIDHYW3N46EJBIQWOSP4VURTYIJ3W7".as_bytes());
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("timestamp >>> {}", timestamp);
    let counter = (timestamp / 1000 / 30) as u64;
    println!("counter >>> {}", counter);
    let digits = 6;
    let otp = generator::gen_otp(&key, counter, digits, HmacShaAlgorithm::SHA1);
    println!("otp >>> {}", otp);
}
