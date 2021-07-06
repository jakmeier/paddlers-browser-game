use crate::prelude::*;
use paddle::JsError;
use paddlers_shared_lib::prelude::*;

pub fn graphql_url() -> PadlResult<String> {
    let domain = hostname()?;
    match domain.as_str() {
        "localhost" => Ok(format!("http://{}/graphql/", &domain)),
        "10.42.0.1" => Ok(format!("http://{}/graphql/", &domain)),
        _ => Ok(format!("https://{}/graphql/", &domain)),
    }
}
pub fn game_master_url() -> PadlResult<String> {
    let domain = hostname()?;
    match domain.as_str() {
        "localhost" => Ok(format!("http://{}/api/", &domain)),
        "10.42.0.1" => Ok(format!("http://{}/api/", &domain)),
        _ => Ok(format!("https://{}/api/", &domain)),
    }
}

/// Parses the location (URL) of the browser to look up the currently selected village
pub fn read_current_village_id() -> PadlResult<VillageKey> {
    query_param("village").and_then(|s| match s.parse() {
        Ok(num) => Ok(VillageKey(num)),
        Err(e) => PadlErrorCode::UrlParseError(format!("{}", e)).dev(),
    })
}

fn hostname() -> PadlResult<String> {
    web_sys::window()
        .unwrap()
        .location()
        .hostname()
        .map_err(JsError::from_js_value)
        .map_err(PadlError::from)
}

pub(crate) fn query_param(key: &str) -> PadlResult<String> {
    let err = PadlError::dev_err(PadlErrorCode::UrlParseError(format!(
        "No such URL query param: {}",
        key
    )));
    let s = web_sys::window().unwrap().location().search()?;
    if s.len() == 0 {
        return Err(err);
    }
    let mut query_params = url::form_urlencoded::parse(s[1..].as_bytes());
    query_params
        .find(|(k, _v)| k == key)
        .map(|(_k, v)| (*v).to_owned())
        .ok_or(err)
}
