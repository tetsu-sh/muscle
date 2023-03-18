use crate::middleware::authn::make_jwt;
use crate::utils::hash::{hash_password, verify};
use crate::utils::password::verify_user_password;
use serde_json::json;
use std::str::FromStr;

use crate::{
    domain::{
        muscle::{BodyPartRepository, BodyPosition, Muscle, MuscleRepository, MuscleSize},
        user::{User, UserRepository},
    },
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
        code: Option<String>,
        raw_password: String,
    ) -> Result<(User, String), MyError> {
        verify_user_password(&raw_password);
        let hashed_password = hash_password(&raw_password)?;
        let user = User::new(name, code, hashed_password)?;
        self.user_repository.save(&user).await?;
        let token = make_jwt(&user.id)?;

        Ok((user, token))
    }

    pub async fn sign_in(&self, code: String, raw_password: String) -> Result<String, MyError> {
        let user = self.user_repository.find_by_code(&code).await?;
        let _ = verify(&raw_password, &user.hashed_password);
        let token = make_jwt(&user.id)?;
        Ok(token)
    }
}
