mod header;
mod index_router;
mod model;
mod request;
mod util;

use crate::index_router::*;
use crate::model::*;
use crate::request::{Request, RequestMethod};
use crate::util::*;

use std::fs::File;

use std::io::{Result, Write};
// #[cfg(any(target_os = "linux", target_os = "l4re"))]
// use std::net::Shutdown;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use std::time::UNIX_EPOCH;

fn main() -> Result<()> {
    init_serv("0.0.0.0:9999")?;
    Ok(())
}

fn init_serv(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Listening on http://{}", addr);
    println!();

    for stream in listener.incoming() {
        spawn(move || handle_conn(stream?));
    }

    Ok(())
}

fn handle_conn(mut stream: TcpStream) -> Result<()> {
    let request_lines = read_request_lines(&stream)?;
    let request_lines_string = String::from_utf8(request_lines).unwrap();
    println!("{}", request_lines_string);

    let first_line = request_lines_string.split(&LF.to_string()).collect::<Vec<_>>()[0];

    let first_line_infos: Vec<_> = first_line.split(&SP.to_string()).collect();
    let request = Request::new(
        first_line_infos[1].to_string(),
        match first_line_infos[0] {
            _ => RequestMethod::GET("GET"),
        },
        1.1,
    );

    // #[cfg(any(target_os = "linux", target_os = "l4re"))]
    // stream.shutdown(Shutdown::Read)?;

    let mut path_buf = std::env::current_dir()?;
    let mut is_index_page = false;
    let simple_request_uri = if request.uri.ends_with("/") {
        &request.uri[..(request.uri.rfind("/").unwrap_or(request.uri.len()))]
    } else {
        &request.uri
    };

    let paths: Vec<_> = simple_request_uri.split(*ROOT_ROUTER).collect();
    if *ROOT_ROUTER != simple_request_uri || include_index_router(simple_request_uri) {
        for path in paths {
            path_buf = path_buf.join(path);
        }
    } else if *ROOT_ROUTER == simple_request_uri {
        for index_router in DEFAULT_INDEX_ROUTERES.iter() {
            let should_join =
                (&index_router[(index_router.find('/').unwrap_or(0) + 1)..]).to_string();
            let should_join = &should_join[..(should_join.find('/').unwrap_or(should_join.len()))];
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
Content-Type: text/html;charset=utf-8\r\n",
            )?;
            write_server_line(&mut stream)?;

            let mut file_infos = Vec::<FileInfo>::new();

            for entry in (&path_buf).read_dir()? {
                let child_file_entry = entry?;
                let child_file_metadata = child_file_entry.metadata()?;

                let child_file_name_str =
                    child_file_entry.file_name().to_str().unwrap_or("Unknown").to_owned();

                let modified_time = child_file_metadata.modified()?;

                file_infos.push(FileInfo {
                    name: child_file_name_str.to_string(),
                    last_modified: modified_time.duration_since(UNIX_EPOCH).unwrap().as_millis(),
                    size: child_file_metadata.len(),
                    is_dir: child_file_metadata.is_dir(),
                });
            }

            let dir_name = simple_request_uri;
            let dir_view_html = gen_dir_view_html(&SERVER_NAME, dir_name, &file_infos);
            let dir_view_html_bytes = dir_view_html.as_bytes();

            stream.write(b"Content-Length: ")?;
            stream.write(dir_view_html_bytes.len().to_string().as_bytes())?;
            stream.write(b"\r\n\r\n")?;
            stream.write_all(dir_view_html_bytes)?;
            write_crlf(&mut stream)?;
        } else {
            stream.write(
                b"\
HTTP/1.1 200 OK\r\n\
Content-Type: ",
            )?;
            stream.write(&mime_bytes_from(&path_buf))?;
            stream.write(b"; charset=utf-8\r\n")?;
            write_server_line(&mut stream)?;
            stream.write(b"Content-Length: ")?;
            stream.write(path_buf_metadata.len().to_string().as_bytes())?;
            stream.write(b"\r\n\r\n")?;

            let mut file = File::open(&path_buf)?;
            transform_stream(&mut file, &mut stream)?;
            write_crlf(&mut stream)?;
        }
    } else {
        stream.write(
            b"\
HTTP/1.1 404 Not Found\r\n\
Content-Type: text/html;charset=utf-8\r\n",
        )?;
        write_server_line(&mut stream)?;

        let not_found_html =
            gen_not_found_html(&format!("{} [ 404 ]", &SERVER_NAME.to_owned()), &request.uri);
        let not_found_html_bytes = not_found_html.as_bytes();

        stream.write(b"Content-Length: ")?;
        stream.write(not_found_html_bytes.len().to_string().as_bytes())?;
        stream.write(b"\r\n\r\n")?;
        stream.write_all(not_found_html_bytes)?;
        write_crlf(&mut stream)?;
    }

    println!(">>>>>> Writed");
    println!();
    println!();

    stream.flush()?;

    // #[cfg(any(target_os = "linux", target_os = "l4re"))]
    // stream.shutdown(Shutdown::Write)?;

    Ok(())
}
