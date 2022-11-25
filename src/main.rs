mod constants;
mod domain;
mod middleware;
mod presentation;
mod repository;
mod usecase;
mod utils;

use actix_web::middleware::Logger;
use actix_web::web::{get, post, Data};
use actix_web::{web, App, HttpServer};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let pool = utils::db::establish_sqlx_connection().await;
    let app_state = utils::state::AppState { sqlx_db: pool };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(api)
            .app_data(Data::new(app_state.clone()))
        // .wrap(middleware::sayhi::SayHi)
    })
    .bind(("127.0.0.1", 8000))?
    .workers(1)
    .run()
    .await
}

// api route definition
pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/train")
                    .route("", get().to(presentation::train::fetch_train))
                    .route("", post().to(presentation::train::create_train)),
            )
            .service(
                web::scope("/user")
                    .route("login", post().to(presentation::user::sign_in))
                    .route("", post().to(presentation::user::sign_up)),
            )
            .service(
                web::scope("/muscle")
                    .route("", get().to(presentation::muscle::fetch_muscle))
                    .route("", post().to(presentation::muscle::create_muscle))
                    .service(
                        web::scope("/body_part")
                            .route("", post().to(presentation::muscle::create_body_part)),
                    ),
            )
            .service(
                web::scope("/healthcheck").route("", get().to(presentation::healthcheck::index)),
            ),
    );
}
