use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;

mod api;
mod models;
mod repository;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Healthy".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db = repository::database::Database::new();
    let app_data = web::Data::new(db);

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::config)
            .wrap(middleware::Logger::default())
            .service(healthcheck)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
