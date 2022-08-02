mod usecase;
mod domain;
mod repository;
mod presentation;
mod utils;


use actix_web::{middleware, web, App, HttpServer};
use actix_web::web::{get, post};


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

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