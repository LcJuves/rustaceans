include!("../lib.rs");

fn main() {
    set_red();
    println!("RED");
    set_green();
    println!("GREEN");
    set_blue();
    println!("BLUE");
    reset();
    println!("RESET");
    set_high_light();
    println!("HIGH_LIGHT");
    set_red();
    set_high_light();
    println!("RED");
    set_green();
    set_high_light();
    println!("GREEN");
    set_blue();
    set_high_light();
    println!("BLUE");
    set_under_line();
    println!("UNDER_LINE");

    reset();

    // clear_screen();
}
