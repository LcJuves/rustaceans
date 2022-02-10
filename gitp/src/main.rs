use std::env::current_dir;
use std::io::{stdin, stdout, BufRead, Error, Write};
use std::process::{Command, Stdio};

use chrono::prelude::*;

fn main() -> Result<(), Error> {
    #[allow(unused_assignments)]
    let mut commit_author = String::new();
    // TODO: ...
    commit_author = "Liangcheng Juves <liangchengj@outlook.com>".to_string();

    let base_dir = current_dir().unwrap();
    if base_dir.join("Cargo.toml").exists() {
        assert!(Command::new("cargo")
            .arg("fmt")
            .current_dir(&base_dir)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap()
            .success());
        dbg!(&base_dir);
    }

    let local: DateTime<Local> = Local::now();
    let local_dt_string = local.format("%Y/%m/%d %H:%M").to_string();
    dbg!(&local_dt_string);

    // git -C "${base_dir}" add "${base_dir}"
    assert!(Command::new("git")
        .arg("-C")
        .arg(&base_dir)
        .arg("add")
        .arg(&base_dir)
        .status()
        .unwrap()
        .success());

    let mut commit_msg = String::new();

    stdout().write(b"Please enter your commit message: ")?;
    stdout().flush()?;
    stdin().lock().read_line(&mut commit_msg)?;

    commit_msg = (&commit_msg
        [0..(commit_msg.rfind("\r").unwrap_or(commit_msg.rfind("\n").unwrap()))])
        .to_owned();

    // git -C "${base_dir}" commit -m "Updated at ${tm}" --author "Liangcheng Juves <liangchengj@outlook.com>"
    if commit_msg.is_empty() {
        commit_msg = format!("Updated at {local_dt_string}");
    }
    assert!(Command::new("git")
        .arg("-C")
        .arg(&base_dir)
        .arg("commit")
        .arg("-m")
        .arg(&commit_msg)
        .arg("--author")
        .arg(&commit_author)
        .status()
        .unwrap()
        .success());

    // git -C "${base_dir}" push -u origin main
    assert!(Command::new("git")
        .arg("-C")
        .arg(&base_dir)
        .arg("push")
        .arg("-u")
        .arg("origin")
        .arg("main")
        .status()
        .unwrap()
        .success());

    Ok(())
}
