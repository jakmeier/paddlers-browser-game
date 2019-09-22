mod shop;

use actix_web::{HttpResponse, Responder, web};
use paddlers_shared_lib::api::{
    shop::{BuildingPurchase, BuildingDeletion},
    tasks::{TaskList},
    abilities::AbilityUse,
};
use paddlers_shared_lib::sql::GameDB;

pub fn index() -> impl Responder {
    HttpResponse::Ok().body("Game Master OK")
}

pub fn purchase_building(
    pool: web::Data<crate::db::Pool>, 
    body: web::Json<BuildingPurchase>) 
    -> impl Responder 
{
    let db: crate::db::DB = pool.get_ref().into();
    db.try_buy_building(body.building_type.into(), (body.x, body.y))
        .map_or_else(
            |e| HttpResponse::from(&e),
            |_| HttpResponse::Ok().into(), 
        )
}

pub fn delete_building(
    pool: web::Data<crate::db::Pool>, 
    body: web::Json<BuildingDeletion>
)-> impl Responder 
{
    let db: crate::db::DB = pool.get_ref().into();
    if let Some(building) = db.find_building_by_coordinates(body.x as i32, body.y as i32) {
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
)-> impl Responder 
{
    let db: crate::db::DB = pool.get_ref().into();
    let village_id = 1; // TODO [user authentication]
    match crate::worker_actions::validate_task_list(&db, &body.0, village_id) {
        Ok(validated) => {
            for upd in validated.update_tasks {
                db.update_task(&upd);
            }
            crate::worker_actions::replace_unit_tasks(&db, &addr.town_worker, body.unit_id, &validated.new_tasks);
        }
        Err(e) => { 
            println!("Task creation failed. {} \n Body: {:?}", e, body.0); 
            return HttpResponse::BadRequest().body("Couldn't create tasks");
        }
    }
    HttpResponse::Ok().into()
}

pub (super) fn use_ability(
    pool: web::Data<crate::db::Pool>, 
    body: web::Json<AbilityUse>,
    addr: web::Data<crate::ActorAddresses>,
) -> HttpResponse
{
    let db: crate::db::DB = pool.get_ref().into();
    // TODO
    println!("Use ability {}", body.ability_type);
            // return HttpResponse::BadRequest().body("Ability not ready");
    HttpResponse::Ok().into()
}