use actix_web::{dev::ServiceRequest, HttpMessage, HttpRequest};
use serde_json::json;

use crate::{domain::account::Account, utils::errors::MyError};

pub fn get_current_user(req: &HttpRequest) -> Result<Account, MyError> {
    req.extensions()
        .get::<Account>()
        .map(|account| account.to_owned())
        .ok_or_else(|| MyError::BadRequest(json!({ "error": "authorized error" })))
}

pub fn get_account_id_from_header(req: &HttpRequest) -> Result<&str, &str> {
    // call from presentation layer
    // TODO want set Request intercepter as like middleware.
    req.headers()
        .get("account_id")
        .ok_or("err")
        .and_then(|auth_header| auth_header.to_str().map_err(|_err| "error stringify"))
}
