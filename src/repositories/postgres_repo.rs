use crate::{DBPool, error::Error, error::Error::*, schema::bicycles::dsl::*, bikes::Bicycle};
use super::bicycle::BicycleRepoInterface;
use crate::diesel::RunQueryDsl;

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
    fn create(&self, _bike: Bicycle) -> Result<Bicycle, Error>  {
        todo!()
    }
    fn update(&self, _bike: Bicycle) -> Result<Bicycle, Error> {
        todo!()
    }
    fn delete(&self, _bike_id: i32) -> Result<bool, Error> {
        todo!()
    }
    fn find_all(&self) -> Result<Vec<Bicycle>, Error> {
        let conn = self.connection_pool.get().map_err(|e| DBPoolError(e));
        let results = bicycles.load::<Bicycle>(&conn.unwrap())?;
        Ok(results)
    }
    fn find_by_id(&self, _bike_id: i32) -> Result<Bicycle, Error> {
        todo!()
    }
    
}