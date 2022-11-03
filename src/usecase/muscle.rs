use serde_json::json;
use std::str::FromStr;

use crate::{
    domain::muscle::{BodyPartRepository, BodyPosition, Muscle, MuscleRepository, MuscleSize},
    utils::errors::MyError,
};

pub struct MuscleUsecase<M: MuscleRepository, P: BodyPartRepository> {
    pub muscle_repository: M,
    pub body_part_repository: P,
}

impl<M: MuscleRepository, P: BodyPartRepository> MuscleUsecase<M, P> {
    pub fn new(muscle_repository: M, body_part_repository: P) -> Self {
        Self {
            muscle_repository,
            body_part_repository,
        }
    }

    pub async fn create_muscle(
        &self,
        name: String,
        body_part_id: String,
        size: String,
    ) -> Result<Muscle, MyError> {
        let body_position = self.body_part_repository.fetch_one(&body_part_id).await?;
        let size = MuscleSize::from_str(&size)?;
        let muscle = Muscle::new(name, body_position, size);
        self.muscle_repository.create(&muscle, body_part_id);
        Ok(muscle)
    }

    pub async fn fetch_muscle(&self, id: &String) -> Result<Muscle, MyError> {
        let muscle = self.muscle_repository.fetch_one(id).await?;
        Ok(muscle.unwrap())
    }

    pub async fn create_body_part(&self, name: String) -> Result<(BodyPosition, String), MyError> {
        let body_part_record = self.body_part_repository.find_by_name(&name).await?;
        println!("{:?}", body_part_record);
        if body_part_record.is_some() {
            return Err(MyError::BadRequest(json!({
                "error":"same name are not forbidden."
            })));
        };

        // want: reorganize it to domain layer.
        let id = ulid::Ulid::new().to_string();

        self.body_part_repository
            .save(&id, BodyPosition::from_str(&name)?);
        Ok((BodyPosition::from_str(&name)?, id))
    }
}
