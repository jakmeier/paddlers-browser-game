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

use actix::prelude::*;
use actix_web::{
    http::header, 
    web, App, HttpServer
};
use actix_cors::Cors;
use paddlers_shared_lib::api::shop::{BuildingPurchase, BuildingDeletion};

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
            .wrap(
                Cors::new()
                    .allowed_origin("http://127.0.0.1:8000")
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600*24),
            )
            .data(dbpool.clone())
            .route("/", web::get().to(api::index))
            .service(
                web::resource("/shop/building")
                .data(web::Json::<BuildingPurchase>)
                .route(web::post().to(api::purchase_building))
            )
            .service(
                web::resource("/shop/building/delete")
                .data(web::Json::<BuildingDeletion>)
                .route(web::post().to(api::delete_building))
            )
    })
    .disable_signals()
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();

    println!("Web-Actix returned");
}