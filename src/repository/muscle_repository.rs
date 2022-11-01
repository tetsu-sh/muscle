use diesel::MysqlConnection;

use crate::{
    domain::{muscle::Muscle, muscle::MuscleRepository},
    utils::errors::MyError,
};

use super::model::MuscleRdbModel;

pub struct MuscleRepositoryImpl<'a> {
    pub conn: &'a MysqlConnection,
}

impl MuscleRepository for MuscleRepositoryImpl<'_> {
    fn create(&self, muscle: Muscle, body_part_id: String) -> Result<(), MyError> {
        let muscle_rdb_model = MuscleRdbModel::from(muscle, body_part_id);
        muscle_rdb_model.save(&self.conn)
    }

    fn fetch_one(&self, id: &String) -> Result<Muscle, MyError> {
        let muscle = MuscleRdbModel::fetch(&self.conn, &id)?;
        Ok(muscle)
    }

    fn find_by_name(&self) {
        todo!()
    }
}
