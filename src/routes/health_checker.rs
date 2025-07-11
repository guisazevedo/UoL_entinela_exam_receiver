use crate::{HOST, PORT};
use actix_web::{get, HttpResponse};

// Health Check Handler
#[get("/health_check")]
pub async fn health_check_handler() -> HttpResponse {
    HttpResponse::Ok().body(format!("SENTINELA EXAM GATEWAY server is running on {HOST}:{PORT}"))
}
