use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use paddlers_shared_lib::prelude::*;
pub mod diesel_queries;
pub use diesel_queries::*;
mod db_actor;
pub use db_actor::*;
type Manager = ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<Manager>;
pub(crate) struct DB(r2d2::PooledConnection<Manager>);

impl DB {
    pub fn new_pool() -> Pool {
        let url = paddlers_shared_lib::get_db_url();
        let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(url);
        let mut builder = r2d2::Pool::builder();

        #[cfg(feature = "local_test")]
        {
            builder = builder.max_size(5);
        }

        #[cfg(not(feature = "local_test"))]
        {
            builder = builder.max_size(50);
        }

        builder.build(manager).expect("Failed to create pool.")
    }
}

impl From<&Pool> for DB {
    fn from(pool: &Pool) -> Self {
        DB(pool.get().expect("Couldn't get DB connection"))
    }
}

impl GameDB for DB {
    fn dbconn(&self) -> &PgConnection {
        &self.0
    }
}
