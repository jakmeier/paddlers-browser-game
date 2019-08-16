use crate::prelude::*;

const DEMO_GRAPH_QL_PATH: &'static str = "http://demogql.paddlers.ch:11025/graphql";
const DEMO_API_PATH: &'static str = "http://demoapi.paddlers.ch:11026";

pub fn graphql_url() -> PadlResult<String> {
    let domain = hostname()?;
    match domain.as_str() {
        "demo.paddlers.ch" => Ok(DEMO_GRAPH_QL_PATH.to_owned()),
        _ => Ok(format!("http://{}:65432/graphql", &domain))
    }
}
pub fn game_master_url() -> PadlResult<String> {
    let domain = hostname()?;
    match domain.as_str() {
        "demo.paddlers.ch" => Ok(DEMO_API_PATH.to_owned()),
        _ => Ok(format!("http://{}:8088", &domain))
    }
}

fn hostname() -> PadlResult<String>{
    stdweb::web::window()
        .location().ok_or(PadlError::dev_err(PadlErrorCode::NoDataFromBrowser("Location")))?
        .hostname().map_err(PadlError::from)
}