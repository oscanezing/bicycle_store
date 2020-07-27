use crate::{error::Error, controllers::model::*, bikes::BicycleDomain};

pub struct BicycleIn {
    pub wheel_size: i32,
    pub description: String,
}

impl BicycleIn {
    pub fn from_request(br: BicycleRequest) -> Self {
        BicycleIn {
            wheel_size: br.wheel_size,
            description: br.description,
        }
    }
}

pub struct BicycleOut {
    pub id: i32,
    pub wheel_size: i32,
    pub description: String,
}

impl BicycleOut {
    pub fn from_domain(origin: BicycleDomain) -> Self {
        BicycleOut {
            id: origin.id.unwrap_or(0),
            wheel_size: origin.wheel_size,
            description: origin.description,
        }
    }
}

pub trait BicycleManager {
    fn create(&self, bike: BicycleIn) -> Result<BicycleOut, Error>;
    fn update(&self, id: i32, bike: BicycleIn) -> Result<BicycleOut, Error>;
    fn delete(&self, id: i32) -> Result<bool, Error>;
    fn find_all(&self) -> Result<Vec<BicycleOut>, Error>;
    fn find_by_id(&self, id: i32) -> Result<BicycleOut, Error>;
}