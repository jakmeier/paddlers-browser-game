use crate::DbConn;
use diesel::prelude::*;

impl db_lib::sql::GameDB for DbConn {
    fn provide_connection(&self) -> &PgConnection {
        &**self
    }
}