use crate::utils::errors::MyError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Account {
    pub id: String,
    pub name: String,
}

impl Account {
    pub fn new(name: String) -> Self {
        let id = Ulid::new().to_string();
        Self { id, name }
    }
    pub fn from(id: String, name: String) -> Result<Account, MyError> {
        let account = Account { id, name };
        Ok(account)
    }
}

#[async_trait]
pub trait AccountRepository {
    /// store Account to DB.
    async fn save(&self, account: &Account) -> Result<(), MyError>;
    /// find one Account from DB by primary key. return Account. if not exist,None.
    async fn fetch_one(&self, id: &String) -> Result<Option<Account>, MyError>;
    fn find_by_name(&self, name: &String);
}
