use actix_web::{web, HttpRequest, Responder};

use crate::domain::train::Train;
use crate::repository::train_repository::TrainRepositoryImpl;
use crate::usecase::train::TrainUsecase;
use crate::utils::errors::MyError;
use actix_web::HttpResponse;
use log::info;
use serde::{Deserialize, Serialize};
use std::convert::From;

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

pub type ApiResponse = Result<HttpResponse, MyError>;

pub async fn create(form: web::Json<CreateTrainRequest>) -> ApiResponse {
    info!("start create");
    let train_repository = TrainRepositoryImpl {};
    let train_usecase = TrainUsecase { train_repository };
    let train = train_usecase.create_train(form.name.clone(), form.volume, form.rep, form.set)?;
    let create_train_response = CreateTrainResponse::from(train);
    Ok(HttpResponse::Ok().json(create_train_response))
}
