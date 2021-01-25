use crate::{authentication::Authentication, db::DB};
use actix_web::*;
use paddlers_shared_lib::{api::hobo::SettleHobo, prelude::*};

use super::check_owns_village;

pub(crate) fn settle_hobo(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<SettleHobo>,
    auth: Authentication,
) -> impl Responder {
    let db: crate::db::DB = pool.get_ref().into();

    let building = db
        .building(body.nest)
        .ok_or_else(|| HttpResponse::BadRequest().body("No such building"));
    if let Err(err) = building {
        return err;
    }
    let building = building.unwrap();

    match building.building_type {
        BuildingType::SingleNest | BuildingType::TripleNest => {
            // OK
        }
        _ => {
            return HttpResponse::BadRequest().body("Not a nest, cannot settle a Paddler here!");
        }
    }
    if let Err(err) = check_owns_village(&db, &auth, building.village()) {
        return err;
    }
    db.settle_hobo(building.village(), building.key());
    HttpResponse::Ok().into()
}

impl DB {
    fn settle_hobo(&self, village: VillageKey, nest_id: BuildingKey) {
        self.insert_hobo(&NewHobo {
            hp: 5,
            home: village.num(),
            color: Some(UnitColor::Yellow),
            speed: 0.1,
            hurried: false,
            nest: Some(nest_id.num()),
        });
    }
}
