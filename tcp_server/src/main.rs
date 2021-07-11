use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::net::TcpListener;

fn echo_main(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening on {}", addr);
    loop {
        let (mut tcp_stream, socket_addr) = listener.accept()?;
        println!("connection recevied from {}", socket_addr);
        let mut out_stream = tcp_stream.try_clone()?;
        std::thread::spawn(move || {
            let mut ret = Vec::<u8>::new();
            let mut buf = [0u8; 1024];
            loop {
                let len = match tcp_stream.read(&mut buf) {
                    Ok(0) => {
                        println!("recevied data is: {}", String::from_utf8_lossy(&ret));
                        println!("connection closed");
                        return Ok(());
                    }
                    Ok(len) => len,
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e),
                };
                for b in &buf[..len] {
                    ret.push(*b);
                }
                out_stream.write_all(&buf[..len]).expect("error");
                out_stream.flush().expect("error");
            }
        });
    }
}

fn main() -> Result<()> {
    echo_main("127.0.0.1:10808")?;
    Ok(())
}
