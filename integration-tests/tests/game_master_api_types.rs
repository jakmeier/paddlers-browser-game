//! Tests to ensure API types (compatible with Diesel and Juniper) are aligned with API types (compatible with WASM)

#[test]
fn test_building_type() {
    use paddlers_shared_lib::strum::IntoEnumIterator;
    let mut db_data_iter = paddlers_shared_lib::models::BuildingType::iter();
    for browser_data in paddlers_api_lib::types::BuildingType::iter() {

        let msg = serde_json::to_string(&browser_data);
        assert!(msg.is_ok(), "Serialization failed");

        let parsed = serde_json::from_str::<paddlers_shared_lib::models::BuildingType>(&msg.unwrap());
        assert!(parsed.is_ok(), "API and DB have diverged");

        let db_data = db_data_iter.next();
        assert!(db_data.is_some(), "Unequal enum member count");
        assert_eq!(parsed.unwrap(), db_data.unwrap());
    }
    assert!(db_data_iter.next().is_none(), "Unequal enum member count");
}

#[test]
fn test_resource_type() {

    use paddlers_shared_lib::strum::IntoEnumIterator;
    let mut db_data_iter = paddlers_shared_lib::models::ResourceType::iter();
    for browser_data in paddlers_api_lib::types::ResourceType::iter() {

        let msg = serde_json::to_string(&browser_data);
        assert!(msg.is_ok(), "Serialization failed");

        let parsed = serde_json::from_str::<paddlers_shared_lib::models::ResourceType>(&msg.unwrap());
        assert!(parsed.is_ok(), "API and DB have diverged");

        let db_data = db_data_iter.next();
        assert!(db_data.is_some(), "Unequal enum member count");
        assert_eq!(parsed.unwrap(), db_data.unwrap());
    }
    assert!(db_data_iter.next().is_none(), "Unequal enum member count");
}
