use crate::{repositories::bicycle::BicycleRepoInterface, error::Error, bikes::BicycleDomain};
use super::bicycle_manager::*;

#[derive(Clone)]
pub struct BicycleService<T> 
where 
    T: BicycleRepoInterface 
{
    pub repository: T,
}

impl<T> BicycleService<T> where T: BicycleRepoInterface {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

impl BicycleDomain {
    fn from_bicycle_in(origin: BicycleIn) -> Self {
        Self{
            id: None,
            description: origin.description,
            wheel_size: origin.wheel_size
        }
    }
}

impl<T> BicycleManager for BicycleService<T> 
where T: BicycleRepoInterface {
    fn create(&self, bike: BicycleIn) -> Result<BicycleOut, Error>  {
        let result = self.repository.create(BicycleDomain::from_bicycle_in(bike))?;
        Ok(BicycleOut::from_domain(result))
    }
    fn update(&self, _bike: BicycleIn) -> Result<BicycleOut, Error> {
        todo!()
    }
    fn delete(&self, _id: i32) -> Result<bool, Error> {
        todo!()
    }
    fn find_all(&self) -> Result<Vec<BicycleOut>, Error> {
        let result = self.repository.find_all()?.into_iter().map(|dm| {
            BicycleOut::from_domain(dm)
        }).collect::<Vec<BicycleOut>>();
        
        Ok(result)
    }
    fn find_by_id(&self, id: i32) -> Result<BicycleOut, Error> {
        let result = self.repository.find_by_id(id)?;
        Ok(BicycleOut::from_domain(result))
    }
}