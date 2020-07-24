use crate::{ 
     RejectionResult, 
     bikes::BicycleResponse,
     services::bicycle::BicycleService, 
     repositories::postgres_repo::BicycleRepoPostgres,
     services::bicycle_manager::BicycleManager};
use warp::{reject, Reply, reply::json};

pub async fn find_all(manager: BicycleService<BicycleRepoPostgres>) -> RejectionResult<impl Reply> {
    let result = manager.find_all().map_err(|e| reject::custom(e));
    Ok(json::<Vec<_>>(&result.unwrap().into_iter().map(|bike| BicycleResponse::of(bike) ).collect()))
    // let conn = db::get_db_con(&db_pool).await.map_err(|e| reject::custom(e))?;
    // let results = bicycles.load::<Bicycle>(&conn).map_err(|e| reject::custom(DBQueryError(e)));
    // Ok(json::<Vec<_>>(&results.unwrap().into_iter().map(|bike| BicycleResponse::of(bike) ).collect()))
}