use crate::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use super::{authentication::keycloak_token, graphql::gql_extract_data};

/// Sends a HTTP request and converts a successful response to a JSON message.
/// Non successful response codes are converted to an error.
pub fn fetch_json<I: serde::Serialize + ?Sized, O: for<'de> serde::Deserialize<'de>>(
    method: &str,
    uri: &str,
    request_body: &I,
) -> impl std::future::Future<Output = PadlResult<O>> {
    let mut opts = RequestInit::new();
    opts.method(method);
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from_str(
        &serde_json::to_string(&request_body).unwrap(),
    )));

    let request = Request::new_with_str_and_init(&uri, &opts);

    async {
        let request = request?;
        let headers = request.headers();
        headers.set("Content-Type", "application/json;charset=UTF-8")?;
        headers.set("Authorization", &keycloak_token())?;
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        if resp.ok() {
            let json = JsFuture::from(resp.json()?).await?;
            let data: O = json.into_serde()?;
            Ok(data)
        } else {
            let status_code = resp.status();
            PadlErrorCode::RestAPI(JsFuture::from(resp.text()?).await?.as_string().unwrap()).dev()
        }
    }
}

pub fn fetch_empty_response<I: serde::Serialize + ?Sized>(
    method: &str,
    uri: &str,
    request_body: &I,
) -> impl std::future::Future<Output = PadlResult<()>> {
    let mut opts = RequestInit::new();
    opts.method(method);
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from_str(
        &serde_json::to_string(&request_body).unwrap(),
    )));

    let request = Request::new_with_str_and_init(&uri, &opts);

    async {
        let request = request?;
        let headers = request.headers();
        headers.set("Content-Type", "application/json;charset=UTF-8")?;
        headers.set("Authorization", &keycloak_token())?;
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        if resp.ok() {
            Ok(())
        } else {
            let status_code = resp.status();
            PadlErrorCode::RestAPI(JsFuture::from(resp.text()?).await?.as_string().unwrap()).dev()
        }
    }
}

pub fn gql_query<I: serde::Serialize + ?Sized, O: for<'de> serde::Deserialize<'de>>(
    uri: &str,
    request_body: &I,
) -> impl std::future::Future<Output = PadlResult<O>> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from_str(
        &serde_json::to_string(&request_body).unwrap(),
    )));

    let request = Request::new_with_str_and_init(&uri, &opts);

    async {
        let request = request?;
        let headers = request.headers();
        headers.set("Content-Type", "application/json;charset=UTF-8")?;
        headers.set("Authorization", &keycloak_token())?;
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();

        let json = JsFuture::from(resp.json()?).await?;
        let data: graphql_client::Response<O> = json.into_serde()?;
        gql_extract_data(data)
    }
}
