#[macro_use]
extern crate diesel;

mod constants;
mod domain;
mod presentation;
mod repository;
mod schema;
mod usecase;
mod utils;

use actix_web::web::{get, post, Data};
use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer, Result};
use async_graphql::EmptyMutation;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Object, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn fetch(&self) -> i32 {
        40
    }
}

type MuscleSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn index(schema: Data<MuscleSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:8000");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let pool = utils::db::establish_connection();

    let state = crate::utils::state::AppState { pool };

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(api)
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(state.clone()))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
            .service(web::resource("/").guard(guard::Post()).to(index))
    })
    .bind(("127.0.0.1", 8000))?
    .workers(1)
    .run()
    .await
}

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/train")
                .service(web::scope("").route("", get().to(presentation::train::fetch)))
                .service(web::scope("").route("", post().to(presentation::train::create)))
            )
            .service(
                web::scope("/healthcheck").route("", get().to(presentation::healthcheck::index)),
            ),
    );
}
