use crate::middleware::authn::make_jwt;
use crate::utils::hash::{hash_password, verify};
use serde_json::json;
use std::str::FromStr;

use crate::{
    domain::user::{User, UserRepository},
    utils::errors::MyError,
};

pub struct UserUsecase<A: UserRepository> {
    pub user_repository: A,
}

impl<A: UserRepository> UserUsecase<A> {
    pub fn new(user_repository: A) -> Self {
        Self { user_repository }
    }

    pub async fn sign_up(
        &self,
        name: String,
        bio: Option<String>,
        raw_password: String,
    ) -> Result<(User, String), MyError> {
        self.verify_password(&raw_password);
        let hashed_password = hash_password(&raw_password).unwrap();
        let user = User::new(name, bio, hashed_password);
        self.user_repository.save(&user).await?;
        let token = make_jwt(&user.id)?;

        Ok((user, token))
    }

    fn verify_password(&self, raw_password: &str) -> Result<(), MyError> {
        if raw_password.len() < 8 {
            return Err(MyError::BadRequest(json!({"error":"password too short"})));
        }
        Ok(())
    }

    pub async fn sign_in(&self, name: String, raw_password: String) -> Result<String, MyError> {
        let user = self.user_repository.find_by_name(&name).await?;
        let _ = verify(&raw_password, &user.hashed_password);
        let token = make_jwt(&user.id)?;
        Ok(token)
    }

    pub async fn fetch(&self, id: &String) -> Result<User, MyError> {
        let user = self.user_repository.fetch_one(id).await?;
        Ok(user)
    }
}
