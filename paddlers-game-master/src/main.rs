#![feature(exclusive_range_pattern)]
#![feature(associated_type_bounds)]
extern crate env_logger;

mod api;
mod authentication;
mod buildings;
mod db;
mod game_master;
mod resource_system;
mod setup;
mod statistics;
mod town_view;
mod worker_actions;

use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use db::*;
use game_master::{
    attack_funnel::AttackFunnel, attack_spawn::AttackSpawner, economy_worker::EconomyWorker,
    story_worker::StoryWorker, town_worker::TownWorker, GameMaster,
};
use paddlers_shared_lib::api::{
    attacks::InvitationDescriptor, quests::QuestCollect, reports::ReportCollect,
    story::StoryStateTransition,
};
use paddlers_shared_lib::prelude::HoboKey;
use paddlers_shared_lib::{
    api::{
        attacks::AttackDescriptor,
        shop::{BuildingDeletion, BuildingPurchase, ProphetPurchase},
        statistics::FrontendRuntimeStatistics,
        tasks::TaskList,
    },
    config::Config,
};

type StringErr = Result<(), String>;

struct ActorAddresses {
    _game_master: Addr<GameMaster>,
    town_worker: Addr<TownWorker>,
    _econ_worker: Addr<EconomyWorker>,
    _attack_worker: Addr<AttackSpawner>,
    db_actor: Addr<DbActor>,
    attack_funnel: Addr<AttackFunnel>,
    story_worker: Addr<StoryWorker>,
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let dbpool: Pool = DB::new_pool();
    let conn: DB = (&dbpool.clone()).into();
    conn.db_scripts_by_env().expect("DB initialization failed.");
    println!("DB successfully migrated");

    let config = Config::from_env().unwrap_or(Config::default());
    let origin = config.frontend_origin.clone();
    let base_url = config.game_master_service_name.clone();

    // This starts an actix runtime in the current thread that can be used from now on.
    let sys = actix::System::new("Actix Main System");

    // Start some DB actors in separate threads - they will be blocking
    let db = dbpool.clone();
    let db_actor = SyncArbiter::start(2, move || DbActor::new(db.clone()));

    // Spawn all "normal" actors onto the actix system
    let town_worker_actor = TownWorker::new(dbpool.clone()).start();
    let attack_funnel =
        AttackFunnel::new(dbpool.clone(), db_actor.clone(), town_worker_actor.clone()).start();
    let attack_worker =
        AttackSpawner::new(dbpool.clone(), db_actor.clone(), attack_funnel.clone()).start();
    let gm_actor = GameMaster::new(dbpool.clone(), &attack_worker).start();
    let econ_worker = EconomyWorker::new(dbpool.clone()).start();
    let story_worker = StoryWorker::new(db_actor.clone(), attack_worker.clone()).start();

    // Also spawn the HTTP server on the same runtime
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin(&origin)
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600 * 24),
            )
            .wrap(actix_web::middleware::Logger::default())
            .data(ActorAddresses {
                _game_master: gm_actor.clone(),
                town_worker: town_worker_actor.clone(),
                _econ_worker: econ_worker.clone(),
                _attack_worker: attack_worker.clone(),
                db_actor: db_actor.clone(),
                attack_funnel: attack_funnel.clone(),
                story_worker: story_worker.clone(),
            })
            .data(config.clone())
            .data(dbpool.clone())
            .route("/", web::get().to(api::index))
            .service(web::resource("/player/create").route(web::post().to(api::new_player)))
            .service(
                web::resource("/shop/building")
                    .data(web::Json::<BuildingPurchase>)
                    .route(web::post().to(api::purchase_building)),
            )
            .service(
                web::resource("/shop/building/delete")
                    .data(web::Json::<BuildingDeletion>)
                    .route(web::post().to(api::delete_building)),
            )
            .service(
                web::resource("/shop/unit/prophet")
                    .data(web::Json::<ProphetPurchase>)
                    .route(web::post().to_async(api::purchase_prophet)),
            )
            .service(
                web::resource("/worker/overwriteTasks")
                    .data(web::Json::<TaskList>)
                    .route(web::post().to(api::overwrite_tasks)),
            )
            .service(
                web::resource("/attacks/create")
                    .data(web::Json::<AttackDescriptor>)
                    .route(web::post().to_async(api::create_attack)),
            )
            .service(
                web::resource("/attacks/invite")
                    .data(web::Json::<InvitationDescriptor>)
                    .route(web::post().to_async(api::new_invitation)),
            )
            .service(
                web::resource("/attacks/notifications/visitor_satisfied")
                    .data(web::Json::<HoboKey>)
                    .route(web::post().to(api::visitor_satisfied_notification)),
            )
            .service(
                web::resource("/report/collect")
                    .data(web::Json::<ReportCollect>)
                    .route(web::post().to_async(api::collect_report_rewards)),
            )
            .service(
                web::resource("/story/transition")
                    .data(web::Json::<StoryStateTransition>)
                    .route(web::post().to(api::story_transition)),
            )
            .service(
                web::resource("/stats")
                    .data(web::Json::<FrontendRuntimeStatistics>)
                    .route(web::post().to(statistics::new_frontend_info)),
            )
            .service(
                web::resource("/quest/collect")
                    .data(web::Json::<QuestCollect>)
                    .route(web::post().to_async(api::collect_quest)),
            )
    })
    .disable_signals()
    .bind(&base_url)
    .expect("binding")
    .start();

    println!("Listening on {}", base_url);

    sys.run().expect("Actix system failure");
    println!("Web-Actix returned");
}
