use log::info;

use crate::{
    domain::train::{Train, TrainRepository},
    utils::errors::MyError,
};

pub struct TrainUsecase<T: TrainRepository> {
    pub train_repository: T,
}

impl<T: TrainRepository> TrainUsecase<T> {
    pub fn new(train_repository: T) -> Self {
        Self { train_repository }
    }

    pub fn create_train(
        &self,
        name: String,
        volume: i32,
        set: i32,
        rep: i32,
    ) -> Result<Train, MyError> {
        let train = Train::new(name, volume, set, rep)?;
        self.train_repository.create();
        Ok(train)
    }

    fn train(_: Self, trains: Vec<Train>) -> Train {
        todo!()
    }
}
