#![cfg(windows)]

use std::path::{Path, PathBuf};
use std::process::Command;

macro_rules! _read_child_output_string {
    ($child:ident) => {
        (|| -> Result<String, Box<dyn Error>> {
            let mut child_stdout = $child.stdout.take().unwrap();
            let mut output_string = String::new();
            let mut first_space = false;
            let mut buf = [0u8; 1];
            (|| -> Result<(), std::io::Error> {
                loop {
                    let _ = match child_stdout.read(&mut buf) {
                        Ok(0) => return Ok(()),
                        Ok(len) => len,
                        Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                        Err(e) => return Err(e),
                    };
                    let buf_str = std::str::from_utf8(&buf).unwrap_or("");
                    if buf_str == ">" || (buf_str == " " && !first_space) {
                        if buf_str == " " {
                            first_space = true;
                        }
                        stdout().write_all(&buf)?;
                        stdout().flush()?;
                    } else {
                        output_string.push_str(buf_str);
                    }
                }
            })()?;
            Ok(output_string)
        })()?
    };
}

pub(crate) fn _compile_kbd_exe() -> Result<PathBuf, std::io::Error> {
    let cargo_manifest_dir_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let kbdbin_path = cargo_manifest_dir_path.join("tests").join("kbdbin.rs");
    let mut kbd_archive_path = cargo_manifest_dir_path.join("tests");

    if cfg!(windows) {
        kbd_archive_path.push("kbdbin.exe");
    } else {
        kbd_archive_path.push("kbdbin");
    }

    let rustc_kbdbin_cmd_status =
        Command::new("rustc").arg(kbdbin_path).arg("-o").arg(&kbd_archive_path).status()?;
    assert!(rustc_kbdbin_cmd_status.success());
    Ok(kbd_archive_path)
}
