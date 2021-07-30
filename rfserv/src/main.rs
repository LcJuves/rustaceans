use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;
use std::io::Write;
use std::net::Shutdown;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

// pub const CRLF: &str = "\r\n";

mod header;
mod request;

use request::{Request, RequestMethod};

fn main() -> Result<()> {
    init_serv("0.0.0.0:9999")?;
    Ok(())
}

fn init_serv(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Listening on {}", addr);
    loop {
        let (tcp_stream, socket_addr) = listener.accept()?;
        spawn(move || {
            println!("Connection recevied from {}", socket_addr);
            let out_stream = tcp_stream.try_clone()?;
            handle_conn(tcp_stream, out_stream)
        });
    }
}

fn handle_conn(in_stream: TcpStream, mut out_stream: TcpStream) -> Result<()> {
    let request_lines = read_request_lines(in_stream).unwrap();
    let request_lines_string = String::from_utf8(request_lines).unwrap();
    println!("{}", request_lines_string);

    let first_line = request_lines_string.split("\r\n").collect::<Vec<_>>()[0];

    println!("First line: {}", first_line);

    let first_line_infos: Vec<_> = first_line.split(" ").collect();
    let request = Request::new(
        first_line_infos[1].to_string(),
        match first_line_infos[0] {
            _ => RequestMethod::GET("GET"),
        },
        1.1,
    );
    println!("Request: {:?}", request);

    /* let paths: Vec<_> = first_line_infos[1].split("/").collect();
    println!("paths: {:?}", paths);
    let mut path_buf = PathBuf::new();
    for path in paths {
        path_buf = path_buf.join(path);
    }
    println!("path_buf: {:?}", path_buf); */

    // out_stream.shutdown(Shutdown::Read)?;

    // in_stream.shutdown(Shutdown::Read)?;

    /* if false {
        // let mut file = OpenOptions::new().read(true).open(path_buf).unwrap();
    } else { */
    // out_stream.write_all(b"HTTP/1.1 404 Not Found\r\n")?;
    // out_stream.write_all(b"Content-Type: text/html;charset=utf-8\r\n")?;

    out_stream.write_all(b"HTTP/1.1 404 Not Found\r\n\
Content-Type: text/plain;charset=utf-8\r\n\r\n\
404\r\n\r\n")?;

    // let mut not_found_temp_html = include_str!("not_found_temp.html").to_string();
    // not_found_temp_html = not_found_temp_html.replace("{}", &request.uri);

    // out_stream.write_all(b"Content-Length: ")?;
    // out_stream.write_all(format!("{}", not_found_temp_html.as_bytes().len()).as_bytes())?;
    // out_stream.write_all(b"\r\n\r\n")?;

    // out_stream.write_all(b"404")?;
    // out_stream.write_all(b"\r\n\r\n")?;
    out_stream.flush()?;
    /* } */

    println!();
    println!(">>>>>> Writed");
    println!();

    out_stream.shutdown(Shutdown::Write)?;

    Ok(())
}

fn read_request_lines(stream: TcpStream) -> Result<Vec<u8>> {
    let mut request_lines = Vec::<u8>::new();
    let mut buf_reader = BufReader::new(&stream);
    loop {
        buf_reader.read_until(b'\n', &mut request_lines)?;
        let mut check_next_line = Vec::<u8>::new();
        buf_reader.read_until(b'\n', &mut check_next_line)?;
        if check_next_line.len() <= 2 {
            stream.shutdown(Shutdown::Read)?;
            break;
        } else {
            request_lines.write_all(&check_next_line)?;
            request_lines.flush()?;
        }
    }
    Ok(request_lines)
}
