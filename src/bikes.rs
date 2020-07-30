use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BicycleDomain {
    pub id: Option<i32>,
    pub wheel_size: i32,
    pub description: String,
}

impl BicycleDomain {
    pub fn is_valid(&self) -> Result<bool, Error> {
        if self.description.len() > 100 {
            return Err(Error::BadDescriptionLen);
        } else if self.wheel_size > 30 {
            return Err(Error::BadWheelSize);
        }
        Ok(true)
    }
}
