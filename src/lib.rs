#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;

mod bikes;
mod controllers;
mod datasource;
mod delivery;
mod error;
mod repositories;
mod schema;
mod services;

pub fn run() {
    dotenv().ok();

    delivery::rest::rest();
}
