use serde_json::json;
use std::str::FromStr;

use crate::{
    domain::{
        account::{Account, AccountRepository},
        muscle::{BodyPartRepository, BodyPosition, Muscle, MuscleRepository, MuscleSize},
    },
    utils::errors::MyError,
};

pub struct AccountUsecase<A: AccountRepository> {
    pub account_repository: A,
}

impl<A: AccountRepository> AccountUsecase<A> {
    pub fn new(account_repository: A) -> Self {
        Self { account_repository }
    }

    pub async fn create_account(&self, name: String) -> Result<Account, MyError> {
        let account = Account::new(name);
        self.account_repository.save(&account).await?;
        Ok(account)
    }

    pub async fn fetch(&self, id: &String) -> Result<Account, MyError> {
        let account = self.account_repository.fetch_one(id).await?;
        Ok(account.unwrap())
    }
}
