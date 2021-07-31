use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::net::Shutdown;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread::spawn;

mod header;
mod model;
mod request;

use model::*;

use request::{Request, RequestMethod};
use fmedia::MediaType;

fn main() -> Result<()> {
    init_serv("0.0.0.0:9999")?;
    Ok(())
}

fn init_serv(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Listening on {}", addr);

    for stream in listener.incoming() {
        spawn(move || handle_conn(stream?));
    }

    Ok(())
}

fn handle_conn(mut stream: TcpStream) -> Result<()> {
    let request_lines = read_request_lines(&stream)?;
    let request_lines_string = String::from_utf8(request_lines).unwrap();
    println!("{}", request_lines_string);

    let first_line = request_lines_string
        .split(&LF.to_string())
        .collect::<Vec<_>>()[0];

    println!("First line: {}", first_line);

    let first_line_infos: Vec<_> = first_line.split(&SP.to_string()).collect();
    let request = Request::new(
        first_line_infos[1].to_string(),
        match first_line_infos[0] {
            _ => RequestMethod::GET("GET"),
        },
        1.1,
    );
    println!("Request: {:?}", request);

    stream.shutdown(Shutdown::Read)?;

    let paths: Vec<_> = first_line_infos[1].split("/").collect();
    println!("paths: {:?}", paths);
    let mut path_buf = PathBuf::new();
    for path in paths {
        path_buf = path_buf.join(path);
    }
    println!("path_buf: {:?}", path_buf);

    if false {
        // let mut file = OpenOptions::new().read(true).open(path_buf).unwrap();
    } else {
        stream.write(
            b"\
HTTP/1.1 404 Not Found\r\n\
Content-Type: text/html;charset=utf-8\r\n\
Server: Rust\r\n",
        )?;

        let mut not_found_temp_html = NOT_FOUND_TEMP_HTML.to_string();
        not_found_temp_html = not_found_temp_html.replace("{}", &request.uri);

        let not_found_temp_html_bytes = not_found_temp_html.as_bytes();

        stream.write(b"Content-Length: ")?;
        stream.write(not_found_temp_html_bytes.len().to_string().as_bytes())?;
        stream.write(b"\r\n\r\n")?;
        stream.write_all(not_found_temp_html_bytes)?;

        stream.flush()?;
    }

    println!();
    println!(">>>>>> Writed");
    println!();

    stream.shutdown(Shutdown::Write)?;

    Ok(())
}

fn read_request_lines(mut stream: &TcpStream) -> Result<Vec<u8>> {
    let mut request_lines = Vec::<u8>::new();

    let mut buf = [0u8];

    fn write_and_flush<W>(w: &mut W, bytes: &[u8]) -> Result<()>
    where
        W: Write,
    {
        w.write(bytes)?;
        w.flush()?;
        Ok(())
    }

    loop {
        let _ = match stream.read(&mut buf) {
            Ok(0) => {
                return Ok(request_lines);
            }
            len @ Ok(_) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        write_and_flush(&mut request_lines, &buf)?;

        if b'\n' == buf[0] {
            let mut buf_tmp = [0u8];
            stream.read(&mut buf_tmp)?;

            if b'\r' == buf_tmp[0] || b'\n' == buf_tmp[0] {
                return Ok(request_lines);
            } else {
                write_and_flush(&mut request_lines, &buf_tmp)?;
            }
        }
    }
}
