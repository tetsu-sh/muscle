
use actix_web::{web, Responder, HttpRequest};


use serde::{Deserialize,Serialize};
use crate::domain::train::Train;
use crate::utils::errors::MyError;
use crate::{usecase::train::TrainUsecase};
use crate::repository::train_repository::TrainRepositoryImpl;
use std::convert::From;
use actix_web::HttpResponse;

#[derive(Deserialize,Serialize)]
pub struct CreateTrainRequest {
    trainname:String,
    volume:i32,
    rep:i32,
    set:i32,
}

#[derive(Deserialize,Serialize)]
pub struct CreateTrainResponse{
    trainid:String,
}

impl From<Train> for CreateTrainResponse {
   fn from(train: Train) -> Self {
       Self { trainid: train.id.0 }
   } 
}

pub type ApiResponse=Result<HttpResponse,MyError>;


pub async fn create(req:HttpRequest,form:web::Json<CreateTrainRequest>)->ApiResponse{
   let repository=TrainRepositoryImpl{};
   let tu= TrainUsecase{train_repository:repository};
   let train=tu.create_train(form.trainname.clone(),form.volume,form.rep,form.set);
   let res=CreateTrainResponse::from(train);
   Ok(HttpResponse::Ok().json(res))
}