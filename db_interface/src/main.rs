#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod graphql;
mod hooks;
mod sql;

use rocket_contrib::databases::diesel;

#[database("game_db")]
pub struct DbConn(diesel::PgConnection);

use hooks::*;

fn main() {
    rocket::ignite()
        .manage(graphql::new_schema())
        .attach(DbConn::fairing())
        .mount(
            "/graphql",
            routes![index, graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
