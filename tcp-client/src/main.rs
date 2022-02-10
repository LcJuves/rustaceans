use std::io::{stdin, BufRead, BufReader, Result, Write};
use std::net::TcpStream;

fn main() -> Result<()> {
    let mut tcp_stream = TcpStream::connect("127.0.0.1:10808")?;
    println!("Please input content to send: ");
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    tcp_stream.write_all(input.as_bytes())?;
    tcp_stream.flush()?;

    let mut server_back = Vec::<u8>::new();
    BufReader::new(&tcp_stream).read_until(b'\n', &mut server_back)?;

    println!("Server callbacked data is: {}", String::from_utf8_lossy(server_back.as_slice()));

    Ok(())
}
