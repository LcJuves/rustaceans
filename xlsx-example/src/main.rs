mod common_util;
mod reflection;
mod robot_generator;
mod util;

#[allow(unused_imports)]
use crate::robot_generator::main::*;
use crate::robot_generator::tputil::*;

use crate::util::calamine_util::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(not(debug_assertions))]
    robot_generator_main()?;

    #[cfg(debug_assertions)]
    {
        // let phone_num = "15211467428";
        // let (ep_jwt_token_current, sessionid, username, email, staff_code, token) =
        //     TOKIO_RT.block_on(sign_in_tp_by_sms(&phone_num))?;

        let (ep_jwt_token_current, sessionid, username, email, staff_code, token) =
            TOKIO_RT.block_on(sign_in_tp_by_scan_moa_arcode())?;
        seeval!((&ep_jwt_token_current, &sessionid, &username, &email, &staff_code, &token));
    }

    Ok(())
}
