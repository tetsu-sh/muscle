mod usecase;
mod domain;
mod repository;
mod presentation;
mod utils;


use actix_web::{get,post,middleware,HttpResponse, web, App, HttpServer,Responder};
use actix_web::web::{get, post};
use presentation::train::create;
use utils::errors::MyError;
use crate::presentation::train::CreateTrainRequest;



#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(api)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}


pub fn api(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
            .service(web::scope("/train").route("", post().to(presentation::train::create)))
            .service(web::scope("/healthcheck").route("", get().to(presentation::healthcheck::index))),
    );
}