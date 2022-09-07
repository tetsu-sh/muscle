
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use crate::utils::errors::MyError;
use crate::domain::train::Train;
use crate::schema::trains;

#[derive(Queryable)]
pub struct TrainRdbModel{
    pub id:i32,
    pub name:String,
    pub volume:i32,
    pub rep:i32,
    pub set:i32,
}

impl TrainRdbModel {
    pub fn from(){

    }

    pub fn to_domain(train:TrainRdbModel)->Train{
        Train{name:train.name,id:train.id, volume:train.volume, rep:train.rep, set:train.set}
    }
    pub fn fetch(conn:&MysqlConnection,id:i32)->Result<TrainRdbModel,MyError>{
        let item=trains::table.load::<TrainRdbModel>(conn)?;
        Ok(item[0])

    }
}


#[derive(Insertable)]
#[table_name="trains"]
pub struct CreateTrain {
    pub id: i32,
    pub name: String,
    pub volume: i32,
    pub rep: i32,
    pub set: i32,
}


pub struct DeleteTrain{
    pub id:i32,
}



