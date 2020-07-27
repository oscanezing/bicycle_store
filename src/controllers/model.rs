use serde::{
    Serialize, 
    Deserialize
};
use crate::services::bicycle_manager::*;

#[derive(Deserialize)]
pub struct BicycleRequest {
    pub wheel_size: i32,
    pub description: String,
}

#[derive(Serialize)]
pub struct BicycleResponse {
    pub id: i32,
    pub wheel_size: i32,
    pub description: String,
}

impl BicycleResponse {
    pub fn of(bicycle: BicycleOut) -> BicycleResponse {
        BicycleResponse {
            id: bicycle.id,
            wheel_size: bicycle.wheel_size,
            description: bicycle.description,
        }
    }
}