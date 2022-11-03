use serde_json::json;

use crate::{
    domain::{
        account::{Account, AccountRepository},
        muscle::Muscle,
        muscle::{BodyPartRepository, BodyPosition, MuscleRepository},
    },
    utils::errors::MyError,
};
use async_trait::async_trait;

use sqlx::MySqlPool;

pub struct AccountRepositoryImpl<'a> {
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
impl AccountRepository for AccountRepositoryImpl<'_> {
    async fn save(&self, account: &Account) -> Result<(), MyError> {
        sqlx::query!(
            "insert into accounts(id,name)
            values(?,?)
            ",
            account.id,
            account.name,
        )
        .execute(self.conn)
        .await?;
        Ok(())
    }

    async fn fetch_one(&self, id: &String) -> Result<Option<Account>, MyError> {
        let record = sqlx::query!(
            "select id,name
            from accounts 
            where accounts.id=? 
            ",
            id
        )
        .fetch_optional(self.conn)
        .await?;
        if let Some(record) = record {
            let account = Account::from(record.id, record.name)?;
            Ok(Some(account))
        } else {
            return Err(MyError::BadRequest(json!({
                "error": format!("no record of id={}.", id)
            })));
        }
    }

    fn find_by_name(&self, name: &String) {
        todo!()
    }
}
