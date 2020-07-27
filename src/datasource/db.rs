use diesel::r2d2::{ConnectionManager, self, Pool};
use diesel::pg::PgConnection;
use std::env;

use dotenv::dotenv;
use crate::error::Error;
use r2d2::PooledConnection;

type DBPool = Pool<ConnectionManager<PgConnection>>;
type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref PG_POOL: DBPool = {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set.");

        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        pool
    };
}

pub fn connection() -> Result<DBConnection, Error> {
    PG_POOL.get().map_err(|e| Error::DBPoolError(e))
}