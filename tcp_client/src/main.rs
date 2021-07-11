use std::io::BufRead;
use std::io::BufReader;

use std::io::Write;
use std::io::{stdin, Result};
use std::net::TcpStream;

fn main() -> Result<()> {
    let mut tcp_stream = TcpStream::connect("127.0.0.1:10808")?;
    println!("Please input content to send: ");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read string from stdin");

    tcp_stream
        .write_all(input.as_bytes())
        .expect("Failed write stream to connection");

    let mut reader = BufReader::new(&tcp_stream);
    let mut server_back: Vec<u8> = Vec::new();
    reader
        .read_until(b'\n', &mut server_back)
        .expect("Could not read into buffer");

    println!(
        "Server callbacked data is: {}",
        String::from_utf8_lossy(server_back.as_slice())
    );

    Ok(())
}
