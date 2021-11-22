use hmac::{Hmac, Mac, NewMac};
use hmacsha1::hmac_sha1;
use sha2::{Sha256, Sha512};

#[allow(dead_code)]
pub enum HmacShaAlgorithm {
    SHA1,
    SHA256,
    SHA512,
}

fn hmac_sha(key: &[u8], input: &[u8], algorithm: HmacShaAlgorithm) -> Vec<u8> {
    match algorithm {
        HmacShaAlgorithm::SHA1 => {
            return hmac_sha1(&key, &input).iter().cloned().collect::<Vec<u8>>();
        }
        HmacShaAlgorithm::SHA256 => {
            type HmacSha256 = Hmac<Sha256>;
            if let Ok(mut mac) = HmacSha256::new_from_slice(&key) {
                mac.update(&input);
                return mac
                    .finalize()
                    .into_bytes()
                    .iter()
                    .cloned()
                    .collect::<Vec<u8>>();
            }
        }
        HmacShaAlgorithm::SHA512 => {
            type HmacSha512 = Hmac<Sha512>;
            if let Ok(mut mac) = HmacSha512::new_from_slice(&key) {
                mac.update(&input);
                return mac
                    .finalize()
                    .into_bytes()
                    .iter()
                    .cloned()
                    .collect::<Vec<u8>>();
            }
        }
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
    let digest = hmac_sha(secret, input, algorithm);

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
