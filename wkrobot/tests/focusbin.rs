use std::process::{Command, Stdio};
use std::thread;

fn main() {
    thread::spawn(|| {
        Command::new("cmd")
            .arg("/k")
            .arg("start /min /wait /high cmd && ping 127.0.0.1 -n 3 -w 1000 > nul")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();
    });
}
