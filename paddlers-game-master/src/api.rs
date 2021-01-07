mod attacks;
mod quests;
mod reports;
mod shop;
mod story;

pub(crate) use attacks::{new_invitation, visitor_satisfied_notification};
pub(crate) use quests::collect_quest;
pub(crate) use reports::collect_report_rewards;
pub(crate) use story::story_transition;

use crate::authentication::Authentication;
use crate::game_master::attack_funnel::PlannedAttack;
use crate::setup::initialize_new_player_account;
use crate::StringErr;
use actix_web::error::BlockingError;
use actix_web::{web, HttpResponse, Responder};
use futures::future::join_all;
use futures::Future;
use paddlers_shared_lib::api::{
    attacks::AttackDescriptor,
    keys::{VillageKey, WorkerKey},
    shop::{BuildingDeletion, BuildingPurchase, ProphetPurchase},
    tasks::TaskList,
    PlayerInitData,
};
use paddlers_shared_lib::sql::GameDB;

pub fn index() -> impl Responder {
    HttpResponse::Ok().body("Game Master OK")
}

pub(crate) fn purchase_prophet(
    pool: web::Data<crate::db::Pool>,
    actors: web::Data<crate::ActorAddresses>,
    body: web::Json<ProphetPurchase>,
    mut auth: Authentication,
) -> impl Future<Item = HttpResponse, Error = ()> {
    let village = body.village;
    std::mem::drop(body);
    web::block(move || {
        let db: crate::db::DB = pool.get_ref().into();
        check_owns_village0(&db, &auth, village)?;
        let result = db.try_buy_prophet(
            village,
            &actors,
            auth.player_object(&db).ok_or("No such player".to_owned())?,
        );
        result
    })
    .then(
        |result: Result<(), BlockingError<std::string::String>>| match result {
            Err(BlockingError::Error(msg)) => Ok(HttpResponse::Forbidden().body(msg).into()),
            Err(BlockingError::Canceled) => Ok(HttpResponse::InternalServerError().into()),
            Ok(()) => Ok(HttpResponse::Ok().into()),
        },
    )
}

pub(crate) fn purchase_building(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<BuildingPurchase>,
    mut auth: Authentication,
    addr: web::Data<crate::ActorAddresses>,
) -> impl Responder {
    let db: crate::db::DB = pool.get_ref().into();

    let building = body.building_type.into();
    if let Err(err) = check_owns_village(&db, &auth, body.village) {
        return err;
    }
    if !db.player_allowed_to_build(
        building,
        body.village,
        &auth.player_object(&db).expect("no player"),
    ) {
        return HttpResponse::BadRequest()
            .body("Player not allowed to build")
            .into();
    }

    db.try_buy_building(building, (body.x, body.y), body.village)
        .and_then(|_| {
            let player = auth.player_key(&db)?;
            db.building_insertion_triggers(building, player, addr)
        })
        .map_or_else(|e| HttpResponse::from(&e), |_| HttpResponse::Ok().into())
}

pub fn delete_building(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<BuildingDeletion>,
    auth: Authentication,
) -> impl Responder {
    let db: crate::db::DB = pool.get_ref().into();

    if let Err(err) = check_owns_village(&db, &auth, body.village) {
        return err;
    }

    if let Some(building) =
        db.find_building_by_coordinates(body.x as i32, body.y as i32, body.village)
    {
        if building.building_type.can_be_deleted() {
            db.delete_building(&building);
            HttpResponse::Ok().into()
        } else {
            HttpResponse::BadRequest().body("This building cannot be deleted".to_owned())
        }
    } else {
        HttpResponse::BadRequest().body(format!("No building at {}|{}", body.x, body.y))
    }
}

pub(super) fn overwrite_tasks(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<TaskList>,
    addr: web::Data<crate::ActorAddresses>,
    auth: Authentication,
) -> impl Responder {
    let db: crate::db::DB = pool.get_ref().into();
    if let Err(err) = check_owns_worker(&db, &auth, body.worker_id) {
        return err;
    }

    match crate::worker_actions::validate_task_list(&db, &body.0) {
        Ok(validated) => {
            for upd in validated.update_tasks {
                db.update_task(&upd);
            }
            crate::worker_actions::replace_worker_tasks(
                &db,
                &addr.town_worker,
                body.worker_id,
                &validated.new_tasks,
                validated.village_id,
            );
        }
        Err(e) => {
            println!("Task creation failed. {} \n Body: {:?}", e, body.0);
            return HttpResponse::BadRequest().body("Couldn't create tasks");
        }
    }
    HttpResponse::Ok().into()
}

