#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod graphql;
mod hooks;
mod sql;

use rocket::http::Method;
use rocket_contrib::databases::diesel;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use paddlers_shared_lib::config::Config;
use std::io::{self, Read};
use std::fs::File;

#[database("game_db")]
pub struct DbConn(diesel::PgConnection);

use hooks::*;

fn main() {
    let config  = File::open("Paddlers.toml")
        .and_then(|mut file| {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
            Ok(buffer)
        })
        .and_then(|buffer| {
            toml::from_str::<Config>(&buffer)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        })
        .map_err(|err| {
            println!("Can't read config file: {}", err);
        })
        .unwrap_or(
            Config::default()
        );
    let origin = "http://".to_owned() + &config.frontend_base_url;
    let allowed_origins = AllowedOrigins::some_exact(&[&origin]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        max_age: Some(3600*24),
        ..Default::default()
    }
    .to_cors().expect("CORS creation failed");

    rocket::ignite()
        .manage(graphql::new_schema())
        .attach(DbConn::fairing())
        .attach(cors)
        .mount(
            "/graphql",
            routes![index, graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
