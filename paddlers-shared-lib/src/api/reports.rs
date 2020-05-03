use crate::api::keys::VisitReportKey;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ReportCollect {
    pub reports: Vec<VisitReportKey>,
}
