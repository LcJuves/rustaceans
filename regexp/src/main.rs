use regex::Regex;

const TO_SEARCH: &str = "
On 2010-03-14, foo happened. On 2014-10-14, bar happened.
";

fn main() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let re2 = Regex::new(r"foo[ ](happened)").unwrap();
    println!("To search: \"{}\"", TO_SEARCH);
    for caps in re.captures_iter(TO_SEARCH) {
        // Note that all of the unwraps are actually OK for this regex
        // because the only way for the regex to match is if all of the
        // capture groups match. This is not true in general though!
        println!(
            "year: {}, month: {}, day: {}",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(3).unwrap().as_str()
        );
    }

    if re2.is_match(TO_SEARCH) {
        println!();
        println!("=================================");
        println!(
            "\"{}\" \nreplaced by\n \"{}\", result is: \"{}\"",
            TO_SEARCH,
            re2,
            re2.replace_all(TO_SEARCH, "$1")
        );
        println!("=================================");
    }

    let re3 = Regex::new(
        r"(?x)
(?P<year>\d{4})  # the year
-
(?P<month>\d{2}) # the month
-
(?P<day>\d{2})   # the day
",
    )
    .unwrap();
    let caps = re3.captures("2010-03-14").unwrap();

    println!(
        "\nYear is: {}, Month is: {}, Day is: {}",
        &caps["year"], &caps["month"], &caps["day"]
    );
}
