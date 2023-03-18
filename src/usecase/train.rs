use log::info;

use crate::{
    domain::muscle::MuscleRepository,
    domain::train::{Train, TrainRepository},
    utils::errors::MyError,
};

pub struct TrainUsecase {
    pub train_repository: Box<dyn TrainRepository>,
    pub muscle_repository: Box<dyn MuscleRepository>,
}

impl TrainUsecase {
    pub fn new(
        train_repository: Box<dyn TrainRepository>,
        muscle_repository: Box<dyn MuscleRepository>,
    ) -> Self {
        Self {
            train_repository,
            muscle_repository,
        }
    }

    /// create new train. connect train with muscle specified by client.
    pub async fn create_train(
        &self,
        account_id: &String,
        name: String,
        volume: i32,
        rep: i32,
        muscle_ids: Vec<String>,
    ) -> Result<Train, MyError> {
        let mut muscles = vec![];
        for muscle_id in muscle_ids {
            muscles.push(self.muscle_repository.fetch_one(&muscle_id).await?.unwrap());
        }
        let train = Train::new(name, volume, rep, muscles)?;
        self.train_repository.save(&train, account_id);
        Ok(train)
    }

    pub async fn fetch_one(&self, id: &String) -> Result<Train, MyError> {
        self.train_repository.fetch_one(id).await
    }
}
