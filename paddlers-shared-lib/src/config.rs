use serde::Deserialize;
#[cfg(feature = "sql_db")]
use dotenv::dotenv;
#[cfg(feature = "sql_db")]
use std::env;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub frontend_base_url: String,
    pub game_master_base_url: String,
    pub graphql_base_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            frontend_base_url: "localhost:80".to_owned(),
            game_master_base_url: "localhost:8088".to_owned(),
            graphql_base_url: "localhost:65432".to_owned(),
        }
    } 
}

#[cfg(feature = "sql_db")]
impl Config {
    pub fn from_env() -> Option<Self> {
        dotenv().ok();
        Some(
            Config {
                frontend_base_url: env::var("FRONTEND_BASE_URL").ok()?,
                game_master_base_url: env::var("GAME_MASTER_BASE_URL").ok()?,
                graphql_base_url: env::var("GRAPHQL_BASE_URL").ok()?,
            }
        )
    }
}