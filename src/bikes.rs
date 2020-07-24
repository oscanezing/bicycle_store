use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BicycleDomain {
    pub id: Option<i32>,
    pub wheel_size: i32,
    pub description: String
}