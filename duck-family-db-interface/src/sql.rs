use crate::DbConn;
use diesel::prelude::*;

impl duck_family_db_lib::sql::GameDB for DbConn {
    fn dbconn(&self) -> &PgConnection {
        &**self
    }
}