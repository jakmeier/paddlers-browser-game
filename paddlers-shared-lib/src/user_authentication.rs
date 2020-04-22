use crate::config::Config;
use jsonwebtoken::*;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

static RSA_PUB_KEY: OnceCell<Vec<u8>> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

#[derive(Debug)]
/// Authenticated user identity object
pub struct PadlUser {
    /// Minimal authenticated user identity. Must remain unique among all services.
    pub uuid: uuid::Uuid,
    private: (),
}

impl PadlUser {
    pub fn from_token(token: &str, config: &Config) -> Result<Self, AuthenticationError> {
        let mut validation = Validation {
            iss: Some(config.keycloak_issuer.clone()),
            algorithms: vec![Algorithm::RS256],
            ..Default::default()
        };
        validation.set_audience(&"account");

        let key = get_verification_key()?;
        let token_parsed = decode::<Claims>(&token, &&key, &validation)
            .map_err(|e| AuthenticationError::InvalidToken(format!("{:?}", e)))?;

        let uuid = uuid::Uuid::parse_str(&token_parsed.claims.sub)
            .map_err(|_| AuthenticationError::InvalidSubject)?;

        Ok(PadlUser { uuid, private: () })
    }
}

fn get_verification_key<'a>() -> Result<&'a [u8], AuthenticationError> {
    RSA_PUB_KEY
        .get_or_try_init(|| {
            let mut file = std::fs::File::open("/opt/keycloak/pub_rsa.der")
                .map_err(|_| AuthenticationError::NoVerificationKey)?;
            use std::io::Read;
            let mut key = vec![];
            file.read_to_end(&mut key)
                .map_err(|_| AuthenticationError::NoVerificationKey)?;
            Ok(key)
        })
        .map(Vec::as_slice)
}

#[derive(Debug)]
pub enum AuthenticationError {
    NoToken,
    InvalidToken(String),
    InvalidSubject,
    NoVerificationKey,
}

use std::fmt::{self, Display, Formatter};
impl Display for AuthenticationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
