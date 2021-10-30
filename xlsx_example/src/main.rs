mod reflection;
mod robot_generator;
mod util;

use crate::robot_generator::main::*;

use calamine::Error;

fn main() -> Result<(), Error> {
    robot_generator_main()?;
    Ok(())
}
