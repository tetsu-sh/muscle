use serde_json::json;

use crate::{
    domain::user::{User, UserRepository},
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
            "insert into users(id,name)
            values(?,?)
            ",
            user.id,
            user.name,
        )
        .execute(self.conn)
        .await?;
        Ok(())
    }

    async fn fetch_one(&self, id: &String) -> Result<User, MyError> {
        let record = sqlx::query!(
            "select id, name, bio, password
            from users 
            where users.id=? 
            ",
            id
        )
        .fetch_optional(self.conn)
        .await?;
        if let Some(record) = record {
            let user = User::from(record.id, record.name, record.bio, record.password)?;
            Ok(user)
        } else {
            return Err(MyError::BadRequest(json!({
                "error": format!("no record of id={}.", id)
            })));
        }
    }

    async fn find_by_name(&self, name: &String) -> Result<User, MyError> {
        let record = sqlx::query!(
            "select id, name, bio, password
            from users
            where users.name=?",
            name
        )
        .fetch_one(self.conn)
        .await?;
        let user = User::from(record.id, record.name, record.bio, record.password)?;
        Ok(user)
    }
}
