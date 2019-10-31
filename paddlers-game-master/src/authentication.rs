use actix_web::dev::Payload;
use actix_web::error::{ErrorUnauthorized, ErrorBadRequest};
use actix_web::{Error, FromRequest, HttpRequest};

use paddlers_shared_lib::user_authentication::PadlUser;
use paddlers_shared_lib::config::Config;

pub struct Authentication {
    pub user: PadlUser,
    _private: (),
}

impl FromRequest for Authentication {
    type Error = Error;
    type Future = Result<Self, Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {

        let config : &Config = req.app_data::<Config>().expect("Need config");
        match req.headers().get(actix_web::http::header::AUTHORIZATION) {
            Some(auth_header) => {
                match auth_header.to_str() {
                    Ok(token) =>
                        match PadlUser::from_token(token, &config) {
                            Ok(user) => Ok(Authentication { user, _private: () } ),
                            Err(e) => Err(ErrorUnauthorized(e))?,
                        },
                    Err(_e) => Err(ErrorBadRequest("Unable to parse token"))?
                }
            },
            None => Err(ErrorUnauthorized("No Authorization Token provided"))?
        }
    }
}