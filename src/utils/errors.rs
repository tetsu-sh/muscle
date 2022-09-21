use actix_web::error::ContentTypeError;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use r2d2::Error as R2D2Error;
use serde_json::json;
use serde_json::Value as JsonValue;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MyError {
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Not Found")]
    NotFound(JsonValue),
    #[error("Bad Request")]
    BadRequest(JsonValue),
    #[error("Unprocessable Entity")]
    UnprocessableEntity(JsonValue),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            MyError::UnprocessableEntity(ref msg) => HttpResponse::UnprocessableEntity().json(msg),
            MyError::NotFound(ref msg) => HttpResponse::NotFound().json(msg),
            MyError::BadRequest(ref msg) => HttpResponse::BadRequest().json(msg),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            MyError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,
            MyError::BadRequest(_) => StatusCode::BAD_REQUEST,
            MyError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}

impl From<ContentTypeError> for MyError {
    fn from(err: ContentTypeError) -> Self {
        match err {
            ContentTypeError::ParseError => {
                MyError::NotFound(json!({"error":"invalid content type"}))
            }
            ContentTypeError::UnknownEncoding => MyError::NotFound(json!({"error":"dencode"})),
            _ => MyError::InternalServerError,
        }
    }
}

impl From<DieselError> for MyError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.message();
                    MyError::UnprocessableEntity(json!({ "error": message }))
                } else {
                    println!("error:{}", info.message());
                    MyError::InternalServerError
                }
            }
            DieselError::NotFound => MyError::NotFound(json!({"error":"request record not found"})),
            _ => MyError::InternalServerError,
        }
    }
}

impl From<R2D2Error> for MyError {
    fn from(err: R2D2Error) -> Self {
        MyError::InternalServerError
    }
}
