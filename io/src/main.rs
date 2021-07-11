use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::{stdout, ErrorKind, Read, Write};

fn main() -> std::io::Result<()> {
    let pwd = current_dir().unwrap();
    let path = pwd.join("Cargo.toml");
    let mut cargo_toml = OpenOptions::new().read(true).open(path)?;

    const BUF_SIZE: usize = 1024;
    let mut buf = [0u8; BUF_SIZE];
    loop {
        let len = match cargo_toml.read(&mut buf) {
            Ok(0) => return Ok(()),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        stdout().write_all(&buf[..len])?;
        stdout().flush()?;
    }
}
