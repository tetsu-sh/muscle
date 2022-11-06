use actix_web::{web, HttpRequest};

use crate::domain::muscle::BodyPosition;
use crate::repository::muscle_repository::MuscleRepositoryImpl;
use crate::usecase::muscle::MuscleUsecase;
use crate::utils::errors::MyError;
use crate::utils::state::AppState;
use crate::{domain::muscle::Muscle, repository::muscle_repository::BodyPartRepositoryImpl};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::convert::From;

pub type ApiResponse = Result<HttpResponse, MyError>;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateMuscleRequest {
    name: String,
    body_part_id: String,
    size: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateMuscleResponse {
    id: String,
}

impl From<Muscle> for CreateMuscleResponse {
    fn from(muscle: Muscle) -> Self {
        Self { id: muscle.id }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FetchMuscleParameter {
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchMuscleResponse {
    id: String,
    name: String,
    position: String,
    size: String,
}

impl FetchMuscleResponse {
    fn from(muscle: Muscle) -> Self {
        Self {
            id: muscle.id,
            name: muscle.name,
            position: muscle.position.to_string(),
            size: muscle.size.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateBodyPartParameter {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateBodyPartResponse {
    id: String,
    body_position: String,
}

impl CreateBodyPartResponse {
    fn from(id: String, body_position: BodyPosition) -> Self {
        Self {
            id,
            body_position: body_position.to_string(),
        }
    }
}

pub async fn create_muscle(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateMuscleRequest>,
) -> ApiResponse {
    // object setting.
    let conn = state.get_sqls_db_conn()?;
    let muscle_repository = MuscleRepositoryImpl { conn: &conn };
    let body_part_repository = BodyPartRepositoryImpl { conn: &conn };
    let muscle_usecase = MuscleUsecase {
        muscle_repository,
        body_part_repository,
    };

    let muscle = muscle_usecase
        .create_muscle(
            form.name.clone(),
            form.body_part_id.clone(),
            form.size.clone(),
        )
        .await?;
    let create_muscle_response = CreateMuscleResponse::from(muscle);
    Ok(HttpResponse::Ok().json(create_muscle_response))
}

pub async fn fetch_muscle(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Query<FetchMuscleParameter>,
) -> ApiResponse {
    // object setting.
    let conn = state.get_sqls_db_conn()?;
    let body_part_repository = BodyPartRepositoryImpl { conn: &conn };
    let muscle_repository = MuscleRepositoryImpl { conn: &conn };
    let muscle_usecase = MuscleUsecase {
        muscle_repository,
        body_part_repository,
    };

    let muscle = muscle_usecase.fetch_muscle(&params.id).await?;
    let fetch_muscle_response = FetchMuscleResponse::from(muscle);
    Ok(HttpResponse::Ok().json(fetch_muscle_response))
}

pub async fn create_body_part(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Json<CreateBodyPartParameter>,
) -> ApiResponse {
    // object setting.
    let conn = state.get_sqls_db_conn()?;
    let body_part_repository = BodyPartRepositoryImpl { conn: &conn };
    let muscle_repository = MuscleRepositoryImpl { conn: &conn };
    let muscle_usecase = MuscleUsecase {
        muscle_repository,
        body_part_repository,
    };

    let (body_part, id) = muscle_usecase.create_body_part(params.name.clone()).await?;
    let create_body_part_response = CreateBodyPartResponse::from(id, body_part);
    Ok(HttpResponse::Ok().json(create_body_part_response))
}
