use diesel::MysqlConnection;

use crate::{domain::train::{ TrainRepository, Train}, utils::errors::MyError};

use super::model::{TrainRdbModel};


pub struct TrainRepositoryImpl<'a> {
    conn:&'a MysqlConnection
}

impl TrainRepository for TrainRepositoryImpl<'_> {
    fn create(&self) {
        todo!()
    }

    fn fetch_one(&self,id:i32)->Result<Train,MyError>{
        let train=TrainRdbModel::fetch(&self.conn,id)?;
        Ok(train)
    }

    fn find_by_name(&self){
        todo!()
    }
}
