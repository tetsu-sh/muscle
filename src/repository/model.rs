use std::str::FromStr;

use crate::domain::muscle::BodyPosition;
use crate::domain::muscle::Muscle;
use crate::domain::muscle::MuscleSize;
use crate::domain::train::Train;
use crate::schema::body_parts;
use crate::schema::muscles;
use crate::schema::train_muscle;
use crate::schema::trains;
use crate::utils::errors::MyError;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Deserialize, Serialize};

#[derive(
    Identifiable, Insertable, Queryable, Debug, Serialize, Deserialize, Associations, Clone,
)]
#[table_name = "trains"]
pub struct TrainRdbModel {
    pub id: String,
    pub name: String,
    pub volume: i32,
    pub rep: i32,
}

impl TrainRdbModel {
    pub fn from(train: Train) -> TrainRdbModel {
        TrainRdbModel {
            id: train.id,
            name: train.name,
            volume: train.volume,
            rep: train.rep,
        }
    }

    pub fn fetch_with_muscle(conn: &MysqlConnection, id: &String) -> Result<Train, MyError> {
        // trainとその子集約であるmuscleも取得する.
        let joined_muscle = muscles::table.left_join(body_parts::table);
        let train_id_joined_muscle = train_muscle::table.left_join(joined_muscle);
        let items = trains::table
            .inner_join(
                train_muscle::table.inner_join(muscles::table.inner_join(body_parts::table)),
            )
            .filter(trains::id.eq(id));

        let res = TrainRdbModel::to_domain(items);
        Ok(res)
    }

    pub fn to_domain(items: Vec<(TrainRdbModel, TrainMuscleRdbModel)>) -> Train {
        let muscles = vec![];
        for (trains_rdb_model, train_muscle_rdb_model) in items.iter() {
            muscles.push()
        }
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
#[derive(
    Identifiable, Insertable, Queryable, Debug, Serialize, Deserialize, Associations, Clone,
)]
#[table_name = "muscles"]
pub struct MuscleRdbModel {
    pub id: String,
    pub name: String,
    pub body_part_id: String,
    pub size: String,
}

impl MuscleRdbModel {
    pub fn from(muscle: Muscle, body_part_id: String) -> MuscleRdbModel {
        MuscleRdbModel {
            id: muscle.id,
            name: muscle.name,
            body_part_id,
            size: muscle.size.to_string(),
        }
    }

    fn from_domain() {}

    pub fn to_domain_with_body_part(item: (MuscleRdbModel, BodyPartRdbModel)) -> Muscle {
        Muscle::new(
            item.0.name,
            BodyPosition::from_str(&item.1.name).unwrap(),
            MuscleSize::from_str(&item.0.size).unwrap(),
        )
    }

    pub fn fetch(conn: &MysqlConnection, id: &String) -> Result<Muscle, MyError> {
        let item = muscles::table
            .inner_join(body_parts::table)
            .filter(muscles::id.eq(id))
            .first::<(MuscleRdbModel, BodyPartRdbModel)>(conn)?;
        Ok(MuscleRdbModel::to_domain_with_body_part(item))
    }

    pub fn save(self, conn: &MysqlConnection) -> Result<(), MyError> {
        diesel::insert_into(muscles::table)
            .values(&self)
            .execute(conn)?;
        Ok(())
    }
}

#[belongs_to(MuscleRdbModel, foreign_key = "muscle_id")]
#[belongs_to(TrainRdbModel, foreign_key = "train_id")]
#[derive(Insertable, Queryable, Debug, Serialize, Deserialize, Associations, Clone)]
#[table_name = "train_muscle"]
pub struct TrainMuscleRdbModel {
    pub muscle_id: String,
    pub train_id: String,
}

impl TrainMuscleRdbModel {
    pub fn new(muscle_id: String, train_id: String) -> Self {
        Self {
            muscle_id,
            train_id,
        }
    }

    pub fn fetch_by_train_muscle_id(
        conn: &MysqlConnection,
        train_id: &String,
        muscle_id: &String,
    ) -> Result<TrainMuscleRdbModel, MyError> {
        let item = train_muscle::table
            .find((train_id, muscle_id))
            .first(conn)?;
        Ok(item)
    }

    pub fn save_one(
        self,
        conn: &MysqlConnection,
        train_id: String,
        muscle_id: String,
    ) -> Result<(), MyError> {
        diesel::insert_into(train_muscle::table)
            .values(&self)
            .execute(conn)?;
        Ok(())
    }
}

#[derive(Insertable, Queryable, Debug, Serialize, Deserialize, Associations, Clone)]
#[table_name = "body_parts"]
pub struct BodyPartRdbModel {
    pub id: String,
    pub name: String,
}

impl BodyPartRdbModel {
    fn to_domain(body_part_rdb_model: BodyPartRdbModel) -> BodyPosition {
        BodyPosition::from_str(&body_part_rdb_model.name).unwrap()
    }

    pub fn save(self, conn: &MysqlConnection, id: String, name: String) -> Result<(), MyError> {
        diesel::insert_into(body_parts::table)
            .values(&self)
            .execute(conn)?;
        Ok(())
    }

    pub fn fetch(conn: &MysqlConnection, id: &String) -> Result<BodyPosition, MyError> {
        let item = body_parts::table.find(id).first(conn)?;
        Ok(BodyPartRdbModel::to_domain(item))
    }
}
