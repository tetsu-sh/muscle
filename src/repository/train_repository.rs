use diesel::MysqlConnection;

use crate::{
    domain::train::{Train, TrainRepository},
    utils::errors::MyError,
};

use super::model::TrainRdbModel;

pub struct TrainRepositoryImpl<'a> {
    pub conn: &'a MysqlConnection,
}

impl TrainRepository for TrainRepositoryImpl<'_> {
    fn create(&self) {
        todo!()
    }

    fn fetch_one(&self, id: &String) -> Result<Train, MyError> {
        let train = TrainRdbModel::fetch(&self.conn, &id)?;
        println!("{:?}",train);
        Ok(train)
    }

    fn find_by_name(&self) {
        todo!()
    }
}
