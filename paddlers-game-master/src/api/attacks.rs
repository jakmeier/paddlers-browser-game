use crate::authentication::Authentication;
use crate::game_master::attack_funnel::PlannedAttack;
use crate::game_master::event::Event;
use crate::game_master::town_worker::TownWorkerEventMsg;
use actix::prelude::*;
use actix_web::error::BlockingError;
use actix_web::Responder;
use actix_web::{web, HttpResponse};
use paddlers_shared_lib::api::attacks::InvitationDescriptor;
use paddlers_shared_lib::prelude::*;

pub(crate) fn visitor_satisfied_notification(
    body: web::Json<HoboKey>,
    addr: web::Data<crate::ActorAddresses>,
) -> impl Responder {
    let event = Event::CheckVisitorHp { hobo_id: body.0 };
    addr.town_worker
        .try_send(TownWorkerEventMsg(event, chrono::Utc::now()))
        .map_err(|e| eprintln!("Send failed: {:?}", e))
}

pub(crate) fn new_invitation(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<InvitationDescriptor>,
    auth: Authentication,
    addr: web::Data<crate::ActorAddresses>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    web::block(move || {
        // Check that request is valid and forward request to actor
        let db: crate::db::DB = pool.get_ref().into();
        let origin_vid = db.building(body.nest).ok_or("Nest not found")?.village();
        let origin_village = db.village(origin_vid);
        let destination_village = db.village(body.to).ok_or("Village not found")?;
        let hobos = db.idle_hobos_in_nest(body.nest);
        if !db.village_owned_by(destination_village.key(), auth.user.uuid) {
            return Err("Village not owned by player".to_owned());
        }
        let atk = PlannedAttack {
            origin_village,
            destination_village,
            hobos,
            no_delay: false,
        };
        addr.attack_funnel
            .try_send(atk)
            .map_err(|e| format!("Spawning attack failed: {:?}", e))
    })
    .then(
        |result: Result<(), BlockingError<std::string::String>>| match result {
            Err(BlockingError::Error(msg)) => Ok(HttpResponse::Forbidden().body(msg).into()),
            Err(BlockingError::Canceled) => Ok(HttpResponse::InternalServerError().into()),
            Ok(()) => Ok(HttpResponse::Ok().into()),
        },
    )
}
