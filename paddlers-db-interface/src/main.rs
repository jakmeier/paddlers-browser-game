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

#[database("game_db")]
pub struct DbConn(diesel::PgConnection);

use hooks::*;

fn main() {
    let config = Config::from_env()
        .unwrap_or(Config::default());
    
    // Debugging CORS issues: 
    // let allowed_origins = AllowedOrigins::all();
    let origin = config.frontend_origin;
    let allowed_origins = AllowedOrigins::some_exact(&[origin]);

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
