// Imports *****************************************************************************************
// External Crates
use actix_web::HttpRequest;
use actix_web::{post, web, Error, HttpResponse};
use log::{error, info};
use serde_json::json;
use std::sync::Arc;
use validator::Validate;

// Internal Modules
use crate::authentication::auth::authenticate_hospital;
use crate::models::models_exams::PayloadEcg;
use crate::services::service_ecg_exam::handler_ecg_exam;
use google_cloud_pubsub::client::Client as PubSubClient;
use google_cloud_storage::client::Client as GcsClient;

// Route Handlers ***********************************************************************************
// Health Check Handler
#[post("/ecg_exam")]
/// Receive and process an ECG exam of a patient
/// # Arguments
/// * `payload` - A JSON object containing the data of the patient
/// # Returns
/// * An HttpResponse containing a 200 OK status if the ECG exam is processed successfully
pub async fn ecg_exam_handler(
    req: HttpRequest,
    payload: web::Json<PayloadEcg>,
    gcs_client: web::Data<Arc<GcsClient>>,
    pubsub_client: web::Data<Arc<PubSubClient>>,
) -> Result<HttpResponse, Error> {
    info!("Starting the route handler for the ECG exam processing");

    // Prep: Authenticate hospital
    // If authentication fails, an error response is returned, else processing continues
    if let Err(e) = authenticate_hospital(req) {
        error!("Authentication error - ECG Exam: {}", e);
        return Ok(HttpResponse::Unauthorized().json(json!({ "error": "Unauthorized" })));
    }

    // STEP 1: Validate the payload
    if let Err(e) = payload.validate() {
        error!("Validation error - ECG Exam: {}", e);
        return Ok(HttpResponse::BadRequest().json(json!({ "error": "Invalid Input" })));
    }

    // STEP 2: Extract data from payload, process and log it, then return response
    let data = payload.into_inner();
    match handler_ecg_exam(data, &gcs_client, &pubsub_client).await {
        Ok(_) => {
            info!("End of the route handler for the ECG exam processing - Success");
            Ok(HttpResponse::Ok().json(json!({ "status": "ECG Exam Processed Successfully" })))
        }
        Err(e) => {
            error!("Error while processing ECG Exam: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({ "error": "Processing Error" })))
        }
    }
}

// TESTS *******************************************************************************************
// DOCUMENTATION: Pass function only - no unit tests for route handlers

