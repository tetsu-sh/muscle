use thiserror::Error;
use actix_web::{error::{ResponseError}, HttpResponse, http::StatusCode};
use actix_web::error::ContentTypeError;
use serde_json::Value as JsonValue;
use serde_json::json;

#[derive(Error,Debug)]
pub enum MyError{

    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Not Found")]
    NotFound(JsonValue),
    #[error("Bad Request")]
    BadRequest(JsonValue),

}


impl ResponseError for MyError{

    fn error_response(&self) -> HttpResponse{
        match self{
            MyError::InternalServerError=>HttpResponse::InternalServerError().json("Internal Server Error"),
            MyError::NotFound(ref msg)=>HttpResponse::NotFound().json(msg),
            MyError::BadRequest(ref msg)=>HttpResponse::BadRequest().json(msg)
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
           MyError::InternalServerError=>StatusCode::INTERNAL_SERVER_ERROR, 
           MyError::NotFound(_)=>StatusCode::NOT_FOUND, 
           MyError::BadRequest(_)=>StatusCode::BAD_REQUEST, 
        }
    }
}

impl From<ContentTypeError> for MyError {
    fn from(err:ContentTypeError)->Self{
       match err {
           ContentTypeError::ParseError=>MyError::NotFound(json!({"error":"invalid content type"})),
           ContentTypeError::UnknownEncoding=>MyError::NotFound(json!({"error":"dencode"})),
           _=>MyError::InternalServerError,
       } 
    }
}