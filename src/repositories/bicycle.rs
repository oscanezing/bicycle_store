use crate::{bikes::BicycleDomain, error::Error};

pub trait BicycleRepoInterface {
    fn create(&self, bike: BicycleDomain) -> Result<BicycleDomain, Error>;
    fn update(&self, bike: BicycleDomain) -> Result<BicycleDomain, Error>;
    fn delete(&self, id: i32) -> Result<bool, Error>;
    fn find_all(&self) -> Result<Vec<BicycleDomain>, Error>;
    fn find_by_id(&self, id: i32) -> Result<BicycleDomain, Error>;
}
