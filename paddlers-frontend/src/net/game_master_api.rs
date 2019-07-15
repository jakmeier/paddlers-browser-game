use paddlers_api_lib::shop::*;
use super::ajax;
use super::SHOP_PATH;
use futures::Future;

pub type GameMasterApiResult = Result<String, stdweb::web::error::Error>;

pub fn http_place_building(b: BuildingPurchase) -> impl Future<Output = GameMasterApiResult> {
    let request_string = &serde_json::to_string(&b).unwrap();
    let promise = ajax::send("POST", &format!("{}/building", SHOP_PATH), request_string);
    promise
}