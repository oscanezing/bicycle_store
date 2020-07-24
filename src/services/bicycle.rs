use crate::{repositories::bicycle::BicycleRepoInterface, bikes::Bicycle, error::Error};
use super::bicycle_manager::BicycleManager;

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

impl<T> BicycleManager for BicycleService<T> 
where T: BicycleRepoInterface {
    fn create(&self, _bike: Bicycle) -> Result<Bicycle, Error>  {
        todo!()
    }
    fn update(&self, _bike: Bicycle) -> Result<Bicycle, Error> {
        todo!()
    }
    fn delete(&self, _id: i32) -> Result<bool, Error> {
        todo!()
    }
    fn find_all(&self) -> Result<Vec<Bicycle>, Error> {
       self.repository.find_all()
    }
    fn find_by_id(&self, _id: i32) -> Result<Bicycle, Error> {
        todo!()
    }
}