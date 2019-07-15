use super::DbConn;

use juniper_rocket;
use rocket::response::content;
use rocket::State;

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
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &connection.into())
}

#[post("/", data = "<request>")]
pub fn post_graphql_handler(
    connection: DbConn,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &connection.into())
}