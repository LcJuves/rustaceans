use std::io::{stdout, Result, Write};

/// # Examples
///
/// ```
/// use std::io;
/// use std::thread::sleep;
/// use std::time::Duration;
///
/// fn main() -> io::Result<()> {
///     for perc in 0..=100 {
///         draw(perc)?;
///         sleep(Duration::from_micros(60_000));
///     }
///     println!();
///     Ok(())
/// }
/// ```
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
