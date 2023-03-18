use crate::utils::errors::MyError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ulid::Ulid;

const NAME_LIMIT: i32 = 30;
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub code: String,
    pub name: String,
    pub hashed_password: String,
}

impl User {
    pub fn new(
        name: String,
        code: Option<String>,
        hashed_password: String,
    ) -> Result<Self, MyError> {
        let id = Ulid::new().to_string();
        if name.chars().count() as i32 > NAME_LIMIT {
            return Err(MyError::BadRequest(
                json!({"error":"train name must be less than 30 letters"}),
            ));
        };
        let code = if let Some(code) = code {
            code
        } else {
            Ulid::new().to_string()
        };
        Ok(Self {
            id,
            code,
            name,
            hashed_password,
        })
    }
    pub fn from(
        id: String,
        code: String,
        name: String,
        hashed_password: String,
    ) -> Result<User, MyError> {
        let user = User {
            id,
            code,
            name,
            hashed_password,
        };
        Ok(user)
    }
}

#[async_trait]
pub trait UserRepository {
    /// store Account to DB.
    async fn save(&self, user: &User) -> Result<(), MyError>;
    /// find one Account from DB by primary key. return Account. if not exist,None.
    async fn fetch_one(&self, id: &String) -> Result<User, MyError>;
    async fn find_by_code(&self, code: &String) -> Result<User, MyError>;
}
