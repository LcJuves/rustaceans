use std::io::{stdin, stdout, BufRead, Result, Write};

fn main() -> Result<()> {
    let mut input = String::new();

    stdout().write(b"> ")?;
    stdout().flush()?;

    stdin().lock().read_line(&mut input)?;

    let output = &input[..input
        .rfind("\r")
        .unwrap_or(input.rfind("\n").unwrap_or(input.len()))];
    print!("{}", output);

    Ok(())
}
