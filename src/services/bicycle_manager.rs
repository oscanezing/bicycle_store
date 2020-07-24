use crate::{bikes::Bicycle, error::Error};

pub trait BicycleManager {
    fn create(&self, bike: Bicycle) -> Result<Bicycle, Error>;
    fn update(&self, bike: Bicycle) -> Result<Bicycle, Error>;
    fn delete(&self, id: i32) -> Result<bool, Error>;
    fn find_all(&self) -> Result<Vec<Bicycle>, Error>;
    fn find_by_id(&self, id: i32) -> Result<Bicycle, Error>;
}