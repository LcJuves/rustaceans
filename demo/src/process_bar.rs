use std::io::{stdout, Result, Write};

#[allow(dead_code)]
pub fn draw(perc: usize) -> Result<()> {
    const BAR_LAB: &str = "-\\|/";
    if perc > 100 {
        panic!("Cannot be greater than 100");
    }
    print!(
        "\r {} \u{1b}[42m{}\u{1b}[0m [ {}% ] ",
        BAR_LAB.chars().nth(perc % 4).unwrap(),
        " ".repeat(perc / 2),
        perc
    );
    stdout().flush()?;
    Ok(())
}
