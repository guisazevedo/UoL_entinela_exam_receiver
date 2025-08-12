use crate::{HOST, PORT};
use actix_web::{get, HttpResponse};

// Health Check Handler
#[get("/health_check")]
pub async fn health_check_handler() -> HttpResponse {
    HttpResponse::Ok().body(format!("SENTINELA EXAM GATEWAY server is running on {HOST}:{PORT}"))
}

// TESTS *******************************************************************************************
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    // Happy path: correct route + GET
    #[actix_web::test]
    /// Test the health check endpoint with a happy path scenario
    async fn health_check_happy_path() {
        let app = test::init_service(App::new().service(health_check_handler)).await;
        let req = test::TestRequest::get().uri("/health_check").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let text = std::str::from_utf8(&body).unwrap();
        assert!(text.starts_with("SENTINELA EXAM GATEWAY server is running"));
    }

    // Out-of-range / wrong method
    #[actix_web::test]
    /// Test the health check endpoint with a wrong method (POST instead of GET)
    async fn health_check_wrong_method() {
        let app = test::init_service(App::new().service(health_check_handler)).await;
        let req = test::TestRequest::post().uri("/health_check").to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), StatusCode::OK);
    }

    // Error handling: wrong path
    #[actix_web::test]
    /// Test the health check endpoint with a wrong path
    async fn health_check_error_wrong_path() {
        let app = test::init_service(App::new().service(health_check_handler)).await;
        let req = test::TestRequest::get().uri("/wrong_path").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}