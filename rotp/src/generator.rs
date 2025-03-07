use hmac::{Hmac, Mac, NewMac};
use sha1::Sha1;
use sha2::{Sha256, Sha512};

#[allow(dead_code)]
pub enum HmacShaAlgorithm {
    SHA1,
    SHA256,
    SHA512,
}

macro_rules! compute_digest_by_sha_type {
    ($sha_ty:ty, $key:ident, $input:ident) => {
        if let Ok(mut mac) = Hmac::<$sha_ty>::new_from_slice(&$key) {
            mac.update(&$input);
            return mac.finalize().into_bytes().as_slice().to_vec();
        }
    };
}

fn compute_digest(key: &[u8], input: &[u8], algorithm: HmacShaAlgorithm) -> Vec<u8> {
    match algorithm {
        HmacShaAlgorithm::SHA1 => compute_digest_by_sha_type!(Sha1, key, input),
        HmacShaAlgorithm::SHA256 => compute_digest_by_sha_type!(Sha256, key, input),
        HmacShaAlgorithm::SHA512 => compute_digest_by_sha_type!(Sha512, key, input),
    }
    Vec::<u8>::new()
}

pub fn gen_otp(
    secret: &[u8],
    moving_factor: u64,
    digits: usize,
    algorithm: HmacShaAlgorithm,
) -> String {
    let input = &moving_factor.to_be_bytes();
    let digest = compute_digest(secret, input, algorithm);

    let offset = (digest.last().unwrap() & 0xf) as usize;
    let binary = (((digest[offset] & 0x7f) as u32) << 24)
        | ((digest[offset + 1] as u32) << 16)
        | ((digest[offset + 2] as u32) << 8)
        | digest[offset + 3] as u32;

    let mut digest_power = 1u32;
    for _ in 0..digits {
        digest_power *= 10;
    }

    let otp = binary % digest_power;
    let mut otp = otp.to_string();
    while otp.len() < digits {
        otp = "0".to_owned() + &otp;
    }

    otp
}

#[allow(dead_code)]
pub fn gen_otp_from_time(
    secret: &[u8],
    time_millis: u128,
    period: u64,
    digits: usize,
    algorithm: HmacShaAlgorithm,
) -> String {
    const T0: u64 = 0;
    #[allow(non_snake_case)]
    let X: u64 = period;

    type T = u64;

    let time_secs = (time_millis / 1000) as u64;
    let time: T = (time_secs - T0) / X;

    gen_otp(secret, time, digits, algorithm)
}

#[inline]
#[allow(dead_code)]
pub fn gen_otp_from_counter(
    secret: &[u8],
    counter: u64,
    digits: usize,
    algorithm: HmacShaAlgorithm,
) -> String {
    gen_otp(secret, counter, digits, algorithm)
}

#[test]
fn test_gen_otp_sha1() {
    const T0: u64 = 0;
    const X: u64 = 30;

    type T = u64;

    let secret = rb32::decode("FHCIDHYW3N46EJBIQWOSP4VURTYIJ3W7".as_bytes());
    let digits = 6;

    let test_data_vec = vec![
        (1637501965720, 54583398, "767557"),
        (1637507332786, 54583577, "387753"),
        (1637507372913, 54583579, "725803"),
    ];

    for test_data in test_data_vec {
        let time_millis = test_data.0;
        let time_secs = time_millis / 1000;
        //////////////////////////////////////
        //////////////////////////////////////
        let time: T = (time_secs - T0) / X;
        assert_eq!(time, test_data.1);
        let otp = gen_otp(&secret, time, digits, HmacShaAlgorithm::SHA1);
        assert_eq!(otp, test_data.2);
    }
}
