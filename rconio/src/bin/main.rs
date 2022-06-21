include!("../lib.rs");

fn main() {
    #[cfg(windows)]
    {
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

        reset();
        set_under_line();
        println!("UNDER_LINE");

        reset();

        print(&format!("{}我的世界{}", SGR_LIGHT_RED, SGR_NONE));

        std::thread::sleep(std::time::Duration::from_millis(3000));
        clear_screen();
    }
}
