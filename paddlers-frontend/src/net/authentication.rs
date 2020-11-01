//! Bridge between the rust frontend app and the Keycloak JS adapter

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/init/javascript.js")]
extern "C" {
    pub fn keycloak_token() -> String;
    pub fn keycloak_preferred_name() -> Option<String>;
}
