use crate::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use super::authentication::keycloak_token;

pub fn fetch<I: serde::Serialize + ?Sized, O: for<'de> serde::Deserialize<'de>>(
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

// #[derive(Debug)]
// pub struct AjaxError {
//     /// The HTTP error code number
//     pub status_code: u16,
//     /// A textual description of the error. If it is a GraphQL error, the "message" value will be stored here.
//     pub description: String,
//     /// Optionally holds an error to be unpacked in further processing
//     pub padl_error: Option<PadlErrorCode>,
// }

// fn padl_error_from_js_array(val: stdweb::Value) -> (Option<PadlErrorCode>, Option<String>) {
//     /* We expect a GQL answer body which look something like this:
//      * {
//      *  data: null,
//      *  errors: [
//      *      {
//      *          extensions: { padlcode: 255 }
//      *          message: "error description",
//      *          locations: [...],
//      *          path: [...],
//      *      },
//      *  ]
//      * }
//      * The interesting bit is the `padlcode` in the error extension, as well as the error message.
//      *
//      * The input to this function is just the error array.
//      * So we want a type-safe
//      *  `errors[0].extension.padlcode`
//      *
//      * See below for a wonderful example of what the JavaScript engine has to
//      * do all the time. In the JS engine, of course, it will only be like that
//      * after several stages of JIT-ing it up.
//      */
//     let gql_error_obj = val
//         .into_array()
//         .and_then(|array| {
//             let vec: Option<Vec<stdweb::Object>> = array.try_into().ok();
//             vec
//         })
//         // Only look at the first error, ignore others
//         .and_then(|v| v.get(0).cloned());

//     let error_message = gql_error_obj
//         .as_ref()
//         .and_then(|inner_obj| inner_obj.to_iter().find(|(key, _val)| key == "message"))
//         .and_then(|(_key, s)| s.try_into().ok());

//     let error_code = gql_error_obj
//         .and_then(|inner_obj| inner_obj.to_iter().find(|(key, _val)| key == "extensions"))
//         .and_then(|(_key, ext)| ext.into_object())
//         .and_then(|inner_obj| inner_obj.to_iter().find(|(key, _val)| key == "padlcode"))
//         .and_then(|(_key, n)| n.try_into().ok())
//         .and_then(PadlApiError::try_from_num)
//         .map(|api_err| match api_err {
//             PadlApiError::PlayerNotCreated => PadlErrorCode::UserNotInDB,
//         });
//     (error_code, error_message)
// }

// impl std::convert::From<stdweb::Value> for AjaxError {
//     fn from(val: stdweb::Value) -> Self {
//         if let Some(obj) = val.as_object() {
//             let mut code = 0;
//             let mut text = None;
//             let mut padl_error = None;
//             for (key, v) in obj.to_iter() {
//                 match key.as_ref() {
//                     "text" => {
//                         text = v.into_string();
//                     }
//                     "code" => {
//                         code = v.try_into().unwrap_or(0);
//                     }
//                     "errors" => {
//                         let (code, msg) = padl_error_from_js_array(v);
//                         padl_error = code;
//                         text = msg;
//                     }
//                     _ => { /* NOP */ }
//                 }
//             }
//             if code == 401 && padl_error.is_none() {
//                 padl_error = Some(PadlErrorCode::AuthorizationRequired);
//             }
//             AjaxError {
//                 status_code: code,
//                 description: text.unwrap_or("No description available".to_owned()),
//                 padl_error,
//             }
//         } else {
//             AjaxError {
//                 status_code: 0,
//                 description: "Returned value is not an object.".to_owned(),
//                 padl_error: None,
//             }
//         }
//     }
// }

// impl TryFrom<stdweb::Value> for AjaxError {
//     type Error = ();
//     fn try_from(v: stdweb::Value) -> Result<Self, Self::Error> {
//         Ok(v.into())
//     }
// }
