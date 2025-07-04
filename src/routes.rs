use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index));
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}
