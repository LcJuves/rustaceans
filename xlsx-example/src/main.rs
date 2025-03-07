mod reflection;
mod robot_generator;
#[macro_use]
mod util;
#[macro_use]
mod macros;

#[allow(unused_imports)]
use crate::robot_generator::main::*;
#[allow(unused_imports)]
use crate::robot_generator::tputil::*;
#[allow(unused_imports)]
use crate::util::calamine::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // #[cfg(not(debug_assertions))]
    robot_generator_main()?;

    /* #[cfg(debug_assertions)]
    {
        // let (ep_jwt_token_current, sessionid, username, email, staff_code, token) =
        //     future_block_on!(sign_in_tp_by_sms())?;

        // let (ep_jwt_token_current, sessionid, username, email, staff_code, token) =
        //     future_block_on!(sign_in_tp_by_scan_moa_qrcode())?;
        // seeval!((&ep_jwt_token_current, &sessionid, &username, &email, &staff_code, &token));

        use crate::robot_generator::upgrade::*;

        let args_string = std::env::args().collect::<String>();
        if args_string.contains("-V") {
            println!("{}", get_cargo_toml_version()?);
            return Ok(());
        }

        // self_upgrade()?;
        // add_me_to_path_var()?;
        set_alias("r-man")?;
    } */

    Ok(())
}
