use actix_web::{web, HttpRequest};
use serde_json::json;

use crate::repository::user_repository::UserRepositoryImpl;
use crate::usecase::user::UserUsecase;
use crate::utils::errors::MyError;
use crate::utils::state::AppState;
use crate::{domain::user::User, middleware::authn};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::{convert::From, error::Error};

pub type ApiResponse = Result<HttpResponse, MyError>;

#[derive(Deserialize, Serialize, Debug)]
pub struct SignUpRequest {
    name: String,
    password: String,
    bio: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SignUpResponse {
    id: String,
    token: String,
}

impl SignUpResponse {
    fn from(user: User, token: String) -> Self {
        Self { id: user.id, token }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignInRequest {
    name: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct SignInResponse {
    token: String,
}

impl SignInResponse {
    fn from(token: String) -> Self {
        Self { token }
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
    bio: Option<String>,
}

impl FetchUserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            bio: user.bio,
        }
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
        .sign_up(form.name.clone(), form.bio.clone(), form.password.clone())
        .await?;
    let create_muscle_response = SignUpResponse::from(user, token);
    Ok(HttpResponse::Ok().json(create_muscle_response))
}

pub async fn sign_in(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<SignInRequest>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let user_repository = UserRepositoryImpl { conn: &conn };
    let user_usecase = UserUsecase { user_repository };

    let token = user_usecase
        .sign_in(form.name.clone(), form.password.clone())
        .await?;
    let fetch_user_response = SignInResponse::from(token);

    Ok(HttpResponse::Ok().json(fetch_user_response))
}

pub async fn fetch_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Query<FetchUserParameter>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let user_id = authn::get_user_id_from_header(&req).unwrap();
    let user_repository = UserRepositoryImpl { conn: &conn };
    let user_usecase = UserUsecase { user_repository };

    let user = user_usecase.fetch(&user_id).await?;
    let fetch_user_response = FetchUserResponse::from(user);

    Ok(HttpResponse::Ok().json(fetch_user_response))
}
