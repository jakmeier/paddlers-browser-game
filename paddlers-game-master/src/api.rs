mod shop;

use actix_web::{HttpResponse, Responder, web};
use paddlers_shared_lib::api::{
    shop::{BuildingPurchase, BuildingDeletion},
    tasks::{TaskList},
    keys::{VillageKey, WorkerKey},
};
use paddlers_shared_lib::sql::GameDB;
use crate::authentication::Authentication;
use crate::setup::initialize_new_player_account;

pub fn index() -> impl Responder {
    HttpResponse::Ok().body("Game Master OK")
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
)-> impl Responder {
    let db: crate::db::DB = pool.get_ref().into();
    if let Err(msg) = initialize_new_player_account(&db, auth.user.uuid) {
        HttpResponse::InternalServerError().body(msg)
    } else {
        HttpResponse::Ok().into()
    }
}

fn check_owns_worker(db: &crate::db::DB, auth: &Authentication, v: WorkerKey) -> Result<(), HttpResponse> {
    if db.worker_owned_by(v, auth.user.uuid) {
        Ok(())
    } else {
        Err(HttpResponse::Forbidden().body(format!("Worker not owned by player")))
    }
}
fn check_owns_village(db: &crate::db::DB, auth: &Authentication, v: VillageKey) -> Result<(), HttpResponse> {
    if db.village_owned_by(v, auth.user.uuid) {
        Ok(())
    } else {
        Err(HttpResponse::Forbidden().body(format!("Village not owned by player")))
    }
}