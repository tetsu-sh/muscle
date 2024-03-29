use super::muscle::Muscle;
use crate::utils::errors::MyError;
use async_trait::async_trait;
use serde_json::json;
use ulid::Ulid;

/// represent training detail and strength.
#[derive(Debug, Clone, PartialEq)]
pub struct Train {
    pub name: String,
    pub id: String,
    // means weight of weight
    pub volume: i32,
    // times
    pub rep: i32,
    // muscles using for this training.
    pub muscles: Vec<Muscle>,
}
const NAME_LIMIT: i32 = 30;
const REP_LIMIT: i32 = 1000;
const VOLUME_LIMIT: i32 = 1000;

impl Train {
    /// create new Train Object with new id.
    pub fn new(name: String, volume: i32, rep: i32, muscles: Vec<Muscle>) -> Result<Self, MyError> {
        if volume > VOLUME_LIMIT {
            return Err(MyError::BadRequest(
                json!({"error":"volume must be less than 1000. you are possibly not human."}),
            ));
        };
        if rep > REP_LIMIT {
            return Err(MyError::BadRequest(
                json!({"error":"rep must be less than 1000. you are possibly not human."}),
            ));
        };
        if name.chars().count() as i32 > NAME_LIMIT {
            return Err(MyError::BadRequest(
                json!({"error":"train name must be less than 30 letters"}),
            ));
        };
        let id = Ulid::new().to_string();
        Ok(Self {
            name,
            id,
            volume,
            rep,
            muscles,
        })
    }
}
#[async_trait]
pub trait TrainRepository {
    /// store Train to DB.
    async fn save(&self, train: &Train, account_id: &String) -> Result<(), MyError>;
    /// find one Train from DB by primary key. return Train. if not exist,None.
    async fn fetch_one(&self, id: &String) -> Result<Train, MyError>;
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
        let test_muscle = Muscle::new(
            "torsow".to_string(),
            crate::domain::muscle::BodyPosition::Tolso,
            crate::domain::muscle::MuscleSize::Middle,
        );
        let train = Train::new(test_name.clone(), test_vol, test_rep, vec![test_muscle]).unwrap();
        assert_eq!(train.name, test_name);
        assert_eq!(train.volume, test_vol);
        assert_eq!(train.rep, test_rep);
    }

    #[test]
    fn test_train_new_failed() {
        let test_name = "x".to_string().repeat((NAME_LIMIT + 1) as usize);
        let test_vol = 100;
        let test_rep = 10;
        let test_muscle = Muscle::new(
            "torsow".to_string(),
            crate::domain::muscle::BodyPosition::Tolso,
            crate::domain::muscle::MuscleSize::Middle,
        );
        let train = Train::new(test_name, test_vol, test_rep, vec![test_muscle]).unwrap_err();
        assert_eq!(
            train,
            MyError::BadRequest(json!({"error":"train name must be less than 30 letters"}))
        );
    }
}
