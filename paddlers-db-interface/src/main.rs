#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate juniper;

mod graphql;
mod hooks;
mod sql;

use paddlers_shared_lib::config::Config;
use rocket::http::Method;
use rocket_contrib::databases::diesel;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[database("game_db")]
pub struct DbConn(diesel::PgConnection);

use hooks::*;

fn main() {
    let config = Config::from_env().unwrap_or(Config::default());

    #[cfg(feature = "local")]
    let allowed_origins = AllowedOrigins::all();
    #[cfg(not(feature = "local"))]
    let origin = config.frontend_origin.clone();
    #[cfg(not(feature = "local"))]
    let allowed_origins = AllowedOrigins::some_exact(&[origin]);

    let mut databse_config_table = std::collections::BTreeMap::new();
    let mut inner_table = std::collections::BTreeMap::<std::string::String, String>::new();
    inner_table.insert("url".to_owned(), config.db_url.to_owned().into());
    databse_config_table.insert("game_db".to_owned(), inner_table.into());

    let rocket_config = rocket::config::Config::build(rocket::config::Environment::Production)
        .address(config.graphql_service_name.clone())
        .port(config.graphql_port)
        .extra(
            "databases",
            rocket::config::Value::Table(databse_config_table),
        )
        .finalize()
        .expect("Check Configuration");

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        #[cfg(feature = "local")]
        allowed_methods: vec![Method::Post, Method::Get]
            .into_iter()
            .map(From::from)
            .collect(),
        #[cfg(not(feature = "local"))]
        allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        max_age: Some(3600 * 24),
        ..Default::default()
    }
    .to_cors()
    .expect("CORS creation failed");

    rocket::custom(rocket_config)
        .manage(graphql::new_schema())
        .manage(config)
        .attach(DbConn::fairing())
        .attach(cors)
        .mount(
            "/graphql",
            routes![index, graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
