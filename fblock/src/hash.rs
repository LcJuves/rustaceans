use cityhash_sys::city_hash_128;
use rb64::encode;

pub(crate) fn compute(bytes: &[u8]) -> String {
    let city_hash_u128 = city_hash_128(bytes);
    let hash_be_bytes = city_hash_u128.to_be_bytes();
    let hash_le_bytes = city_hash_u128.to_le_bytes();
    let mut hash_vec = hash_be_bytes.to_vec();
    hash_vec.extend_from_slice(&hash_le_bytes);
    let rb64_bytes = encode(&hash_vec, false, false, false);
    String::from_utf8_lossy(&rb64_bytes).to_string()
}
