use std::future::{ready, Ready};

use actix_web::dev::Payload;
use actix_web::error::{ErrorBadRequest, ErrorUnauthorized};
use actix_web::{Error, FromRequest, HttpRequest};

use crate::db::DB;
use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::user_authentication::PadlUser;

pub struct Authentication {
    pub user: PadlUser,
    cached_player: Option<Player>,
    _private: (),
}

impl FromRequest for Authentication {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        fn authenticate(req: &HttpRequest) -> Result<Authentication, Error> {
            let config: &Config = req.app_data::<Config>().expect("Need config");
            match req.headers().get(actix_web::http::header::AUTHORIZATION) {
                Some(auth_header) => match auth_header.to_str() {
                    Ok(token) => match PadlUser::from_token(token, &config) {
                        Ok(user) => Ok(Authentication {
                            user,
                            _private: (),
                            cached_player: None,
                        }),
                        Err(e) => Err(ErrorUnauthorized(e))?,
                    },
                    Err(_e) => Err(ErrorBadRequest("Unable to parse token"))?,
                },
                None => Err(ErrorUnauthorized("No Authorization Token provided"))?,
            }
        }
        ready(authenticate(req))
    }
}

impl Authentication {
    pub(crate) fn player_object(&mut self, db: &DB) -> Option<&Player> {
        if self.cached_player.is_none() {
            self.cached_player = db.player_by_uuid(self.user.uuid);
        }
        self.cached_player.as_ref()
    }
    pub(crate) fn player_key(&mut self, db: &DB) -> Result<PlayerKey, String> {
        self.player_object(&db)
            .ok_or("No such player".to_owned())
            .map(|p| p.key())
    }
}
