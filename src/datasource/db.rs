use crate::{error, error::Error::*, DBPool};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::pg::PgConnection;
use std::env;

type CustomResult<T> = std::result::Result<T, error::Error>;

pub fn create_pool() -> CustomResult<DBPool> {
    let db_url = env::var("DATABASE_URL").expect("No database url provided");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager).map_err(DBPoolError)
}