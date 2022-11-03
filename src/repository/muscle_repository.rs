use std::str::FromStr;

use crate::{
    domain::{
        muscle::Muscle,
        muscle::{BodyPartRepository, BodyPosition, MuscleRepository},
    },
    utils::errors::MyError,
};
use async_trait::async_trait;

use sqlx::MySqlPool;

pub struct MuscleRepositoryImpl<'a> {
    pub conn: &'a MySqlPool,
}

#[derive(PartialEq, Debug)]
struct MusclesQueryModel {
    id: String,
    name: String,
    position: String,
    size: String,
}

#[async_trait]
impl MuscleRepository for MuscleRepositoryImpl<'_> {
    async fn create(&self, muscle: &Muscle, body_part_id: String) -> Result<(), MyError> {
        sqlx::query!(
            "insert into muscles
            values(?,?,?,?)
            ",
            muscle.id,
            muscle.name,
            body_part_id,
            muscle.size.to_string()
        )
        .execute(self.conn)
        .await?;
        Ok(())
    }

    async fn fetch_one(&self, id: &String) -> Result<Muscle, MyError> {
        let record = sqlx::query!(
            "select muscles.id,muscles.name,muscles.size,body_parts.name as position
            from muscles join body_parts on muscles.body_part_id=body_parts.id 
            where muscles.id=? 
            ",
            id
        )
        .fetch_one(self.conn)
        .await?;
        let muscle = Muscle::from(record.id, record.name, record.position, record.size)?;
        Ok(muscle)
    }

    async fn fetch_by_train_id(&self, train_id: &String) -> Result<Vec<Muscle>, MyError> {
        let muscles_iter = sqlx::query!(
            "select muscles.id,muscles.name,muscles.size,body_parts.name as position
        from muscles join train_muscle on muscles.id=train_muscle.muscle_id 
        join body_parts on muscles.body_part_id=body_parts.id 
        where train_muscle.train_id=?",
            train_id
        )
        .fetch_all(self.conn)
        .await?
        .into_iter()
        .map(move |x| Muscle::from(x.id, x.name, x.position, x.size));
        Ok(muscles_iter.map(Result::unwrap).collect())
    }

    fn find_by_name(&self, name: &String) {
        todo!()
    }
}

pub struct BodyPartRepositoryImpl<'a> {
    pub conn: &'a MySqlPool,
}

#[async_trait]
impl BodyPartRepository for BodyPartRepositoryImpl<'_> {
    async fn save(&self, id: String, body_part: BodyPosition) -> Result<(), MyError> {
        println!("bodypart create");
        sqlx::query!(
            "insert into body_parts
            values(?,?)
            ",
            id,
            body_part.to_string()
        )
        .execute(self.conn)
        .await?;
        Ok(())
    }

    async fn fetch_one(&self, id: &String) -> Result<BodyPosition, MyError> {
        let record = sqlx::query!(
            "select id,name
            from body_parts 
            where body_parts.id=? 
            ",
            id
        )
        .fetch_one(self.conn)
        .await?;
        let body_position = BodyPosition::from_str(&record.name)?;
        Ok(body_position)
    }

    async fn find_by_name(&self, name: &String) -> Result<Option<BodyPosition>, MyError> {
        let record = sqlx::query!(
            "select id,name
            from body_parts 
            where body_parts.name=? 
            ",
            name
        )
        .fetch_optional(self.conn)
        .await?;
        if let Some(record) = record {
            let body_position = BodyPosition::from_str(&record.name)?;
            Ok(Some(body_position))
        } else {
            Ok(None)
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     // use actix_web::test;
//     use async_std::test;
//     use chrono::Utc;
//     use serde_yaml;
//     use sqlx::MySqlPool;
//     use std::env;
//     use testfixtures::MySqlLoader;

//     fn load_test_data() {
//         let f = std::fs::File::open("repository/fixtures/train.yaml");
//         let yml = serde_yaml::from_reader(f)?;
//     }

//     #[async_std::test]
//     async fn test_fetch_by_train_id_error() -> Result<(), MyError> {
//         let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
//         let pool_for_query = pool.clone();
//         let loader = MySqlLoader::new(|cfg| {
//             cfg.location(Utc);
//             cfg.database(pool);
//             cfg.skip_test_database_check();
//             cfg.paths(vec!["fixtures/todos.yml"]);
//         })
//         .await?;

//         // load your fixtures
//         loader.load().await.unwrap();
//     }
// }
