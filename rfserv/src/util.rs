use crate::model::*;

use std::io::{Read, Result, Write};
use std::net::TcpStream;
use std::path::PathBuf;

use fmedia::MediaType;

pub(crate) fn gen_dir_view_html(
    serv_name: &str,
    dir_name: &str,
    file_infos: &Vec<FileInfo>,
) -> String {
    let mut html = String::new();
    html.push_str(&DIR_VIEWER_HEML_PART1);
    html.push_str(serv_name);
    html.push_str(&DIR_VIEWER_HEML_PART2);
    html.push_str(&COMMON_CSS);
    html.push_str(&DIR_VIEWER_HEML_PART3);
    html.push_str(&format!("let dirname='{}';", dir_name));
    let files = (|| {
        let mut json = String::new();
        json.push_str("[");
        for file_info in file_infos {
            json.push_str(&format!(
                r#"["{}",{},{},{}],"#,
                file_info.name, file_info.last_modified, file_info.size, file_info.is_dir,
            ));
        }
        json = (&json[..(json.rfind(",").unwrap_or(json.len()))]).to_string();
        json.push_str("]");
        json
    })();
    html.push_str(&format!("let files={};", files));
    html.push_str(&DIR_VIEWER_JS);
    html.push_str(&DIR_VIEWER_HEML_PART4);
    html
}

pub(crate) fn read_request_lines(mut stream: &TcpStream) -> Result<Vec<u8>> {
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

pub(crate) fn mime_bytes_from(path_buf: &PathBuf) -> Vec<u8> {
    let media_ty = match path_buf.extension() {
        Some(extension) => MediaType::from_file_extension(extension.to_str().unwrap()),
        _ => None,
    };
    if let Some(mty) = media_ty {
        mty.as_bytes().to_vec()
    } else {
        b"text/plain".to_vec()
    }
}

pub(crate) fn write_crlf<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    w.write(&CRLF.as_bytes())?;
    Ok(())
}

pub(crate) fn write_server_line<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    w.write(b"Server: ")?;
    w.write(&SERVER_NAME.as_bytes())?;
    write_crlf(w)?;
    Ok(())
}