use ssri::{Algorithm, IntegrityOpts};

pub(crate) fn compute(bytes: &[u8]) -> String {
    let integrity = IntegrityOpts::new().algorithm(Algorithm::Sha512).chain(bytes).result();
    let (_, hex) = integrity.to_hex();
    hex
}
