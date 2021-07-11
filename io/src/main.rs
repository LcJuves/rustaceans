use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::{stdout, Read, Write};

fn main() -> std::io::Result<()> {
    let pwd = current_dir().unwrap();
    let path = pwd.join("Cargo.toml");
    let mut cargo_toml = OpenOptions::new().read(true).open(path)?;

    const BUF_SIZE: usize = 1024;
    let mut buf = [0u8; BUF_SIZE];
    loop {
        let read_len = cargo_toml.read(&mut buf)?;
        stdout().write_all(&buf[..read_len])?;
        stdout().flush()?;

        if read_len < buf.len() {
            break;
        }
    }
    Ok(())
}
