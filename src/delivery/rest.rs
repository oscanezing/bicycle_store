use std::{convert::Infallible}; 
use warp::{Filter, hyper::StatusCode};
use crate::{datasource, error, controllers::routes, repositories::postgres_repo::BicycleRepoPostgres, services::bicycle::BicycleService};

#[tokio::main]
pub async fn rest() {
    let db_pool = datasource::db::create_pool().expect("failed to create pool");

    let bike_repo = BicycleRepoPostgres::new(db_pool.clone());

    let bike_service = BicycleService::new(bike_repo);

    let health_route = warp::path!("health")
        .map(|| StatusCode::OK);

    let bicycle_routes = routes::bicycle_routes(bike_service);

    let routes = health_route.or(bicycle_routes)
    .with(warp::cors().allow_any_origin())
    .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

pub fn with_manager(bicycle_manager: BicycleService<BicycleRepoPostgres>) -> impl Filter<Extract = (BicycleService<BicycleRepoPostgres>,), Error = Infallible> + Clone {
    warp::any().map(move || bicycle_manager.clone())
}