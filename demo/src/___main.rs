/**
 * Created at 2020/9/15 15:14.
 *
 * @author Liangcheng Juves
 */
use std::io::{stdout, Result, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let mut bar_out = ['\0'; 51];
    let bar_lab: &str = "-\\|/";
    let mut perc = 0;
    while perc <= 100 {
        print!(
            "\r {} \u{1b}[42m{}\u{1b}[0m [ {}% ] ",
            bar_lab.chars().nth(perc % 4).unwrap(),
            bar_out.iter().collect::<String>(),
            perc
        );
        stdout().flush()?;
        sleep(Duration::from_micros(60_000));
        bar_out[perc / 2] = ' ';
        perc += 1;
    }
    println!();
    Ok(())
}
