use serde::Deserialize;
#[cfg(feature = "sql_db")]
use dotenv::dotenv;
#[cfg(feature = "sql_db")]
use std::env;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub db_url: String,
    pub frontend_origin: String,
    pub game_master_service_name: String,
    pub graphql_service_name: String,
    pub graphql_port: u16,
    pub keycloak_issuer: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            db_url: "postgresql://postgres:password@localhost:5432".to_owned(),
            frontend_origin: "localhost".to_owned(),
            game_master_service_name: "localhost:8088".to_owned(),
            graphql_service_name: "localhost".to_owned(),
            graphql_port: 65432,
            keycloak_issuer: "http://localhost:10002/auth/realms/Paddlers".to_owned(),
        }
    } 
}

#[cfg(feature = "sql_db")]
impl Config {
    pub fn from_env() -> Option<Self> {
        dotenv().ok();
        Some(
            Config {
                db_url: env::var("DATABASE_URL").ok()?,
                frontend_origin: env::var("FRONTEND_ORIGIN").ok()?,
                game_master_service_name: env::var("GAME_MASTER_SERVICE_NAME").ok()?,
                graphql_service_name: env::var("GRAPHQL_SERVICE_NAME").ok()?,
                graphql_port: env::var("GRAPHQL_PORT").ok()?.parse().ok()?,
                keycloak_issuer: env::var("KEYCLOAK_ISSUER").ok()?,
            }
        )
    }
}