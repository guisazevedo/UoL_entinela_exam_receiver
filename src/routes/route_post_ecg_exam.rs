// Imports *****************************************************************************************
// External Crates
use actix_web::{post, web, HttpResponse, Error};
use serde_json::json;
use validator::Validate;
use log::{error, info};
use std::sync::Arc;

// Internal Modules
use crate::models::model_ecg_exam::Payload;
use crate::services::service_ecg_exam::{handle_ecg_exam};
use google_cloud_storage::client::{Client as GcsClient};
use google_cloud_pubsub::client::{Client as PubSubClient};

// Route Handlers ***********************************************************************************
// Health Check Handler
#[post("/ecg_exam")]
/// Receive and process an ECG exam of a patient
/// # Arguments
/// * `payload` - A JSON object containing the data of the patient
/// # Returns
/// * An HttpResponse containing a 200 OK status if the ECG exam is processed successfully
pub async fn ecg_exam_handler(
    payload: web::Json<Payload>,
    gcs_client: web::Data<Arc<GcsClient>>,
    pubsub_client: web::Data<Arc<PubSubClient>>,
) -> Result<HttpResponse, Error> {
    info!("Starting the route handler for the ECG exam processing");

    // STEP 1: Validate the payload
    if let Err(e) = payload.validate() {
        error!("Validation error - ECG Exam: {}", e);
        return Ok(HttpResponse::BadRequest().json(json!({ "error": "Invalid Input" })));
    }

    // STEP 2: Extract data from payload and process it
    let data = payload.into_inner();
    match handle_ecg_exam(data, &gcs_client, &pubsub_client).await {
        Ok(_) => {
            info!("End of the route handler for the ECG exam processing - Success");
            Ok(HttpResponse::Ok().json(json!({ "status": "ECG Exam Processed Successfully" })))
        },
        Err(e) => {
            error!("Error while processing ECG Exam: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({ "error": "Processing Error" })))
        }
    }
}

// TESTS *******************************************************************************************
