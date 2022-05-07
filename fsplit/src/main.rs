pub(crate) fn compute_sha512sum(bytes: &[u8]) -> String {
    let integrity = IntegrityOpts::new().algorithm(Algorithm::Sha512).chain(bytes).result();
    let (_, hex) = integrity.to_hex();
    hex
}

fn main() {
    println!("Hello, world!");
}
