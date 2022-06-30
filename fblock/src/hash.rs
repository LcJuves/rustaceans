use cityhash_sys::city_hash_128;
use rb64::encode;

pub(crate) fn compute(bytes: &[u8]) -> String {
    let hash_bytes = city_hash_128(bytes).to_be_bytes();
    let rb64_bytes = encode(&hash_bytes, false, false, false);
    String::from_utf8_lossy(&rb64_bytes).to_string()
}