/// Must be called by an identified user (via JWT) before using any other Game-Master or GQL services
pub(super) fn new_player(
    pool: web::Data<crate::db::Pool>,
    auth: Authentication,
    body: web::Json<PlayerInitData>,
) -> impl Responder {
    let db: crate::db::DB = pool.get_ref().into();
    if let Err(msg) = initialize_new_player_account(&db, auth.user.uuid, &body) {
        HttpResponse::InternalServerError().body(msg)
    } else {
        HttpResponse::Ok().into()
    }
}
pub(crate) fn create_attack(
    pool: web::Data<crate::db::Pool>,
    actors: web::Data<crate::ActorAddresses>,
    body: web::Json<AttackDescriptor>,
    auth: Authentication,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    let pool0 = pool.clone();
    let pool1 = pool.clone();
    let attack = body.0;
    let (x, y) = attack.to;
    let from_key = attack.from;
    let home_id = from_key.num();
    let attack_funnel = actors.attack_funnel.clone();

    let future_hobos = attack
        .units
        .into_iter()
        .map(move |hobo_key| {
            let db: crate::db::DB = pool.clone().get_ref().into();
            web::block(move || match db.hobo(hobo_key) {
                Some(hobo) => Ok(hobo),
                None => Err("Invalid hobo"),
            })
            .map_err(|e: BlockingError<_>| match e {
                BlockingError::Error(msg) => HttpResponse::Forbidden().body(msg).into(),
                BlockingError::Canceled => internal_server_error("Canceled"),
            })
            .and_then(move |hobo| {
                if hobo.home != home_id {
                    Err(HttpResponse::Forbidden()
                        .body("Hobo not from this village")
                        .into())
                } else {
                    Ok(hobo)
                }
            })
        })
        .collect::<Vec<_>>();
    let future_hobos = join_all(future_hobos);

    let future_villages = web::block(move || {
        let db: crate::db::DB = pool0.get_ref().into();
        check_owns_village0(&db, &auth, from_key)?;
        let destination = db.village_at(x as f32, y as f32);
        if destination.is_none() {
            Err("Invalid target village".to_owned())
        } else {
            Ok(destination.unwrap())
        }
    })
    .map_err(|e: BlockingError<std::string::String>| match e {
        BlockingError::Error(msg) => HttpResponse::Forbidden().body(msg).into(),
        BlockingError::Canceled => internal_server_error("Canceled"),
    })
    .and_then(move |target_village| {
        let db: crate::db::DB = pool1.get_ref().into();
        if let Some(origin_village) = db.village(from_key) {
            Ok((origin_village, target_village))
        } else {
            Err(internal_server_error("Owned village doesn't exist"))
        }
    });
    let joined = future_hobos.join(future_villages);
    joined
        .map(
            |(hobos, (origin_village, destination_village))| PlannedAttack {
                origin_village: Some(origin_village),
                destination_village,
                hobos,
                no_delay: false,
            },
        )
        .and_then(move |pa| attack_funnel.try_send(pa).map_err(internal_server_error))
        .map(|()| HttpResponse::Ok().into())
}

fn check_owns_worker(
    db: &crate::db::DB,
    auth: &Authentication,
    v: WorkerKey,
) -> Result<(), HttpResponse> {
    if db.worker_owned_by(v, auth.user.uuid) {
        Ok(())
    } else {
        Err(HttpResponse::Forbidden().body(format!("Worker not owned by player")))
    }
}
fn check_owns_village0(db: &crate::db::DB, auth: &Authentication, v: VillageKey) -> StringErr {
    if db.village_owned_by(v, auth.user.uuid) {
        Ok(())
    } else {
        Err("Village not owned by player".to_owned())
    }
}
fn check_owns_village(
    db: &crate::db::DB,
    auth: &Authentication,
    v: VillageKey,
) -> Result<(), HttpResponse> {
    check_owns_village0(db, auth, v).map_err(|msg| HttpResponse::Forbidden().body(msg))
}

fn internal_server_error(e: impl ToString) -> actix_web::Error {
    HttpResponse::InternalServerError()
        .body(e.to_string())
        .into()
}
