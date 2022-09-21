use super::muscle::Muscle;
use crate::utils::errors::MyError;
use log::info;
use serde_json::json;
use ulid::Ulid;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Train {
    pub name: String,
    pub id: String,
    pub volume: i32,
    pub rep: i32,
    pub set: i32,
}
const NAME_LIMIT: i32 = 30;
const REP_LIMIT: i32 = 1000;
const SET_LIMIT: i32 = 1000;
const VOLUME_LIMIT: i32 = 1000;

impl Train {
    pub fn new(name: String, volume: i32, rep: i32, set: i32) -> Result<Self, MyError> {
        if volume > VOLUME_LIMIT {
            return Err(MyError::BadRequest(
                json!({"error":"volume must be less than 1000.you are possibly not human."}),
            ));
        };
        if rep > REP_LIMIT {
            return Err(MyError::BadRequest(
                json!({"error":"rep must be less than 1000.you are possibly not human."}),
            ));
        };
        if set > SET_LIMIT {
            return Err(MyError::BadRequest(
                json!({"error":"set must be less than 1000.you are possibly not human."}),
            ));
        };
        if name.chars().count() as i32 > NAME_LIMIT {
            return Err(MyError::BadRequest(
                json!({"error":"train name must be less than 30 letters"}),
            ));
        }
        let id = Ulid::new().to_string();
        Ok(Self {
            name,
            id,
            volume,
            rep,
            set,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TrainTemplate {
    name: String,
    id: String,
    muscle: Vec<Muscle>,
}

impl TrainTemplate {
    fn new(name: String, muscle: Vec<Muscle>) -> Self {
        Self {
            name,
            id: Uuid::new_v4().to_string(),
            muscle,
        }
    }
}

pub trait TrainRepository {
    fn create(&self);
    fn fetch_one(&self, id: &String) -> Result<Train, MyError>;
    fn find_by_name(&self);
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_train_new() {
        let test_name = "x".to_string().repeat(30);
        let test_vol = 100;
        let test_rep = 10;
        let test_set = 10;
        let train = Train::new(test_name.clone(), test_vol, test_rep, test_set).unwrap();
        assert_eq!(train.name, test_name);
        assert_eq!(train.volume, test_vol);
        assert_eq!(train.rep, test_rep);
        assert_eq!(train.set, test_set);
    }

    #[test]
    fn test_train_new_failed() {
        let test_name = "x".to_string().repeat((NAME_LIMIT + 1) as usize);
        let test_vol = 100;
        let test_rep = 10;
        let test_set = 10;
        let train = Train::new(test_name.clone(), test_vol, test_rep, test_set).unwrap_err();
        assert_eq!(
            train,
            MyError::BadRequest(json!({"error":"train name must be less than 30 letters"}))
        );
    }
}
