// use warp::{Rejection, Filter, Reply, filters::BoxedFilter};
// use crate::{delivery, handler, DBPool};

// pub fn bicycle_routes(db_pool: DBPool) -> BoxedFilter<(impl Reply,)> {
//     warp::path!("bicycles")
//     .and(warp::get())
//     .and(delivery::rest::with_db(db_pool.clone()))
//     .and_then(handler::find_all).boxed()
// }

use warp::{Filter, Reply, filters::BoxedFilter};
use crate::{ 
    delivery,
    services::bicycle::BicycleService,
    repositories::postgres_repo::BicycleRepoPostgres,
};
use super::handler;

pub fn bicycle_routes(manager: BicycleService<BicycleRepoPostgres>) -> BoxedFilter<(impl Reply,)> {
    let bicycles = warp::path!("bicycles");
    bicycles
    .and(warp::get())
    .and(delivery::rest::with_manager(manager))
    // .and_then(move |_| async move {BicycleService::<BicycleRepoPostgres>::find_all})
    .and_then(handler::find_all).boxed()
    // .map(|a| "")
    // .boxed()

    // let save = bicycles
    //     .and(warp::post())
    //     // .and(warp::body::json())
    //     .and(delivery::rest::with_manager(manager))
    //     .and_then(handler::find_all);

    // find_all.and(save).boxed()    
    // .
    
}