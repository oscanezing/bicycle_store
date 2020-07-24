use crate::{ 
     RejectionResult, 
     services::bicycle::BicycleService, 
     repositories::postgres_repo::BicycleRepoPostgres,
     services::bicycle_manager::*};
use warp::{reject, Reply, reply::json};
use super::model::*;

pub async fn find_all(manager: BicycleService<BicycleRepoPostgres>) -> RejectionResult<impl Reply> {
    let result = manager.find_all().map_err(|e| reject::custom(e));
    Ok(json::<Vec<_>>(&result.unwrap().into_iter().map(|bike| BicycleResponse::of(bike) ).collect()))
}

pub async fn create(bicycle_request: BicycleRequest, manager: BicycleService<BicycleRepoPostgres>) -> RejectionResult<impl Reply> {
    let bicycle_in = BicycleIn::from_request(bicycle_request);
    let result = manager.create(bicycle_in).map_err(|e| reject::custom(e))?;
    Ok(json::<_>(&BicycleResponse::of(result)))
}


