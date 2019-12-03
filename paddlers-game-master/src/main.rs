#![feature(result_map_or_else)]
#![feature(exclusive_range_pattern)]
extern crate env_logger;

mod db;
mod resource_system;
mod api;
mod game_master;
mod buildings;
mod worker_actions;
mod town_view;
mod statistics;
mod setup;
mod authentication;

use db::*;
use game_master::{
    GameMaster,
    town_worker::TownWorker,
    economy_worker::EconomyWorker,
    attack_spawn::AttackSpawner,
};
use actix::prelude::*;
use actix_web::{
    http::header, 
    web, App, HttpServer
};
use actix_cors::Cors;
use paddlers_shared_lib::{
    api::{
        shop::{BuildingPurchase, BuildingDeletion, ProphetPurchase},
        tasks::TaskList,
        statistics::FrontendRuntimeStatistics,
    },
    config::{
        Config,
    },
};

type StringErr = Result<(),String>;

struct ActorAddresses {
    _game_master: Addr<GameMaster>,
    town_worker: Addr<TownWorker>,
    _econ_worker: Addr<EconomyWorker>,
    _attack_worker: Addr<AttackSpawner>,
    db_actor: Addr<DbActor>,
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let dbpool: Pool = DB::new_pool();
    let conn: DB = (&dbpool.clone()).into();
    conn.db_scripts_by_env().expect("DB initialization failed.");
    println!("DB successfully migrated");

    let config = Config::from_env()
        .unwrap_or(Config::default());
    let origin = config.frontend_origin.clone();
    let base_url = config.game_master_service_name.clone();

    let sys = actix::System::new("background-worker-example");
    let attack_worker = AttackSpawner::new(dbpool.clone()).start();
    let gm_actor = GameMaster::new(dbpool.clone(), &attack_worker).start();
    let town_worker_actor = TownWorker::new(dbpool.clone()).start();
    let econ_worker = EconomyWorker::new(dbpool.clone()).start();
    let db_actor = DbActor::new(dbpool.clone()).start();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin(&origin)
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600*24),
            )
            .wrap(actix_web::middleware::Logger::default())
            .data(
                ActorAddresses {
                    _game_master: gm_actor.clone(),
                    town_worker: town_worker_actor.clone(),
                    _econ_worker: econ_worker.clone(),
                    _attack_worker: attack_worker.clone(),
                    db_actor: db_actor.clone(),
                })
            .data(config.clone())
            .data(dbpool.clone())
            .route("/", web::get().to(api::index))
            .service(
                web::resource("/player/create")
                .route(web::post().to(api::new_player))
            )
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
            .service(
                web::resource("/shop/unit/prophet")
                .data(web::Json::<ProphetPurchase>)
                .route(web::post().to_async(api::purchase_prophet))
            )
            .service(
                web::resource("/worker/overwriteTasks")
                .data(web::Json::<TaskList>)
                .route(web::post().to(api::overwrite_tasks))
            )
            .service(
                web::resource("/stats")
                .data(web::Json::<FrontendRuntimeStatistics>)
                .route(web::post().to(statistics::new_frontend_info))
            )
    })
    .disable_signals()
    .bind(&base_url).expect("binding")
    .start();

    println!("Listening on {}", base_url);

    sys.run().expect("Actix system failure");
    println!("Web-Actix returned");
}