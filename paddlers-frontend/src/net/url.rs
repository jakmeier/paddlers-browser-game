use crate::prelude::*;
use paddlers_shared_lib::prelude::*;

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

/// Parses the location (URL) of the browser to look up the currently selected village
pub fn read_current_village_id() -> PadlResult<VillageKey> {
    query_param("village")
        .and_then(
            |s| 
                match s.parse() {
                    Ok(num) => Ok(VillageKey(num)),
                    Err(e) => PadlErrorCode::UrlParseError(format!("{}",e)).dev(),
                }
            )
}

fn hostname() -> PadlResult<String>{
    stdweb::web::window()
        .location().ok_or(PadlError::dev_err(PadlErrorCode::NoDataFromBrowser("Location")))?
        .hostname().map_err(PadlError::from)
}

fn query_param(key: &str) -> PadlResult<String> {
    let err = PadlError::dev_err(PadlErrorCode::UrlParseError(format!("No such URL query param: {}", key)));
    let s = stdweb::web::window().location().unwrap().search()?;
    if s.len() == 0 {
        return Err(err);
    }
    let mut query_params = url::form_urlencoded::parse(s[1..].as_bytes());
    query_params
        .find(|(k, _v)| k == key)
        .map(|(_k, v)| (*v).to_owned())
        .ok_or(err)
}