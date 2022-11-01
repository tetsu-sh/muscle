use log::info;

use crate::{
    domain::muscle::{Muscle, MuscleRepository},
    domain::train::{Train, TrainRepository},
    utils::errors::MyError,
};

pub struct TrainUsecase<T: TrainRepository, M: MuscleRepository> {
    pub train_repository: T,
    pub muscle_repository: M,
}

impl<T: TrainRepository, M: MuscleRepository> TrainUsecase<T, M> {
    pub fn new(train_repository: T, muscle_repository: M) -> Self {
        Self {
            train_repository,
            muscle_repository,
        }
    }

    pub fn create_train(
        &self,
        name: String,
        volume: i32,
        set: i32,
        rep: i32,
        muscle_ids: Vec<String>,
    ) -> Result<Train, MyError> {
        let train = Train::new(name, volume, set)?;
        let train_creater = self.train_repository.create(train);
        Ok(train)
    }

    fn train(_: Self, trains: Vec<Train>) -> Train {
        todo!()
    }

    pub fn fetch_one(&self, id: &String) -> Result<Train, MyError> {
        self.train_repository.fetch_one(id)
    }
}
