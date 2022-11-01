use diesel::connection::{AnsiTransactionManager, TransactionManager};
use diesel::{Connection, MysqlConnection};

use crate::{
    domain::{
        muscle::Muscle,
        train::{Train, TrainRepository},
    },
    utils::errors::MyError,
};

use super::model::{TrainMuscleRdbModel, TrainRdbModel};

pub struct TrainRepositoryImpl<'a> {
    pub conn: &'a MysqlConnection,
}

impl TrainRepository for TrainRepositoryImpl<'_> {
    fn create(&self, train: Train, muscle_ids: Vec<String>) -> Result<(), MyError> {
        // store train model and connect train and muscles.
        // TODO transaction
        let tm = AnsiTransactionManager::new();
        let train_rdb_model = TrainRdbModel::from(train);
        train_rdb_model.save(&self.conn, train);
        for muscle_id in muscle_ids.iter() {
            let train_muscle_rdb_model = TrainMuscleRdbModel::new(*muscle_id, train.id);
            train_muscle_rdb_model.save_one(self.conn, train.id, *muscle_id);
        }
        Ok(())
    }

    fn fetch_one(&self, id: &String) -> Result<Train, MyError> {
        let train = TrainRdbModel::fetch_with_muscle(&self.conn, &id)?;
        Ok(train)
    }

    fn find_by_name(&self) {
        todo!()
    }
}
