use crate::domain::muscle::BodyPosition;
use crate::domain::muscle::MuscleSize;
use crate::domain::train::Train;
use crate::schema::body_parts;
use crate::schema::muscles;
use crate::schema::trains;
use crate::utils::errors::MyError;
use diesel::mysql::MysqlConnection;
use diesel::Insertable;

#[derive(Queryable, Insertable)]
#[table_name = "trains"]
pub struct TrainRdbModel {
    pub id: String,
    pub name: String,
    pub volume: i32,
    pub rep: i32,
}

impl TrainRdbModel {
    pub fn from(self, train: Train) -> TrainRdbModel {
        TrainRdbModel {
            id: train.id,
            name: train.name,
            volume: train.volume,
            rep: train.rep,
        }
    }

    pub fn to_domain(&self) -> Train {
        Train {
            name: self.name.clone(),
            id: self.id.clone(),
            volume: self.volume,
            rep: self.rep,
        }
    }
    pub fn fetch(conn: &MysqlConnection, id: &String) -> Result<Train, MyError> {
        let item = trains::table.load::<TrainRdbModel>(conn)?;
        Ok(item[0].to_domain())
    }

    pub fn save(self, conn: &MysqlConnection, train: Train) -> Result<(), MyError> {
        diesel::insert_into(trains::table)
            .values(&self)
            .execute(conn)?;
        Ok(())
    }
}

#[derive(Insertable)]
#[table_name = "trains"]
pub struct CreateTrain {
    pub id: String,
    pub name: String,
    pub volume: i32,
    pub rep: i32,
}

pub struct DeleteTrain {
    pub id: i32,
}

#[belongs_to(BodyPartRdbModel, foreign_key = "body_part_id")]
#[derive(Queryable, Insertable)]
#[table_name = "muscles"]
pub struct MuscleRdbModel {
    pub id: String,
    pub name: String,
    pub body_part_id: String,
    pub size: MuscleSize,
}

impl MuscleRdbModel {
    fn new() {}

    fn from_domain() {}

    fn to_domain() {}
}

#[derive(Queryable, Insertable)]
#[table_name = "body_parts"]
pub struct BodyPartRdbModel {
    pub id: String,
    pub name: BodyPosition,
}

impl BodyPartRdbModel {
    fn from() {}

    fn to_domain() {}
}
