use crate::authentication::Authentication;
use crate::db::CollectReportRewardsMessage;
use actix::prelude::*;
use actix_web::error::BlockingError;
use actix_web::{web, HttpResponse};
use paddlers_shared_lib::api::reports::ReportCollect;
use paddlers_shared_lib::prelude::*;

pub(crate) fn collect_report_rewards(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<ReportCollect>,
    auth: Authentication,
    addr: web::Data<crate::ActorAddresses>,
) -> impl Future<Item = HttpResponse, Error = ()> {
    web::block(move || {
        // Check that request is valid and forward request to actor
        let db: crate::db::DB = pool.get_ref().into();
        for rid in body.0.reports {
            let report = db.report(rid).ok_or("No such report".to_owned())?;
            super::check_owns_village0(&db, &auth, report.village())?;
            spawn_report_collection(&addr, report);
        }
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

fn spawn_report_collection(addr: &web::Data<crate::ActorAddresses>, report: VisitReport) {
    let msg = CollectReportRewardsMessage(report);
    let future = addr
        .db_actor
        .send(msg)
        .map_err(|e| eprintln!("Reward collection spawn failed: {:?}", e));
    Arbiter::spawn(future);
}
