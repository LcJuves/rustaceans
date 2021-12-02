mod reflection;
mod robot_generator;
mod util;

// use crate::robot_generator::main::*;
use crate::robot_generator::tputil::*;

use crate::util::calamine_util::*;

use calamine::Error;

fn main() -> Result<(), Error> {
    // robot_generator_main()?;

    let login_redirect_url_and_session_id = TOKIO_RT
        .block_on(get_login_redirect_url_and_session_id())
        .unwrap();

    println!(
        "login_redirect_url_and_session_id >>> {:?}",
        login_redirect_url_and_session_id
    );

    let authorize_redirect_url_and_session_id = TOKIO_RT
        .block_on(get_authorize_redirect_url_and_session_id(
            &login_redirect_url_and_session_id.0,
            &login_redirect_url_and_session_id.1,
        ))
        .unwrap();

    println!(
        "authorize_redirect_url_and_session_id >>> {:?}",
        authorize_redirect_url_and_session_id
    );

    let sign_in_client_id_and_response_type = TOKIO_RT
        .block_on(get_sign_in_client_id_and_response_type(
            &authorize_redirect_url_and_session_id.0,
            &authorize_redirect_url_and_session_id.1,
        ))
        .unwrap();

    println!(
        "sign_in_client_id_and_response_type >>> {:?}",
        sign_in_client_id_and_response_type
    );

    let userauth_should_tokens_and_session_id = TOKIO_RT
        .block_on(get_userauth_should_tokens_and_session_id(
            &sign_in_client_id_and_response_type.0,
            &sign_in_client_id_and_response_type.1,
        ))
        .unwrap();

    println!(
        "userauth_should_tokens_and_session_id >>> {:?}",
        userauth_should_tokens_and_session_id
    );

    let phone_num = "15211467428";
    let user_name = TOKIO_RT
        .block_on(send_sms_and_get_user_name(
            &userauth_should_tokens_and_session_id.0,
            &userauth_should_tokens_and_session_id.1,
            &userauth_should_tokens_and_session_id.2,
            phone_num,
        ))
        .unwrap();

    println!("user_name >>> {}", user_name);

    let sms_code = read_stdin_sms_code()?;
    TOKIO_RT
        .block_on(verify_sms_code(
            &userauth_should_tokens_and_session_id.0,
            &userauth_should_tokens_and_session_id.1,
            &userauth_should_tokens_and_session_id.2,
            phone_num,
            &user_name,
            &sms_code,
        ))
        .unwrap();

    Ok(())
}
