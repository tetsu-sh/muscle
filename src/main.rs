mod usecase;
mod domain;


use actix_web::{get,middleware,HttpRequest, web, App, HttpServer,Responder};
use usecase::train::TrainUsecase;


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/train/{name}")]
async fn create_train(name:web::Path<String>){
    TrainUsecase:create_train(name);
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default().log_target("http_log"))
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
