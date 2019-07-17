use actix_web::{HttpResponse, Responder, web};
use paddlers_api_lib::shop::{BuildingPurchase, BuildingDeletion};
use paddlers_db_lib::sql::GameDB;

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