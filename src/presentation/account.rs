use actix_web::{web, HttpRequest};

use crate::domain::user::User;
use crate::repository::user_repository::UserRepositoryImpl;
use crate::usecase::user::UserUsecase;
use crate::utils::errors::MyError;
use crate::utils::state::AppState;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::convert::From;

pub type ApiResponse = Result<HttpResponse, MyError>;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserRequest {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserResponse {
    id: String,
    name: String,
}

impl From<User> for CreateUserResponse {
    fn from(account: User) -> Self {
        Self {
            id: account.id,
            name: account.name,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FetchUserParameter {
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchUserResponse {
    id: String,
    name: String,
}

impl FetchUserResponse {
    fn from(account: User) -> Self {
        Self {
            id: account.id,
            name: account.name,
        }
    }
}

pub async fn create_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateUserRequest>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let account_repository = UserRepositoryImpl { conn: &conn };
    let user_usecase = UserUsecase {
        user_repository: account_repository,
    };

    let user = user_usecase.user_account(form.name.clone()).await?;
    let create_muscle_response = CreateUserResponse::from(user);
    Ok(HttpResponse::Ok().json(create_muscle_response))
}

pub async fn fetch_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Query<FetchUserParameter>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let user_repository = UserRepositoryImpl { conn: &conn };
    let user_usecase = UserUsecase { user_repository };

    let user = user_usecase.fetch(&params.id).await?;
    let fetch_user_response = FetchUserResponse::from(user);

    Ok(HttpResponse::Ok().json(fetch_user_response))
}
