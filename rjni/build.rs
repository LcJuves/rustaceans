use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let profile = env::var("PROFILE")?;
    println!("cargo:rustc-env=PROFILE={}", profile);

    let out_dir = env::var("OUT_DIR")?;
    println!("cargo:rustc-env=OUT_DIR={}", out_dir);
    Ok(())
}
