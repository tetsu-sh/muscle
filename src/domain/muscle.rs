use crate::utils::errors::MyError;
use std::str::FromStr;
use strum::{Display, EnumString};
use ulid::Ulid;

#[derive(Debug, Clone, PartialEq)]
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
}

pub trait MuscleRepository {
    fn create(&self, muscle: Muscle, body_part_id: String) -> Result<(), MyError>;
    fn fetch_one(&self, id: &String) -> Result<Muscle, MyError>;
    fn find_by_name(&self);
}

#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display)]
pub enum BodyPosition {
    Arm,
    Back,
    Leg,
    Chest,
    Tolso,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display)]
pub enum MuscleSize {
    Large,
    Middle,
    Small,
}
