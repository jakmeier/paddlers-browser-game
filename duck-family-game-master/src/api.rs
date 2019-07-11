use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

pub fn index() -> impl Responder {
    HttpResponse::Ok().body("Game Master OK")
}

#[derive(Deserialize)]
pub struct BuildingPurchase {
    building_type: duck_family_db_lib::models::BuildingType,
    x: usize,
    y: usize,
}
pub fn purchase_building(
    pool: web::Data<crate::db::Pool>, 
    body: web::Json<BuildingPurchase>) 
    -> impl Responder 
{
    let db: crate::db::DB = pool.get_ref().into();
    db.try_buy_building(body.building_type, (body.x, body.y))
        .map_or_else(
            |e| HttpResponse::from(&e),
            |_| HttpResponse::Ok().into(), 
        )
}