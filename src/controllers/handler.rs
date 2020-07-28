use crate::{
    services::{
        bicycle::BicycleService, 
        bicycle_manager::*
    }, 
    repositories::postgres_repo::BicycleRepoPostgres
    };
use warp::{
    reject, 
    Reply, 
    reply::json, 
    Rejection, 
    hyper::StatusCode
};
use super::model::*;

type RejectionResult<T> = std::result::Result<T, Rejection>;

pub async fn find_all(manager: BicycleService<BicycleRepoPostgres>) -> RejectionResult<impl Reply> {
    let result = manager.find_all().map_err(|e| reject::custom(e));
    Ok(json::<Vec<_>>(&result.unwrap().into_iter().map(|bike| BicycleResponse::of(bike) ).collect()))
}

pub async fn find_by_id(manager: BicycleService<BicycleRepoPostgres>, id: i32) -> RejectionResult<impl Reply> {
    let result = manager.find_by_id(id).map_err(|e| reject::custom(e))?;
    Ok(json::<_>(&BicycleResponse::of(result)))
}

pub async fn create(manager: BicycleService<BicycleRepoPostgres>, bicycle_request: BicycleRequest) -> RejectionResult<impl Reply> {
    let bicycle_in = BicycleIn::from_request(bicycle_request);
    let result = manager.create(bicycle_in).map_err(|e| reject::custom(e))?;
    Ok(json::<_>(&BicycleResponse::of(result)))
}

pub async fn update(manager: BicycleService<BicycleRepoPostgres>, id: i32, bicycle_request: BicycleRequest) -> RejectionResult<impl Reply> {
    let bike_in = BicycleIn::from_request(bicycle_request);
    let result = manager.update(id, bike_in).map_err(|e| reject::custom(e))?;
    Ok(json::<_>(&BicycleResponse::of(result)))
}

pub async fn delete(manager: BicycleService<BicycleRepoPostgres>, id: i32) -> RejectionResult<impl Reply> {
    let result = manager.delete(id).map_err(|e| reject::custom(e))?;
    match result {
        true => Ok(StatusCode::OK),
        false => Ok(StatusCode::NOT_FOUND)
    }
}



