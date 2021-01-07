use crate::authentication::Authentication;
use actix::prelude::*;
use actix_web::error::BlockingError;
use actix_web::{web, HttpResponse};
use paddlers_shared_lib::api::quests::QuestCollect;

pub(crate) fn collect_quest(
    _pool: web::Data<crate::db::Pool>,
    body: web::Json<QuestCollect>,
    auth: Authentication,
    _addr: web::Data<crate::ActorAddresses>,
) -> impl Future<Item = HttpResponse, Error = ()> {
    web::block(move || {
        // Check thatquest is active and all conditions are met, then forward request to DB actor
        // TODO: Checks and execution
        // let db: crate::db::DB = pool.get_ref().into();
        println!("Collecting quest {:?}", body);
        Ok(())
    })
    .then(
        |result: Result<(), BlockingError<std::string::String>>| match result {
            Err(BlockingError::Error(msg)) => Ok(HttpResponse::Forbidden().body(msg).into()),
            Err(BlockingError::Canceled) => Ok(HttpResponse::InternalServerError().into()),
            Ok(()) => Ok(HttpResponse::Ok().into()),
        },
    )
}
