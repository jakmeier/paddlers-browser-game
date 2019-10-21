use super::DbConn;

use juniper_rocket;
use rocket::response::content;
use rocket::State;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

use paddlers_shared_lib::prelude::Config;
use crate::graphql::Schema;

#[get("/")]
pub (crate) fn index() -> String {
    format!("GraphQL interface OK")
}


#[get("/graphiql")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}


#[get("/?<request>")]
pub fn get_graphql_handler(
    connection: DbConn,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    user_info: UserInfo,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &connection.into())
}

#[post("/", data = "<request>")]
pub fn post_graphql_handler(
    connection: DbConn,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    user_info: UserInfo,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &connection.into())
}

use paddlers_shared_lib::user_authentication::*;

#[derive(Debug)]
pub struct UserInfo {
    user: PadlUser,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserInfo {
    type Error = AuthenticationError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(s) => {
                let config = request.guard::<State<Config>>().expect("Config broken");
                match PadlUser::from_token(s, &config) {
                    Ok(user) => Outcome::Success(UserInfo{ user }),
                    Err(e) => Outcome::Failure((Status::Unauthorized, e)),
                }
            },
            None => Outcome::Failure((Status::BadRequest, AuthenticationError::NoToken)),
        }
    }
}