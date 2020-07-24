use crate::{error, error::Error::*, DBPool};
use diesel::r2d2::{ConnectionManager, self};
use diesel::pg::PgConnection;
use std::env;

type CustomResult<T> = std::result::Result<T, error::Error>;

pub fn create_pool() -> CustomResult<DBPool> {
    let db_url = env::var("DATABASE_URL").expect("No database url provided");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder().build(manager).map_err(DBPoolError)
}

// pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
// lazy_static! {
//     pub static ref POOL: Pool = {
//         // DATABASE_URL=postgres://postgres:postgres@localhost/warp
//         dotenv().ok();

//         let database_url = env::var("DATABASE_URL")
//             .expect("DATABASE_URL must be set.");

//         // create db connection pool
//         let manager = ConnectionManager::<PgConnection>::new(database_url);
//         let pool: Pool = r2d2::Pool::builder()
//             .build(manager)
//             .expect("Failed to create pool.");
//         pool
//     };
// }