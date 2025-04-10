use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/api/v1")]
pub async fn api_v1() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Welcome to API v1",
        "version": "1.0"
    }))
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(api_v1);
}