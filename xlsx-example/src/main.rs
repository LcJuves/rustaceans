mod reflection;
mod robot_generator;
mod util;

// use crate::robot_generator::main::*;
use crate::robot_generator::tputil::*;

use crate::util::calamine_util::*;

use calamine::Error;
use qrcode::render::unicode;
use qrcode::QrCode;

fn main() -> Result<(), Error> {
    // robot_generator_main()?;

    let phone_num = "15211467428";
    let (ep_jwt_token_current, sessionid) =
        TOKIO_RT.block_on(sign_in_tp_by_sms(&phone_num)).unwrap();
    seeval!(ep_jwt_token_current);

    TOKIO_RT
        .block_on(req_api_v1_users_info(&ep_jwt_token_current, &sessionid))
        .unwrap();

    let code = QrCode::new("Liangcheng Juves").unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", image);

    Ok(())
}
