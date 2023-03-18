use serde_json::json;

use crate::{
    domain::{
        muscle::Muscle,
        muscle::{BodyPartRepository, BodyPosition, MuscleRepository},
        user::{User, UserRepository},
    },
    utils::errors::MyError,
};
use async_trait::async_trait;

use sqlx::MySqlPool;

pub struct UserRepositoryImpl<'a> {
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
impl UserRepository for UserRepositoryImpl<'_> {
    async fn save(&self, user: &User) -> Result<(), MyError> {
        sqlx::query!(
            "insert into users(id,code,name,password)
            values(?,?,?,?)
            ",
            user.id,
            user.code,
            user.name,
            user.hashed_password,
        )
        .execute(self.conn)
        .await?;
        Ok(())
    }

    async fn fetch_one(&self, id: &String) -> Result<Option<User>, MyError> {
        let record = sqlx::query!(
            "select id,name,code,password
            from users 
            where users.id=? 
            ",
            id
        )
        .fetch_optional(self.conn)
        .await?;
        if let Some(record) = record {
            let user = User::from(record.id, record.name)?;
            Ok(Some(user))
        } else {
            return Err(MyError::BadRequest(json!({
                "error": format!("no record of id={}.", id)
            })));
        }
    }

    async fn find_by_code(&self, code: &String) -> Result<User, MyError> {
        let record = sqlx::query!(
            "select id,name,code, password
            from users
            where users.code=?",
            code
        )
        .fetch_one(self.conn)
        .await?;
        let user = User::from(record.id, record.code, record.name, record.password)?;
        Ok(user)
    }
}
