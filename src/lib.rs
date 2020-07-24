#[macro_use]
extern crate diesel;

use dotenv::dotenv;

mod datasource;
mod error;
mod schema;
mod bikes;
mod delivery;
mod controllers;
mod repositories;
mod services;

use warp::Rejection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

type RejectionResult<T> = std::result::Result<T, Rejection>;
type DBPool = Pool<ConnectionManager<PgConnection>>;
// type DBConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn run() {
    dotenv().ok();

    delivery::rest::rest();
}
