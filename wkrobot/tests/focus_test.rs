include!("../src/lib.rs");

use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

fn _compile_focus_exe() -> Result<PathBuf, std::io::Error> {
    let cargo_manifest_dir_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let focusbin_path = cargo_manifest_dir_path.join("tests").join("focusbin.rs");
    let mut focus_archive_path = cargo_manifest_dir_path.join("tests");

    if cfg!(windows) {
        focus_archive_path.push("focusbin.exe");
    } else {
        focus_archive_path.push("focusbin");
    }

    let rustc_focusbin_cmd_status = Command::new("rustc")
        .arg(focusbin_path)
        .arg("-o")
        .arg(&focus_archive_path)
        .status()?;
    assert!(rustc_focusbin_cmd_status.success());
    Ok(focus_archive_path)
}

#[test]
fn test_focus_window() -> Result<(), Box<dyn Error>> {
    // FIXME
    // TODO

    let focus_archive_path = _compile_focus_exe()?;

    // let mut focusbin_child = Command::new(&focus_archive_path)
    //     .stdin(Stdio::null())
    //     .stdout(Stdio::null())
    //     .stderr(Stdio::null())
    //     .spawn()?;
    // focusbin_child.wait()?;

    // focus_window(r"C:\WINDOWS\system32\cmd.exe")?;

    std::fs::remove_file(&focus_archive_path)?;
    Ok(())
}
