// curl -d '{"building_type":"BlueFlowers", "x": 11, "y": 5}' -H "Content-Type: application/json" -X POST http://127.0.0.1:8088/shop/building
// // XXX

// pub fn http_place_building(b: BuildingPurchase) -> impl Future<Output = ?> {
//     let request_string = &serde_json::to_string(&request_body).unwrap();
//     let promise = ajax::send("POST", format!("{}/building", SHOP_PATH), request_string);
//     promise.map(|x| {
//         let response: BuildingsResponse = 
//             serde_json::from_str(&x.unwrap()).unwrap();
//         response
//     })
// }