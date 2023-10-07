use crate::authentication::Authentication;
use crate::db::CollectReportRewardsMessage;
use crate::StringError;
use actix_web::{web, HttpResponse};
use futures_util::TryFutureExt;
use paddlers_shared_lib::api::reports::ReportCollect;
use paddlers_shared_lib::prelude::*;

pub(crate) async fn collect_report_rewards(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<ReportCollect>,
    auth: Authentication,
    addr: web::Data<crate::ActorAddresses>,
) -> Result<HttpResponse, StringError> {
    // Check that request is valid and forward request to actor
    let db: crate::db::DB = pool.get_ref().into();
    for rid in body.0.reports {
        let report = db.report(rid).ok_or("No such report")?;
        super::check_owns_village0(&db, &auth, report.village())?;
        spawn_report_collection(&addr, report).await;
    }
    Ok(HttpResponse::Ok().into())
}

async fn spawn_report_collection(addr: &web::Data<crate::ActorAddresses>, report: VisitReport) {
    let msg = CollectReportRewardsMessage(report);
    let _err = addr
        .db_actor
        .send(msg)
        .map_err(|e| eprintln!("Reward collection spawn failed: {:?}", e))
        .await;
}
