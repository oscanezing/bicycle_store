use warp::{Filter, hyper::StatusCode};
use crate::{error, repositories::postgres_repo::BicycleRepoPostgres, services::bicycle::BicycleService,
    controllers::{routes, handler}};

#[tokio::main]
pub async fn rest() {
    let bike_repo = BicycleRepoPostgres{};

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

    let update = bicycle_prefix.clone()
        .and(routes::update())
        .and_then(handler::update);

    let delete = bicycle_prefix
        .and(routes::delete())
        .and_then(handler::delete);

    let bicycles_api = list_all_bicycles
        .or(create)
        .or(get_bike)
        .or(update)
        .or(delete)
        .with(warp::log("bicycle_api"));

    let routes = health_route
    .or(bicycles_api)
    .with(warp::cors().allow_any_origin())
    .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}