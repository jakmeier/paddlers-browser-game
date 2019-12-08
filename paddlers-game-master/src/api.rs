mod shop;

use futures::Future;
use actix_web::{HttpResponse, Responder, web};
use actix_web::error::BlockingError;
use paddlers_shared_lib::api::{
    shop::{BuildingPurchase, BuildingDeletion, ProphetPurchase},
    tasks::{TaskList},
    keys::{VillageKey, WorkerKey},
    PlayerInitData,
    attacks::AttackDescriptor,
};
use paddlers_shared_lib::sql::GameDB;
use crate::authentication::Authentication;
use crate::setup::initialize_new_player_account;
use crate::StringErr;

pub fn index() -> impl Responder {
    HttpResponse::Ok().body("Game Master OK")
}

pub (crate) fn purchase_prophet(
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
            auth.player_object(&db).ok_or("No such player".to_owned())?
        );
        result
    })
    .then( |result: Result<(), BlockingError<std::string::String>> |
        match result {
            Err(BlockingError::Error(msg)) => Ok(HttpResponse::Forbidden().body(msg).into()),
            Err(BlockingError::Canceled) => Ok(HttpResponse::InternalServerError().into()),
            Ok(()) => Ok(HttpResponse::Ok().into()),
        }
    )
}

pub fn purchase_building(
    pool: web::Data<crate::db::Pool>, 
    body: web::Json<BuildingPurchase>,
    auth: Authentication,
    ) 
    -> impl Responder 
{
    let db: crate::db::DB = pool.get_ref().into();

    if let Err(err) = check_owns_village(&db, &auth, body.village) {
        return err;
    }

    db.try_buy_building(body.building_type.into(), (body.x, body.y), body.village)
        .map_or_else(
            |e| HttpResponse::from(&e),
            |_| HttpResponse::Ok().into(), 
        )
}

pub fn delete_building(
    pool: web::Data<crate::db::Pool>, 
    body: web::Json<BuildingDeletion>,
    auth: Authentication,
)-> impl Responder 
{
    let db: crate::db::DB = pool.get_ref().into();

    if let Err(err) = check_owns_village(&db, &auth, body.village) {
        return err;
    }

    if let Some(building) = db.find_building_by_coordinates(body.x as i32, body.y as i32, body.village) {
        db.delete_building(&building);
        HttpResponse::Ok().into()
    } else {
        HttpResponse::BadRequest().body(format!("No building at {}|{}", body.x, body.y))
    }
}

pub (super) fn overwrite_tasks(
    pool: web::Data<crate::db::Pool>, 
    body: web::Json<TaskList>,
    addr: web::Data<crate::ActorAddresses>,
    auth: Authentication,
)-> impl Responder 
{
    let db: crate::db::DB = pool.get_ref().into();
    
    if let Err(err) = check_owns_worker(&db, &auth, body.worker_id) {
        return err;
    }

    match crate::worker_actions::validate_task_list(&db, &body.0) {
        Ok(validated) => {
            for upd in validated.update_tasks {
                db.update_task(&upd);
            }
            crate::worker_actions::replace_worker_tasks(&db, &addr.town_worker, body.worker_id, &validated.new_tasks, validated.village_id);
        }
        Err(e) => { 
            println!("Task creation failed. {} \n Body: {:?}", e, body.0); 
            return HttpResponse::BadRequest().body("Couldn't create tasks");
        }
    }
    HttpResponse::Ok().into()
}

/// Must be called by an identified user (via JWT) before using any other Game-Master or GQL services
pub (super) fn new_player(
    pool: web::Data<crate::db::Pool>, 
    auth: Authentication,
    body: web::Json<PlayerInitData>,
)-> impl Responder {
    let db: crate::db::DB = pool.get_ref().into();
    if let Err(msg) = initialize_new_player_account(&db, auth.user.uuid, &body) {
        HttpResponse::InternalServerError().body(msg)
    } else {
        HttpResponse::Ok().into()
    }
}

pub (crate) fn create_attack(
    pool: web::Data<crate::db::Pool>,
    actors: web::Data<crate::ActorAddresses>,
    body: web::Json<AttackDescriptor>,
    mut auth: Authentication,
) -> impl Future<Item = HttpResponse, Error = ()> {
    
    web::block(move || {
        // let db: crate::db::DB = pool.get_ref().into();
        // check_owns_village0(&db, &auth, village)?;
        println!("Sending attack now");
        // TODO: Perform checks, enqueue attack in attack spawner
        Ok(())
    })
    .then( |result: Result<(), BlockingError<std::string::String>> |
        Ok(HttpResponse::Ok().into()),
    )
}


fn check_owns_worker(db: &crate::db::DB, auth: &Authentication, v: WorkerKey) -> Result<(), HttpResponse> {
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
fn check_owns_village(db: &crate::db::DB, auth: &Authentication, v: VillageKey) -> Result<(), HttpResponse> {
    check_owns_village0(db, auth, v).map_err(
        |msg|
        HttpResponse::Forbidden().body(msg)
    )
}