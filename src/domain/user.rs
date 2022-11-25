use crate::utils::errors::MyError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub bio: Option<String>,
    pub hashed_password: String,
}

impl User {
    pub fn new(name: String, bio: Option<String>, hashed_password: String) -> Self {
        let id = Ulid::new().to_string();
        Self {
            id,
            name,
            bio,
            hashed_password,
        }
    }
    pub fn from(
        id: String,
        name: String,
        bio: Option<String>,
        hashed_password: String,
    ) -> Result<User, MyError> {
        let user = User {
            id,
            name,
            bio,
            hashed_password,
        };
        Ok(user)
    }
}

#[async_trait]
pub trait UserRepository {
    /// store user to DB.
    async fn save(&self, user: &User) -> Result<(), MyError>;
    /// find one user from DB by primary key. return user. if not exist,None.
    async fn fetch_one(&self, id: &String) -> Result<User, MyError>;
    async fn find_by_name(&self, name: &String) -> Result<User, MyError>;
}
