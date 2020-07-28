use crate::{repositories::bicycle::BicycleRepoInterface, error::Error, bikes::*};
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
        let bike_dm = BicycleDomain::from_bicycle_in(bike);
        bike_dm.is_valid()?;
        let result = self.repository.create(bike_dm)?;
        Ok(BicycleOut::from_domain(result))
    }
    fn update(&self, bike_id: i32, bike: BicycleIn) -> Result<BicycleOut, Error> {
        let mut bike_domain = BicycleDomain::from_bicycle_in(bike);
        bike_domain.is_valid()?;
        bike_domain.id = Some(bike_id);
        let result = self.repository.update(bike_domain)?;
        Ok(BicycleOut::from_domain(result))
    }
    fn delete(&self, id: i32) -> Result<bool, Error> {
        self.repository.delete(id)
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

#[cfg(test)]
mod tests {
    use super::*;
    struct RepoMock {}

    impl BicycleRepoInterface for RepoMock {
        fn create(&self, bike: BicycleDomain) -> Result<BicycleDomain, Error>  {
            if bike.wheel_size > 0 {
                Ok(bike)
            } else {
                Err(Error::DBQueryError(diesel::result::Error::NotFound))
            }
        }
        fn update(&self, bike: BicycleDomain) -> Result<BicycleDomain, Error> {
            if bike.id > Some(0) {
                Ok(bike)
            } else {
                Err(Error::DBQueryError(diesel::result::Error::NotFound))
            }
        }
        fn delete(&self, id: i32) -> Result<bool, Error> {
            if id > 0 {
                Ok(true)
            } else {
                Err(Error::DBQueryError(diesel::result::Error::NotFound))
            }
        }
        fn find_all(&self) -> Result<Vec<BicycleDomain>, Error> {
            let mut result = vec![];
            result.push(BicycleDomain{
                id: Some(1),
                description: String::from("some description"),
                wheel_size: 21
            });
            result.push(BicycleDomain{
                id: Some(2),
                description: String::from("some description 1"),
                wheel_size: 22
            });
            Ok(result)
        }
        fn find_by_id(&self, id: i32) -> Result<BicycleDomain, Error> {
            if id > 0 {
                Ok(BicycleDomain{
                    id: Some(1),
                    description: String::from("some description"),
                    wheel_size: 21
                })
            } else {
                Err(Error::DBQueryError(diesel::result::Error::NotFound))
            }
        }
    }

    fn new_service() -> BicycleService<RepoMock> {
        let repo = RepoMock{};
        BicycleService{repository: repo}
    }

    fn bike_in(wheel_size: i32, description: &str) -> BicycleIn {
        BicycleIn {
            description: String::from(description),
            wheel_size
        }
    }

    #[test]
    fn create_success() {
        let service = new_service();
        let bike_in = bike_in(21, "some description");
        let result = service.create(bike_in);
        assert!(result.is_ok());
        assert!(result.unwrap().wheel_size.eq(&21));
    }

    #[test]
    fn create_inivalid_wheel() {
        let service = new_service();
        let bike_in = bike_in(31, "invalid wheel size");
        let result = service.create(bike_in);
        
        assert!(!result.is_ok());
        
        let er = format!("{}", result.err().unwrap());
        assert!(er.contains("invalid wheel size"));
    }

    #[test]
    fn create_inivalid_description() {
        let service = new_service();
        let bike_in = bike_in(29, 
            "this is an invalid description because contains more than 100 characters.
            this is an invalid description because contains more than 100 characters");
        let result = service.create(bike_in);
        
        assert!(!result.is_ok());
        
        let er = format!("{}", result.err().unwrap());
        assert!(er.contains("invalid description"));
    }

    #[test]
    fn update_inivalid_wheel() {
        let service = new_service();
        let bike_in = bike_in(31, "invalid wheel size");
        let result = service.update(1, bike_in);
        
        assert!(!result.is_ok());
        
        let er = format!("{}", result.err().unwrap());
        assert!(er.contains("invalid wheel size"));
    }

    #[test]
    fn udpate_inivalid_description() {
        let service = new_service();
        let bike_in = bike_in(29, 
            "this is an invalid description because contains more than 100 characters.
            this is an invalid description because contains more than 100 characters");
        let result = service.update(1, bike_in);
        
        assert!(!result.is_ok());
        
        let er = format!("{}", result.err().unwrap());
        assert!(er.contains("invalid description"));
    }
}