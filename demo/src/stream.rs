use std::env::current_dir;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Read, Result};

pub const BUF_SIZE: usize = 1024;

/// # Examples
///
/// ```
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     let mut cargo_toml = open_cargo_toml()?;
///     let mut read_listener = |bytes: &[u8], finished: bool| {
///         if !finished {
///             print!("{}", String::from_utf8_lossy(bytes));
///         } else {
///             const SIGN: &str = "========";
///             println!();
///             println!("{} finished {}", SIGN, SIGN);
///             println!();
///             print!("{}", String::from_utf8_lossy(bytes));
///         }
///     };
///     closure_read_stream(&mut cargo_toml, &mut read_listener)?;
///     Ok(())
/// }
/// ```
#[allow(dead_code)]
pub fn closure_read_stream<R, F>(reader: &mut R, read_callback: &mut F) -> Result<()>
where
    R: Read + ?Sized,
    F: FnMut(&[u8], bool),
{
    let mut readied = Vec::<u8>::new();
    let mut buf = [0u8; BUF_SIZE];

    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => {
                read_callback(&readied, true);
                return Ok(());
            }
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        readied.clear();
        for b in &buf[..len] {
            readied.push(*b);
        }
        read_callback(&buf[..len], false);
    }
}

pub trait ReadListener {
    fn on_reading(&self, bytes: &[u8], finished: bool);
}

/// # Examples
///
/// ```
/// use std::io;
///
/// pub struct ReadListenerImpl;
/// impl ReadListenerImpl {
///     pub fn new() -> ReadListenerImpl {
///         Self
///     }
/// }
///
/// impl ReadListener for ReadListenerImpl {
///     fn on_reading(&self, bytes: &[u8], finished: bool) {
///         if !finished {
///             print!("{}", String::from_utf8_lossy(bytes));
///         } else {
///             const SIGN: &str = "========";
///             println!();
///             println!("{} finished {}", SIGN, SIGN);
///             println!();
///             print!("{}", String::from_utf8_lossy(bytes));
///         }
///     }
/// }
///
/// fn main() -> io::Result<()> {
///     let mut cargo_toml = open_cargo_toml()?;
///     read_stream(&mut cargo_toml, &mut ReadListenerImpl::new())?;
///     Ok(())
/// }
/// ```
#[allow(dead_code)]
pub fn read_stream<R: Read + ?Sized, RL: ReadListener>(
    reader: &mut R,
    read_listener: &mut RL,
) -> Result<()> {
    let mut closure = |bytes: &[u8], finished: bool| read_listener.on_reading(&bytes, finished);
    closure_read_stream(reader, &mut closure)?;
    Ok(())
}

/// Can be used to open `Cargo.toml` under `Package`
#[allow(dead_code)]
pub fn open_cargo_toml() -> Result<File> {
    let pwd = current_dir().unwrap();
    let path = pwd.join("Cargo.toml");
    let cargo_toml = OpenOptions::new().read(true).open(path)?;
    Ok(cargo_toml)
}
