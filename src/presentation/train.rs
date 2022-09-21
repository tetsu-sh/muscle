use actix_web::{web, HttpRequest, Responder};

use crate::repository::model;
use crate::repository::train_repository::TrainRepositoryImpl;
use crate::usecase::train::TrainUsecase;
use crate::utils::errors::MyError;
use crate::utils::state::AppState;
use crate::{domain::train::Train, utils::db::DbPool};
use actix_web::HttpResponse;
use log::info;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Deserialize, Serialize,Debug)]
pub struct CreateTrainRequest {
    name: String,
    volume: i32,
    rep: i32,
    set: i32,
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

#[derive(Deserialize, Serialize,Debug)]
pub struct FetchTrainParameter {
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchTrainResponse {
    id: String,
    name: String,
    volume: i32,
    rep: i32,
    set: i32,
}

impl FetchTrainResponse {
    fn from(train: Train) -> Self {
        Self {
            id: train.id,
            name: train.name,
            volume: train.volume,
            rep: train.rep,
            set: train.set,
        }
    }
}

pub type ApiResponse = Result<HttpResponse, MyError>;

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateTrainRequest>,
) -> ApiResponse {
    info!("start create");
    println!("{:?}",form);
    println!("{:?}",req);
    let conn = state.get_conn()?;
    let train_repository = TrainRepositoryImpl { conn: &conn };
    let train_usecase = TrainUsecase { train_repository };
    let train = train_usecase.create_train(form.name.clone(), form.volume, form.rep, form.set)?;
    let create_train_response = CreateTrainResponse::from(train);
    Ok(HttpResponse::Ok().json(create_train_response))
}

pub async fn fetch(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Query<FetchTrainParameter>,
) -> ApiResponse {
    println!("{:?}",params);
    println!("{:?}",req);
    let conn = state.get_conn()?;
    let train_repository = TrainRepositoryImpl { conn: &conn };
    let train_usecase = TrainUsecase { train_repository };
    let train = train_usecase.fetch_one(&params.id)?;
    let res = FetchTrainResponse::from(train);

    Ok(HttpResponse::Ok().json(res))
}
