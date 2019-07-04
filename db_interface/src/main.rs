#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::databases::diesel;

#[database("game_db")]
struct DbConn(diesel::PgConnection);

use db_lib::models::*;
use db_lib::schema::*;
use diesel::prelude::*;

fn main() {
    rocket::ignite()
       .attach(DbConn::fairing())
       .mount("/", routes![index])
       .launch();
}

#[get("/")]
fn index(connection: DbConn) -> String {
    let results = attacks_to_units::table
        .inner_join(units::table)
        .inner_join(attacks::table)
        .limit(5)
        .load::<(AttackToUnit, Unit, Attack)>(&*connection)
        .expect("Error loading data");

    let mut response = String::new();
    response += &format!("Displaying {} rows", results.len());
    for unit in results {
        response += &format!("{:?} {:?}", unit.1, unit.2);
        response += &format!("----------\n");
    }
    response += &format!("\nDONE\n");
    response
}