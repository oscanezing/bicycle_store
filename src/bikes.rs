use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Bicycle {
    pub id: i32,
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
    pub fn of(bicycle: Bicycle) -> BicycleResponse {
        BicycleResponse {
            id: bicycle.id,
            wheel_size: bicycle.wheel_size,
            description: bicycle.description,
        }
    }
}