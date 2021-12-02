mod reflection;
mod robot_generator;
mod util;

// use crate::robot_generator::main::*;
use crate::robot_generator::tputil::*;

use crate::util::calamine_util::*;

use calamine::Error;

fn main() -> Result<(), Error> {
    // robot_generator_main()?;

    let phone_num = "15211467428";
    let ep_jwt_token_current = TOKIO_RT.block_on(sign_in_tp_by_sms(&phone_num)).unwrap();
    println!("ep_jwt_token_current >>> {}", ep_jwt_token_current);

    Ok(())
}
