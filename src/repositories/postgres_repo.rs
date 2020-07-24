use crate::{DBPool, error::Error, error::Error::*, schema::bicycles, schema::bicycles::dsl::*, bikes::BicycleDomain};
use super::bicycle::BicycleRepoInterface;
use crate::diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "bicycles"]
pub struct NewBicycle {
    pub wheel_size: i32,
    pub description: String,
}

impl NewBicycle {
    fn from_domain(origin: BicycleDomain) -> Self {
        Self {
            description: origin.description,
            wheel_size: origin.wheel_size,
        }
    }
}

#[derive(Serialize, Deserialize, Queryable)]
struct Bicycle {
    pub id: i32,
    pub wheel_size: i32,
    pub description: String,
}

impl BicycleDomain {
    fn from_bicycle(origin: Bicycle) -> Self {
        Self {
            id: Some(origin.id),
            description: origin.description,
            wheel_size: origin.wheel_size
        }
    }
}

#[derive(Clone)]
pub struct BicycleRepoPostgres {
    pub connection_pool: DBPool,
}

impl BicycleRepoPostgres {
    pub fn new(connection_pool: DBPool) -> Self {
        Self {connection_pool}
    }
}

impl BicycleRepoInterface for BicycleRepoPostgres {
    fn create(&self, bike: BicycleDomain) -> Result<BicycleDomain, Error>  {
        let conn = self.connection_pool.get().map_err(|e| DBPoolError(e))?;
        let result = diesel::insert_into(bicycles).values(NewBicycle::from_domain(bike)).get_result(&conn)?;
        Ok(BicycleDomain::from_bicycle(result))
    }
    fn update(&self, _bike: BicycleDomain) -> Result<BicycleDomain, Error> {
        todo!()
    }
    fn delete(&self, _bike_id: i32) -> Result<bool, Error> {
        todo!()
    }
    fn find_all(&self) -> Result<Vec<BicycleDomain>, Error> {
        let conn = self.connection_pool.get().map_err(|e| DBPoolError(e));
        let db_results = bicycles.load::<Bicycle>(&conn.unwrap())?;
        
        let results: Vec<BicycleDomain> = db_results.into_iter().map(|db_data| {
            BicycleDomain {
                id: Some(db_data.id),
                description: db_data.description,
                wheel_size: db_data.wheel_size,
            }
        }).collect();
        Ok(results)
    }
    fn find_by_id(&self, _bike_id: i32) -> Result<BicycleDomain, Error> {
        todo!()
    }
}

