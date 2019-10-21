//! Bridge between the rust frontend app and the Keycloak JS adapter
 
#[allow(dead_code)]
/// Currently only used for debugging
pub fn read_jwt() -> Option<String> {
    let jwt = js!{
        return window.keycloak.token;
    };
    println!("Encoded JWT: {:?}", jwt);
    jwt.into_string()
}