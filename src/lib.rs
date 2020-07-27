#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;

mod datasource;
mod error;
mod schema;
mod bikes;
mod delivery;
mod controllers;
mod repositories;
mod services;

pub fn run() {
    dotenv().ok();

    delivery::rest::rest();
}
