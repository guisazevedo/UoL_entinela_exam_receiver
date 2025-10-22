// Imports *****************************************************************************************
// External Crates
use actix_web::{post, web, Error, HttpResponse};
use log::{error, info};
use serde_json::json;
use std::sync::Arc;
use validator::Validate;

// Internal Modules
use crate::models::models_exams::PayloadXray;
use crate::services::service_xray_exam::handler_xray_exam;
use google_cloud_pubsub::client::Client as PubSubClient;
use google_cloud_storage::client::Client as GcsClient;

// Route Handlers ***********************************************************************************
// Health Check Handler
#[post("/xray_exam")]
/// Receive and process an XRay exam of a patient
/// # Arguments
/// * `payload` - A JSON object containing the data of the patient
/// # Returns
/// * An HttpResponse containing a 200 OK status if the XRay exam is processed successfully
pub async fn xray_exam_handler(
    payload: web::Json<PayloadXray>,
    gcs_client: web::Data<Arc<GcsClient>>,
    pubsub_client: web::Data<Arc<PubSubClient>>,
) -> Result<HttpResponse, Error> {
    info!("Starting the route handler for the Xray exam processing");

    // Prep: Authenticate hospital
    // TODO

    // STEP 1: Validate the payload
    if let Err(e) = payload.validate() {
        error!("Validation error - XRay Exam: {}", e);
        return Ok(HttpResponse::BadRequest().json(json!({ "error": "Invalid Input" })));
    }

    // STEP 2: Extract data from payload, process and log it, then return response
    let data = payload.into_inner();
    match handler_xray_exam(data, &gcs_client, &pubsub_client).await {
        Ok(_) => {
            info!("End of the route handler for the XRay exam processing - Success");
            Ok(HttpResponse::Ok().json(json!({ "status": "Xray Exam Processed Successfully" })))
        }
        Err(e) => {
            error!("Error while processing XRay Exam: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({ "error": "Processing Error" })))
        }
    }
}

// TESTS *******************************************************************************************
// DOCUMENTATION: Pass function only - no unit tests for route handlers

