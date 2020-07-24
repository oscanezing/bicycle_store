use std::{convert::Infallible, rc::Rc}; 
use warp::{Filter, hyper::StatusCode};
use crate::{datasource, error, repositories::postgres_repo::BicycleRepoPostgres, services::bicycle::BicycleService,
    controllers::{routes, handler}};

#[tokio::main]
pub async fn rest() {
    let db_pool = datasource::db::create_pool().expect("failed to create pool");

    let bike_repo = BicycleRepoPostgres::new(db_pool.clone());

    let bike_service = BicycleService::new(bike_repo);

    let health_route = warp::path!("health")
        .map(|| StatusCode::OK);

    // let bikes = warp::path!("bicycles");
    // let bicycle_routes = bikes
    //     .and(warp::get())
    //     .and(with_manager(bike_service.clone()))
    //     .and_then(handler::find_all)
    //     .or(bikes
    //         .and(warp::post())
    //         .and(warp::body::json())
    //         .and(with_manager(bike_service.clone()))
    //         .and_then(handler::create));

    let bicycle_prefix = routes::path_prefix(bike_service);

    let list_all_bicycles = bicycle_prefix.clone()
        .and(routes::list())
        .and_then(handler::find_all);

    let get_bike = bicycle_prefix.clone()
        .and(routes::get())
        .and_then(handler::find_by_id);
    
    let create = bicycle_prefix.clone()
        .and(routes::create())
        .and_then(handler::create);

    let bicycles_api = list_all_bicycles
        .or(create)
        .or(get_bike)
        .with(warp::log("bicycle_api"));

    let routes = health_route
    .or(bicycles_api)
    .with(warp::cors().allow_any_origin())
    .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

pub fn with_manager(bicycle_manager: BicycleService<BicycleRepoPostgres>) -> impl Filter<Extract = (BicycleService<BicycleRepoPostgres>,), Error = Infallible> + Clone {
    warp::any().map(move || bicycle_manager.clone())
}