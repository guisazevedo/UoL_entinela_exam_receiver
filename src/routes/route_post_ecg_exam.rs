// Imports *****************************************************************************************
// External Crates
use actix_web::{post, web, HttpResponse, Error};
use serde_json::json;
use validator::Validate;
use log::{error, info};

// Internal Modules
use crate::models::model_ecg_exam::Payload;
use crate::services::service_ecg_exam::{preprocess_ecg_data, save_ecg_exam_data, send_to_pubsub};

// Route Handlers ***********************************************************************************
// Health Check Handler
#[post("/ecg_exam")]
/// Receive and process an ECG exam of a patient
/// # Arguments
/// * `payload` - A JSON object containing the data of the patient
/// # Returns
/// * An HttpResponse containing a 200 OK status if the ECG exam is processed successfully
pub async fn ecg_exam_handler(payload: web::Json<Payload>) -> Result<HttpResponse, Error> {
    println!("Starting the route handler for the ECG exam processing");

    // STEP 1: Validate the payload
    if let Err(e) = payload.validate() {
        error!("Validation error - ECG Exam: {}", e);
        return Ok(HttpResponse::BadRequest().json(json!({ "error": "Invalid Input" })));
    }

    // STEP 2: Extract data from payload
    let data = payload.into_inner();

    // STEP 3: Pre-process the data
    let prep_data = match preprocess_ecg_data(&data) {
        Ok(data) => data,
        Err(e) => {
            error!("Error while preprocessing ECG Exam: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({ "error": "Preprocessing Error" })));
        }
    };

    // STEP 4: Save ECG exam data to persistent storage
    if let Err(e) = save_ecg_exam_data(&prep_data) {
        error!("Error while saving ECG Exam data: {}", e);
        return Ok(HttpResponse::InternalServerError().json(json!({ "error": "Storage Error" })));
    }

    // STEP 5: Send to PubSub for further processing
    if let Err(e) = send_to_pubsub(&prep_data) {
        error!("Error while sending ECG Exam data to PubSub: {}", e);
        return Ok(HttpResponse::InternalServerError().json(json!({ "error": "Publishing Error" })));
    }

    // STEP 6: Return success response
    info!("ECG Exam successfully processed");
    Ok(HttpResponse::Ok().json(json!({ "status": "ECG Exam Processed Successfully" })))
}