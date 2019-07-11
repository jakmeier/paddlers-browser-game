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

#[database("game_db")]
pub struct DbConn(diesel::PgConnection);

use hooks::*;

fn main() {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8000", "http://127.0.0.1:8000/", "http://localhost:65432"]);

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
