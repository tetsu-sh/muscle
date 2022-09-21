use crate::domain::train::Train;
use crate::schema::trains;
use crate::utils::errors::MyError;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct TrainRdbModel {
    pub id: String,
    pub name: String,
    pub volume: i32,
    pub rep: i32,
    pub set: i32,
}

impl TrainRdbModel {
    pub fn from(self) {}

    pub fn to_domain(&self) -> Train {
        Train {
            name: self.name.clone(),
            id: self.id.clone(),
            volume: self.volume,
            rep: self.rep,
            set: self.set,
        }
    }
    pub fn fetch(conn: &MysqlConnection, id: &String) -> Result<Train, MyError> {
        let item = trains::table.load::<TrainRdbModel>(conn)?;
        Ok(item[0].to_domain())
    }
}

#[derive(Insertable)]
#[table_name = "trains"]
pub struct CreateTrain {
    pub id: String,
    pub name: String,
    pub volume: i32,
    pub rep: i32,
    pub set: i32,
}

pub struct DeleteTrain {
    pub id: i32,
}
