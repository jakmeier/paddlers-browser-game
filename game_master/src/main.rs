#![feature(result_map_or_else)]

mod db;
mod town_defence;
mod attack_spawn;
mod resource_system;
mod api;
mod game_master;
mod shop;
mod buildings;

use db::*;
use game_master::GameMaster;

use actix_web::{web, App, HttpServer};
use actix::prelude::*;

type StringErr = Result<(),String>;

fn main() {

    let dbpool = DB::new_pool();

    let pool_clone = dbpool.clone();
    std::thread::spawn(move || {
        println!("Starting Game Master...");
        System::run(|| {
            GameMaster::new(pool_clone).start();
        }).unwrap();
        println!("Game master system returned");
    });

    HttpServer::new(move || {
        App::new()
            .data(dbpool.clone())
            .route("/", web::get().to(api::index))
            .service(
                web::resource("/shop/building")
                .data(web::Json::<api::BuildingPurchase>)
                .route(web::post().to(api::purchase_building))
            )
    })
    .disable_signals()
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();

    println!("Web-Actix returned");
}