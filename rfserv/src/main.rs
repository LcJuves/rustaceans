use std::fs::OpenOptions;

use std::io::Read;
use std::io::Result;
use std::os::unix::fs::PermissionsExt;

use std::io::Write;
use std::net::Shutdown;
use std::net::{TcpListener, TcpStream};

use std::thread::spawn;

mod header;
mod model;
mod request;

use model::*;

mod index_router;
use index_router::*;

use fmedia::MediaType;
use request::{Request, RequestMethod};

fn main() -> Result<()> {
    init_serv("0.0.0.0:9999")?;
    Ok(())
}

fn init_serv(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Listening on http://{}", addr);

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

    stream.shutdown(Shutdown::Read)?;

    let mut path_buf = std::env::current_dir()?;

    let mut is_index_page = false;

    let paths: Vec<_> = first_line_infos[1].split(*ROOT_ROUTER).collect();
    if *ROOT_ROUTER != &request.uri || include_index_router(&request.uri) {
        for path in paths {
            path_buf = path_buf.join(path);
        }
    } else if *ROOT_ROUTER == &request.uri {
        for index_router in DEFAULT_INDEX_ROUTERES.iter() {
            let should_join = &index_router[(index_router.rfind('/').unwrap() + 1)..];
            let check_path_buf = path_buf.join(should_join);
            if check_path_buf.exists() {
                is_index_page = true;
                path_buf = check_path_buf;
                break;
            }
        }
    }

    if is_index_page || path_buf.exists() {
        let path_buf_metadata = path_buf.metadata()?;

        if path_buf_metadata.is_dir() {
            stream.write(
                b"\
HTTP/1.1 200 OK\r\n\
Content-Type: text/html;charset=utf-8\r\n\
Server: Rust\r\n",
            )?;

            let mut dir_viewer_html = DIR_VIEWER_HTML.to_string();
            dir_viewer_html = dir_viewer_html.replace("${dirname}", &request.uri);

            // TODO: Empty directory html
            // TODO: Show 'Last modified'
            // TODO: Human file size

            let mut files_html = String::new();

            for entry in (&path_buf).read_dir()? {
                let child_file_entry = entry?;
                let child_file_metadata = child_file_entry.metadata()?;

                files_html.push_str("<tr>");
                if cfg!(unix) {
                    files_html.push_str(&format!(
                        "<td>{:o}</td>",
                        child_file_metadata.permissions().mode()
                    ));
                } else {
                    files_html.push_str(&format!("<td>{}</td>", "Unknown"));
                }
                files_html.push_str(&format!("<td>{}</td>", child_file_metadata.len()));
                let child_file_name = child_file_entry.file_name();
                let child_file_name_str = child_file_name.to_str().unwrap();

                let ref request_uri_ref = request.uri;

                files_html.push_str(&format!(
                    "<td><a href=\"javascript:void(0);\" onclick=\"window.location.href='{}';\">{}</a></td>",
                    format!("{}/{}", match request_uri_ref.ends_with("/") {
                        true => &request_uri_ref[0..request_uri_ref.rfind('/').unwrap()],
                        _=> &request_uri_ref
                    }, child_file_name_str), child_file_name_str
                ));
                files_html.push_str("</tr>");
            }

            dir_viewer_html = dir_viewer_html.replace("${files}", &files_html);

            let dir_viewer_html_bytes = dir_viewer_html.as_bytes();

            stream.write(b"Content-Length: ")?;
            stream.write(dir_viewer_html_bytes.len().to_string().as_bytes())?;
            stream.write(b"\r\n\r\n")?;
            stream.write_all(dir_viewer_html_bytes)?;
        } else {
            let media_ty = match path_buf.extension() {
                Some(extension) => MediaType::from_file_extension(extension.to_str().unwrap()),
                _ => None,
            };

            stream.write(
                b"\
HTTP/1.1 200 OK\r\n\
Content-Type: ",
            )?;
            match media_ty {
                Some(mty) => {
                    stream.write(mty.as_bytes())?;
                }
                _ => {
                    stream.write(b"text/plain")?;
                }
            }
            stream.write(b"; charset=utf-8\r\n")?;
            stream.write(b"Server: Rust\r\n")?;
            stream.write(b"Content-Length: ")?;
            stream.write(path_buf_metadata.len().to_string().as_bytes())?;
            stream.write(b"\r\n\r\n")?;

            let mut file = OpenOptions::new().read(true).open(path_buf)?;

            std::io::copy(&mut file, &mut stream)?;
        }
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
    }

    println!();
    println!(">>>>>> Writed");
    println!();

    stream.flush()?;
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
        stream.read(&mut buf)?;
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
