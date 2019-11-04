use super::DbConn;

use rocket::response::content;
use rocket::State;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use juniper_rocket::{self, GraphQLResponse, GraphQLRequest};
use juniper::FieldError;

use paddlers_shared_lib::prelude::{Config, PadlApiError};
use paddlers_shared_lib::user_authentication::*;
use crate::graphql::Schema;

#[derive(Debug)]
pub struct UserInfo {
    user: Option<PadlUser>,
}

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
    request: GraphQLRequest,
    schema: State<Schema>,
    user_info: UserInfo,
) -> GraphQLResponse {
    generic_graphql_handler(connection, request, schema, user_info)
}

#[post("/", data = "<request>")]
pub fn post_graphql_handler(
    connection: DbConn,
    request: GraphQLRequest,
    schema: State<Schema>,
    user_info: UserInfo,
) -> GraphQLResponse {
    generic_graphql_handler(connection, request, schema, user_info)
}

fn generic_graphql_handler(
    connection: DbConn,
    request: GraphQLRequest,
    schema: State<Schema>,
    user_info: UserInfo,
) -> GraphQLResponse {
    if let Some(player_ctx)  = crate::graphql::Context::new(connection, user_info.user) {
        request.execute(&schema, &player_ctx)
    } else {
        // Lookup error code from shared lib that frontend understands
        let n = PadlApiError::PlayerNotCreated as i32;
        // Create a GQL error
        let err = FieldError::new(
            "Player is not in DB",
            graphql_value!({ "padlcode": n })
        );
        // Pack GQL Error into a GQL response
        // Note: Juniper will send this as BadRequest, although I think
        //       the standard for GQL would be 200 OK
        //       Either way, the HTTP code should not be considered by
        //       the frontend too much, the errors in the response are
        //       what really counts.
        juniper_rocket::GraphQLResponse::error(err)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserInfo {
    type Error = AuthenticationError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(s) => {
                let config = request.guard::<State<Config>>().expect("Config broken");
                match PadlUser::from_token(s, &config) {
                    Ok(user) => Outcome::Success(UserInfo{ user: Some(user) }),
                    Err(e) => {
                        println!("{}", e);
                        Outcome::Failure((Status::Unauthorized, e))
                    },
                }
            },
            None => Outcome::Success(UserInfo{ user: None }),
        }
    }
}