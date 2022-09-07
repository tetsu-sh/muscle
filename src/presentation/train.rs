use actix_web::{web, HttpRequest, Responder};

use crate::{domain::train::Train, utils::db::DbPool};
use crate::repository::train_repository::TrainRepositoryImpl;
use crate::usecase::train::TrainUsecase;
use crate::utils::errors::MyError;
use actix_web::HttpResponse;
use log::info;
use serde::{Deserialize, Serialize};
use std::convert::From;
use crate::repository::model;

#[derive(Deserialize, Serialize)]
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


pub struct FetchTrainParameter{
    id:i32,
}

pub struct FetchTrainResponse{
    id:i32,
    name:String,
    volume:i32,
    rep:i32,
    set:i32,
}

impl FetchTrainResponse {
    fn from(train: model::TrainRdbModel )->Self{
        Self { id: train.id, name: train.name, volume: train.volume, rep: train.rep, set: train.set }
    }
}

pub type ApiResponse = Result<HttpResponse, MyError>;

pub async fn create(pool:DbPool,form: web::Json<CreateTrainRequest>) -> ApiResponse {
    info!("start create");
    let conn=pool.get()?;
    let train_repository = TrainRepositoryImpl {&conn};
    let train_usecase = TrainUsecase { train_repository };
    let train = train_usecase.create_train(form.name.clone(), form.volume, form.rep, form.set)?;
    let create_train_response = CreateTrainResponse::from(train);
    Ok(HttpResponse::Ok().json(create_train_response))
}

pub async fn fetch(pool:DbPool,params:web::Query<FetchTrainParameter>)->ApiResponse{
    let conn=pool.get()?;
    Ok(HttpResponse::Ok().json())
}