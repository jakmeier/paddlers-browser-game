use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub frontend_base_url: String,
    pub game_master_base_url: String,
    pub graphql_base_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            frontend_base_url: "127.0.0.1:8000".to_owned(),
            game_master_base_url: "127.0.0.1:8088".to_owned(),
            graphql_base_url: "127.0.0.1:65432".to_owned(),
        }
    }
}