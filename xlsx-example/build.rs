// build.rs

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET")?;
    println!("cargo:rustc-env=TARGET={}", target);

    // let profile = env::var("PROFILE")?;
    // println!("cargo:rustc-env=PROFILE={}", profile);

    // let cargo_encoded_rustflags = env::var("CARGO_ENCODED_RUSTFLAGS")?;
    // println!("cargo:rustc-env=CARGO_ENCODED_RUSTFLAGS={}", cargo_encoded_rustflags);

    // let rustflags = env::var("RUSTFLAGS")?;
    // println!("cargo:rustc-env=RUSTFLAGS={}", rustflags);

    // let cargo_build_rustflags = env::var("CARGO_BUILD_RUSTFLAGS")?;
    // println!("cargo:rustc-env=CARGO_BUILD_RUSTFLAGS={}", cargo_build_rustflags);

    /* if profile == "release" {
        if target == "x86_64-pc-windows-gnu" {
            let rustflags = "-Clink-arg=-s";
            println!("cargo:rustc-env=CARGO_ENCODED_RUSTFLAGS={}", rustflags);
            env::set_var("CARGO_ENCODED_RUSTFLAGS", rustflags);
        } else if target == "x86_64-pc-windows-msvc" {
            let rustflags = "-Clink-arg=/DEBUG:NONE";
            println!("cargo:rustc-env=CARGO_ENCODED_RUSTFLAGS={}", rustflags);
            env::set_var("CARGO_ENCODED_RUSTFLAGS", rustflags);
        }
    } */

    // println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
