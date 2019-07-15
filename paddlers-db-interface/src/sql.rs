use crate::DbConn;
use diesel::prelude::*;

impl paddlers_db_lib::sql::GameDB for DbConn {
    fn dbconn(&self) -> &PgConnection {
        &**self
    }
}