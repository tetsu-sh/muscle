use std::str::FromStr;

use crate::utils::errors::MyError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use ulid::Ulid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Muscle {
    pub id: String,
    pub name: String,
    pub position: BodyPosition,
    pub size: MuscleSize,
}

impl Muscle {
    pub fn new(name: String, position: BodyPosition, size: MuscleSize) -> Self {
        let id = Ulid::new().to_string();
        Self {
            id,
            name,
            position,
            size,
        }
    }
    pub fn from(
        id: String,
        name: String,
        position: String,
        size: String,
    ) -> Result<Muscle, MyError> {
        let position = BodyPosition::from_str(&position)?;
        let size = MuscleSize::from_str(&size)?;
        let muscle = Muscle {
            id,
            name,
            position,
            size,
        };
        Ok(muscle)
    }
}

#[async_trait]
pub trait MuscleRepository {
    async fn create(&self, muscle: &Muscle, body_part_id: String) -> Result<(), MyError>;
    async fn fetch_one(&self, id: &String) -> Result<Muscle, MyError>;
    async fn fetch_by_train_id(&self, train_id: &String) -> Result<Vec<Muscle>, MyError>;
    fn find_by_name(&self, name: &String);
}

#[async_trait]
pub trait BodyPartRepository {
    async fn save(&self, id: String, body_position: BodyPosition) -> Result<(), MyError>;
    async fn fetch_one(&self, id: &String) -> Result<BodyPosition, MyError>;
    async fn find_by_name(&self, name: &String) -> Result<Option<BodyPosition>, MyError>;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumString, Display)]
pub enum BodyPosition {
    Arm,
    Back,
    Leg,
    Chest,
    Tolso,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumString, Display)]
pub enum MuscleSize {
    Large,
    Middle,
    Small,
}
