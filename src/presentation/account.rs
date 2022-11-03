use actix_web::{web, HttpRequest};

use crate::domain::account::Account;
use crate::domain::muscle::BodyPosition;
use crate::repository::account_repository::AccountRepositoryImpl;
use crate::repository::muscle_repository::MuscleRepositoryImpl;
use crate::usecase::account::AccountUsecase;
use crate::usecase::muscle::MuscleUsecase;
use crate::utils::errors::MyError;
use crate::utils::state::AppState;
use crate::{domain::muscle::Muscle, repository::muscle_repository::BodyPartRepositoryImpl};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::convert::From;

pub type ApiResponse = Result<HttpResponse, MyError>;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateAccountRequest {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateAccountResponse {
    id: String,
    name: String,
}

impl From<Account> for CreateAccountResponse {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            name: account.name,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FetchAccountParameter {
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchAccountResponse {
    id: String,
    name: String,
}

impl FetchAccountResponse {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            name: account.name,
        }
    }
}

pub async fn create_account(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateAccountRequest>,
) -> ApiResponse {
    println!("{:?}", form);
    println!("{:?}", req);
    let conn = state.get_sqls_db_conn()?;
    let account_repository = AccountRepositoryImpl { conn: &conn };
    let account_usecase = AccountUsecase { account_repository };

    let account = account_usecase.create_account(form.name.clone()).await?;
    let create_muscle_response = CreateAccountResponse::from(account);
    Ok(HttpResponse::Ok().json(create_muscle_response))
}

pub async fn fetch_account(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Query<FetchAccountParameter>,
) -> ApiResponse {
    println!("{:?}", params);
    println!("{:?}", req);
    let conn = state.get_sqls_db_conn()?;
    let account_repository = AccountRepositoryImpl { conn: &conn };
    let account_usecase = AccountUsecase { account_repository };

    let account = account_usecase.fetch(&params.id).await?;
    let fetch_account_response = FetchAccountResponse::from(account);

    Ok(HttpResponse::Ok().json(fetch_account_response))
}
