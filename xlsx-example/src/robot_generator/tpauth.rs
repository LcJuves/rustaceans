use lazy_static::lazy_static;

pub struct LoginedAuthInfo<'a> {
    ep_jwt_token_current: &'a str,
    sessionid: &'a str,
    user_name: &'a str,
    email: &'a str,
    staff_code: &'a str,
    token: &'a str,
}

impl<'a> LoginedAuthInfo<'a> {
    pub fn new() -> Self {
        LoginedAuthInfo {
            ep_jwt_token_current: "",
            sessionid: "",
            user_name: "",
            email: "",
            staff_code: "",
            token: "",
        }
    }

    pub fn ep_jwt_token_current(&self) -> String {
        self.ep_jwt_token_current.to_owned()
    }

    pub fn sessionid(&self) -> String {
        self.sessionid.to_owned()
    }

    pub fn user_name(&self) -> String {
        self.user_name.to_owned()
    }

    pub fn email(&self) -> String {
        self.email.to_owned()
    }

    pub fn staff_code(&self) -> String {
        self.staff_code.to_owned()
    }

    pub fn token(&self) -> String {
        self.token.to_owned()
    }
}

lazy_static! {
    pub static ref AUTH_CONF: LoginedAuthInfo<'static> = {
        let mut login_auth_info = LoginedAuthInfo::new();

        login_auth_info
    };
}
