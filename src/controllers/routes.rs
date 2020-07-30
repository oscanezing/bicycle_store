use super::model::BicycleRequest;
use crate::{repositories::postgres_repo::BicycleRepoPostgres, services::bicycle::BicycleService};
use std::convert::Infallible;
use warp::{filters::BoxedFilter, path, Filter};

pub fn with_manager(
    bicycle_manager: BicycleService<BicycleRepoPostgres>,
) -> impl Filter<Extract = (BicycleService<BicycleRepoPostgres>,), Error = Infallible> + Clone {
    warp::any().map(move || bicycle_manager.clone())
}

pub fn path_prefix(
    manager: BicycleService<BicycleRepoPostgres>,
) -> BoxedFilter<(BicycleService<BicycleRepoPostgres>,)> {
    path!("api" / "v1" / "bicycles" / ..)
        .and(with_manager(manager.clone()))
        .boxed()
}

pub fn list() -> BoxedFilter<()> {
    warp::get().and(warp::path::end()).boxed()
}

pub fn get() -> BoxedFilter<(i32,)> {
    warp::get().and(warp::path::param::<i32>()).boxed()
}

pub fn create() -> BoxedFilter<(BicycleRequest,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::post().and(warp::path::end()).and(json_body).boxed()
}

pub fn update() -> BoxedFilter<(i32, BicycleRequest)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::put()
        .and(warp::path::param::<i32>())
        .and(json_body)
        .boxed()
}

pub fn delete() -> BoxedFilter<(i32,)> {
    warp::delete().and(warp::path::param::<i32>()).boxed()
}
