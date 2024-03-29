mod attacks;
mod hobo;
mod quests;
mod reports;
mod shop;
mod story;

pub(crate) use attacks::{
    create_attack, new_invitation, visitor_satisfied_notification, welcome_visitor,
};
pub(crate) use hobo::settle_hobo;
pub(crate) use quests::collect_quest;
pub(crate) use reports::collect_report_rewards;
pub(crate) use story::story_transition;

use crate::authentication::Authentication;
use crate::setup::initialize_new_player_account;
use crate::StringErr;
use actix_web::{web, HttpResponse, Responder};
// use futures::Future;
use paddlers_shared_lib::sql::GameDB;
use paddlers_shared_lib::{
    api::{
        keys::{VillageKey, WorkerKey},
        shop::{BuildingDeletion, BuildingPurchase, BuildingUpgrade, ProphetPurchase},
        tasks::TaskList,
        PlayerInitData,
    },
    keys::SqlKey,
};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Game Master OK")
}

pub(crate) async fn purchase_prophet(
    pool: web::Data<crate::db::Pool>,
    actors: web::Data<crate::ActorAddresses>,
    body: web::Json<ProphetPurchase>,
    mut auth: Authentication,
) -> HttpResponse {
    let village = body.village;
    std::mem::drop(body);
    let result = web::block(move || {
        let db: crate::db::DB = pool.get_ref().into();
        check_owns_village0(&db, &auth, village)?;
        let result = db.try_buy_prophet(
            village,
            &actors,
            auth.player_object(&db).ok_or("No such player".to_owned())?,
        );
        result
    })
    .await;

    match result {
        Err(_) => HttpResponse::InternalServerError().into(),
        Ok(Err(msg)) => HttpResponse::Forbidden().body(msg).into(),
        Ok(Ok(())) => HttpResponse::Ok().into(),
    }
}

pub(crate) async fn purchase_building(
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
    let player = auth.player_object(&db).expect("no player");
    if !db.player_allowed_to_build(building, body.village, player) {
        return HttpResponse::BadRequest()
            .body("Player not allowed to build")
            .into();
    }

    db.try_buy_building(building, (body.x, body.y), body.village, player, addr)
        .map_or_else(
            |e| HttpResponse::InternalServerError().body(e),
            |id| HttpResponse::Ok().json(id).into(),
        )
}

pub async fn delete_building(
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
pub async fn upgrade_building(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<BuildingUpgrade>,
    auth: Authentication,
) -> impl Responder {
    let db: crate::db::DB = pool.get_ref().into();

    if let Some(building) = db.building(body.building) {
        if let Err(err) = check_owns_village(&db, &auth, building.village()) {
            return err;
        }
        if building.lv as usize == body.current_level {
            if let Some(price) = building.building_type.upgrade_cost(building.lv as usize) {
                if let Err(err) = db.try_spend(&price, building.village()) {
                    return HttpResponse::BadRequest().body(err);
                }
            } else {
                return HttpResponse::BadRequest().body("Cannot be upgraded.".to_owned());
            }
            db.set_building_level(building.key(), body.current_level as i32 + 1);
            HttpResponse::Ok().into()
        } else {
            HttpResponse::BadRequest()
                .body("Unexpected building level. Duplicate request?".to_owned())
        }
    } else {
        HttpResponse::BadRequest().body("No such building".to_owned())
    }
}

pub(super) async fn overwrite_tasks(
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
pub(super) async fn new_player(
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
