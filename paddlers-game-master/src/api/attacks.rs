use crate::game_master::event::Event;
use crate::game_master::town_worker::TownWorkerEventMsg;
use actix_web::Responder;
use actix_web::{web, HttpResponse};
use paddlers_shared_lib::prelude::*;

pub(crate) fn visitor_satisfied_notification(
    body: web::Json<HoboKey>,
    addr: web::Data<crate::ActorAddresses>,
) -> impl Responder {
    let event = Event::CheckVisitorHp { hobo_id: body.0 };
    addr.town_worker
        .try_send(TownWorkerEventMsg(event, chrono::Utc::now()))
        .expect("Sending event failed");
    HttpResponse::Ok()
}
