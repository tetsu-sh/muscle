use actix_web::{web, HttpRequest};

use crate::domain::train::Train;
use crate::repository::muscle_repository::MuscleRepositoryImpl;
use crate::repository::train_repository::TrainRepositoryImpl;
use crate::repository::user_repository::UserRepositoryImpl;
use crate::usecase::train::TrainUsecase;
use crate::utils::errors::MyError;
use crate::utils::state::AppState;
use crate::{domain::muscle::Muscle, middleware};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTrainRequest {
    name: String,
    volume: i32,
    rep: i32,
    muscle_ids: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateTrainResponse {
    id: String,
}

impl From<Train> for CreateTrainResponse {
    fn from(train: Train) -> Self {
        Self { id: train.id }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FetchTrainParameter {
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchTrainResponse {
    id: String,
    name: String,
    volume: i32,
    rep: i32,
    muscles: Vec<Muscle>,
}

impl FetchTrainResponse {
    fn from(train: Train) -> Self {
        Self {
            id: train.id,
            name: train.name,
            volume: train.volume,
            rep: train.rep,
            muscles: train.muscles,
        }
    }
}

pub type ApiResponse = Result<HttpResponse, MyError>;

pub async fn create_train(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateTrainRequest>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let user_id = middleware::authn::get_user_id_from_header(&req).unwrap();

    let train_repository = Box::new(TrainRepositoryImpl { conn: &conn });
    let muscle_repository = Box::new(MuscleRepositoryImpl { conn: &conn });

    let train_usecase = TrainUsecase {
        train_repository,
        muscle_repository,
    };
    let train = train_usecase
        .create_train(
            &user_id.to_string(),
            form.name.clone(),
            form.volume,
            form.rep,
            form.muscle_ids.clone(),
        )
        .await?;
    let create_train_response = CreateTrainResponse::from(train);
    Ok(HttpResponse::Ok().json(create_train_response))
}

pub async fn fetch_train(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Query<FetchTrainParameter>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let train_repository = Box::new(TrainRepositoryImpl { conn: &conn });
    let muscle_repository = Box::new(MuscleRepositoryImpl { conn: &conn });
    let train_usecase = TrainUsecase {
        train_repository,
        muscle_repository,
    };
    let train = train_usecase.fetch_one(&params.id).await?;
    let res = FetchTrainResponse::from(train);

    Ok(HttpResponse::Ok().json(res))
}
