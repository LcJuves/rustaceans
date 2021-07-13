use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Read, Result};

trait ReadListener {
    fn on_reading(&self, bytes: &[u8], finished: bool);
}

fn read_stream<R: Read + ?Sized, RL: ReadListener>(
    reader: &mut R,
    read_listener: &mut RL,
) -> Result<()> {
    const BUF_SIZE: usize = 1024;
    let mut readed = Vec::<u8>::new();
    let mut buf = [0u8; BUF_SIZE];
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => {
                read_listener.on_reading(&readed, true);
                return Ok(());
            }
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        readed.clear();
        for b in &buf[..len] {
            readed.push(*b);
        }
        read_listener.on_reading(&buf[..len], false);
    }
}

struct ReadListenerImpl;

impl ReadListenerImpl {
    fn new() -> ReadListenerImpl {
        Self
    }
}

impl ReadListener for ReadListenerImpl {
    fn on_reading(&self, bytes: &[u8], finished: bool) {
        if !finished {
            print!("{}", String::from_utf8_lossy(bytes));
        } else {
            const SIGN: &str = "========";
            println!();
            println!("{} finished {}", SIGN, SIGN);
            println!();
            print!("{}", String::from_utf8_lossy(bytes));
        }
    }
}

fn main() -> Result<()> {
    let pwd = current_dir().unwrap();
    let path = pwd.join("Cargo.toml");
    let mut cargo_toml = OpenOptions::new().read(true).open(path)?;
    read_stream(&mut cargo_toml, &mut ReadListenerImpl::new())?;
    Ok(())
}
