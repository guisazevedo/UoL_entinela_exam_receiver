// Imports *****************************************************************************************
// External Crates
use anyhow::{Result};
use log::{info};
use serde::Serialize;
use chrono;
use std::collections::HashMap;

// Internal Modules
use crate::models::model_ecg_exam::{Payload};

// Services ****************************************************************************************
// Follow service protocol for handling ECG exam data
/// Handles the processing of an ECG exam data from processing to storage and PubSub
/// # Arguments
/// * `data` - A Payload struct containing the validated data of the ECG exam
/// /// # Returns
/// * A Result indicating success or failure of the operation
/// /// # Errors
/// * Returns an error if any step in the processing fails
pub fn handle_ecg_exam(data: Payload) -> Result<()> {

    // STEP 1: Pre-process the data
    let prep_data = preprocess_ecg_data(data)?;

    // STEP 2: Save ECG exam data to persistent storage
    let parquet = prep_data.get("parquet")
        .unwrap() // DOCUMENTATION -> unwrap is safe here as we control the data flow at hashmap creation below
        .clone();
    save_ecg_exam_data(parquet)?;

    // STEP 3: Send to PubSub for further processing
    let pubsub_data = prep_data.get("pubsub")
        .unwrap() // DOCUMENTATION -> unwrap is safe here as we control the data flow at hashmap creation below
        .clone();
    //send_to_pubsub(pubsub_data)?;

    // STEP 4: Return success response
    info!("ECG exam successfully processed");
    Ok(())
}


// Support Functions & Structs *********************************************************************

/// Struct to represent the ECG exam data in a format suitable for Parquet storage
/// # Arguments
/// * `timestamp` - A string representing the timestamp of the ECG exam
/// * data - A Payload struct containing the data of the ECG exam
#[derive(serde::Serialize)]
struct EcgExamParquet {
    exam_type: String,
    timestamp: String,
    data: Payload,
}

/// Struct to represent the ECG exam data in a format suitable for PubSub
/// # Arguments
/// * `topic` - A string representing the PubSub topic
/// * `exam_type` - A string representing the type of the ECG exam
/// * `timestamp` - A string representing the timestamp of the ECG exam
/// * `patient_id` - A string representing the patient id (SHA256 hash)
/// * `hospital_id` - A string representing the hospital id (SHA256 hash)
#[derive(Serialize)]
struct EcgExamPubSub {
    topic: String,
    exam_type: String,
    timestamp: String,
    patient_id: String,
    hospital_id: String,
}

/// Pre-process the ECG data
fn preprocess_ecg_data(data: Payload) -> Result<HashMap<String, serde_json::Value>> {
    // STEP 1: Get name variables
    let utc_timestamp = chrono::Utc::now();
    let utc_timestamp_string = utc_timestamp.format("%Y-%m-%dT%H%M%S%.fZ").to_string();

    // STEP 2: Create the ECG exam data structure for Parquet storage
    let ecg_exam_parquet = EcgExamParquet {
        exam_type: "ECG Exam".to_string(),
        timestamp: utc_timestamp_string.clone(),
        data: data.clone(),
    };

    // STEP 3: Create the ECG exam data structure for PubSub
    let ecg_exam_pubsub = EcgExamPubSub {
        topic: "ecg_exam_topic".to_string(),
        exam_type: "ECG Exam".to_string(),
        timestamp: utc_timestamp_string,
        patient_id: data.patient_id.clone(),
        hospital_id: data.hospital_id.clone(),
    };

    // STEP 4: Convert the structures to HashMap for further processing
    let mut map = HashMap::new();
    map.insert("parquet".to_string(), serde_json::to_value(ecg_exam_parquet)?);
    map.insert("pubsub".to_string(), serde_json::to_value(ecg_exam_pubsub)?);

    Ok(map)
}

/// Save the ECG exam data to persistent storage
fn save_ecg_exam_data(data: serde_json::Value) -> Result<()> {
    // Save ECG exam data to persistent storage as parquet -> GCP Cloud Storage
    Ok(())
}

/// Send the ECG exam data to PubSub for further processing
fn send_to_pubsub(data: serde_json::Value) -> Result<()> {
    // Implement the logic to send data to PubSub for further processing
    Ok(())
}

// TESTS *******************************************************************************************
// DOCUMENTATION -> for each function implement unit tests for:
// * happy path
// * sad path
// * edge cases
#[cfg(test)]
mod tests {

    #[test]
    /// Placeholder
    fn test_preprocess_ecg_data() {
        assert!(true);
    }
}