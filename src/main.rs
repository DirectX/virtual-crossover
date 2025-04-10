use actix_web::{get, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use serde_json::json;

mod api;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Welcome to the server",
        "endpoints": {
            "api_v1": "/api/v1"
        }
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("Server starting at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .configure(api::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
