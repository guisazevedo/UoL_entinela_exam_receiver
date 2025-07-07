// Imports *****************************************************************************************
// External Crates
use actix_web::web;

// Internal Modules
mod health_checker;
pub mod route_post_ecg_exam;

// Router Configuration ****************************************************************************
pub fn config(cfg: &mut web::ServiceConfig) {
    // Register services for the application v1
    cfg.service(
        web::scope("/v1")
            // Health Check
            .service(health_checker::health_check_handler)
            // ECG exam route
            .service(route_post_ecg_exam::ecg_exam_handler)
            // Future Enhancements: Add more routes here
    );
}
