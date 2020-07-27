use crate::{
    error::Error, 
    schema::{
        bicycles, 
        bicycles::dsl::*
    }, 
    bikes::BicycleDomain, 
    datasource::db,
    diesel::RunQueryDsl
};
use super::bicycle::BicycleRepoInterface;
use diesel::{
    ExpressionMethods, 
    QueryDsl
};
use serde::{
    Deserialize, 
    Serialize
};

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "bicycles"]
pub struct NewBicycle {
    pub wheel_size: i32,
    pub description: String,
}

impl NewBicycle {
    fn from_domain(origin: BicycleDomain) -> Self {
        Self {
            description: origin.description,
            wheel_size: origin.wheel_size,
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
struct Bicycle {
    #[primary_key]
    pub id: i32,
    pub wheel_size: i32,
    pub description: String,
}

impl Bicycle {
    fn from_domain(origin: &BicycleDomain) -> Self {
        Self {
            id: origin.id.unwrap(),
            description: String::from(&origin.description),
            wheel_size: origin.wheel_size
        }
    }
}

impl BicycleDomain {
    fn from_bicycle(origin: Bicycle) -> Self {
        Self {
            id: Some(origin.id),
            description: origin.description,
            wheel_size: origin.wheel_size
        }
    }
}

#[derive(Clone)]
pub struct BicycleRepoPostgres {}

impl BicycleRepoInterface for BicycleRepoPostgres {
    fn create(&self, bike: BicycleDomain) -> Result<BicycleDomain, Error>  {
        let conn = db::connection()?;
        let result = diesel::insert_into(bicycles).values(NewBicycle::from_domain(bike)).get_result(&conn)?;
        Ok(BicycleDomain::from_bicycle(result))
    }
    fn update(&self, bike: BicycleDomain) -> Result<BicycleDomain, Error> {
        let conn = db::connection()?;
        let to_update = Bicycle::from_domain(&bike);
        let updated = diesel::update(bicycles.filter(bicycles::id.eq(to_update.id)))
            .set(to_update)
            .get_result(&conn)?;
        Ok(BicycleDomain::from_bicycle(updated))
    }
    fn delete(&self, bike_id: i32) -> Result<bool, Error> {
        let conn = db::connection()?;
        let result = diesel::delete(bicycles.filter(bicycles::id.eq(bike_id))).execute(&conn)?;
        Ok(result > 0)
    }
    fn find_all(&self) -> Result<Vec<BicycleDomain>, Error> {
        let conn = db::connection()?;
        let db_results = bicycles.load::<Bicycle>(&conn)?;
        
        let results: Vec<BicycleDomain> = db_results.into_iter().map(|db_data| {
            BicycleDomain {
                id: Some(db_data.id),
                description: db_data.description,
                wheel_size: db_data.wheel_size,
            }
        }).collect();
        Ok(results)
    }
    fn find_by_id(&self, bike_id: i32) -> Result<BicycleDomain, Error> {
        let conn = db::connection()?;
        let db_result = bicycles.find(bike_id).first(&conn)?;
        Ok(BicycleDomain::from_bicycle(db_result))
    }
}

