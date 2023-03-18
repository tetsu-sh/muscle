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
pub struct SignUpRequest {
    name: String,
    code: Option<String>,
    raw_password: String,
}

#[derive(Deserialize, Serialize)]
pub struct SignUpResponse {
    code: String,
}

impl SignUpResponse {
    fn from(code: String) -> Self {
        Self { code }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignInParameter {
    code: String,
    raw_password: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchUserResponse {
    token: String,
}

impl FetchUserResponse {
    fn from(token: String) -> Self {
        Self { token }
    }
}

pub async fn sign_up(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<SignUpRequest>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let user_repository = UserRepositoryImpl { conn: &conn };
    let user_usecase = UserUsecase { user_repository };

    let (user, token) = user_usecase
        .sign_up(
            form.name.clone(),
            form.code.clone(),
            form.raw_password.clone(),
        )
        .await?;
    let create_muscle_response = SignUpResponse::from(user.code);
    Ok(HttpResponse::Ok().json(create_muscle_response))
}

pub async fn fetch_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Query<SignInParameter>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let user_repository = UserRepositoryImpl { conn: &conn };
    let user_usecase = UserUsecase { user_repository };

    let token = user_usecase
        .sign_in(params.code.clone(), params.raw_password.clone())
        .await?;
    let fetch_user_response = FetchUserResponse::from(token);

    Ok(HttpResponse::Ok().json(fetch_user_response))
}
