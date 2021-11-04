use std::env::args;
use std::io::{stdin, BufRead, Result};

fn grep(target: &str) -> Result<()> {
    for line_ret in stdin().lock().lines() {
        let line = line_ret?;
        if line.contains(target) {
            if args().len() >= 3 && "--colored" == &args().nth(2).unwrap() {
                let line = line.replace(target, "");
                println!("\u{1b}[31m{}\u{1b}[0m{}", target, line);
            } else {
                println!("{}", line);
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    grep(&args().nth(1).unwrap())?;
    Ok(())
}
