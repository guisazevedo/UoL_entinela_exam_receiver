// Imports *****************************************************************************************
// External Crates
use anyhow::{Result};
use log::{info};
use serde::Serialize;
use chrono;
use std::collections::HashMap;
use polars::prelude::*;
use polars::io::json::JsonReader;
use polars::io::parquet::{ParquetWriter, ZstdLevel};
use std::io::Cursor;
use google_cloud_storage::client::{Client as GcsClient};
use google_cloud_pubsub::client::{Client as PubSubClient};
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use std::sync::Arc;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use std::borrow::Cow;

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
pub async  fn handle_ecg_exam(
    data: Payload,
    gcs_client: &Arc<GcsClient>,
    pubsub_client: &Arc<PubSubClient>,
) -> Result<()> {

    // STEP 1: Pre-process the data
    let prep_data = preprocess_ecg_data(data)?;

    // STEP 2: Save ECG exam data to persistent storage
    let parquet = prep_data.get("parquet")
        .unwrap() // DOCUMENTATION -> safe given data flow at hashmap creation below
        .clone();
    save_ecg_exam_data(parquet, gcs_client).await?;

    // STEP 3: Send to PubSub for further processing
    let pubsub_data = prep_data.get("pubsub")
        .unwrap() // DOCUMENTATION -> safe given data flow at hashmap creation below
        .clone();
    send_to_pubsub(pubsub_data, pubsub_client).await?;

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
    #[serde(flatten)]
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
        topic: "topic-ecg-dev".to_string(), // TODO -> discuss topic name for dev/prod
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
async fn save_ecg_exam_data(
    data: serde_json::Value,
    gcs_client: &Arc<GcsClient>,
) -> Result<()> {

    // Save ECG exam data to persistent storage as parquet -> GCP Cloud Storage
    // STEP 1: create the unique file name
    let bucket_name = std::env::var("BUCKET_NAME")?;
    // TODO -> define environment variable for bucket name
    let exam_type = "ecg_exam";
    let hospital_id = data.get("hospital_id") // TODO -> review error handling
        .and_then(|v| v.as_str())
        .expect("hospital_id was not set");
    let patient_id = data.get("patient_id")
        .and_then(|v| v.as_str())
        .expect("patient_id was not set");
    let timestamp = data.get("timestamp")
        .and_then(|v| v.as_str())
        .expect("timestamp was not set");
    let object_name = format!(
        "{}/{}/{}/{}.parquet",
        exam_type, hospital_id, patient_id, timestamp
    );

    // STEP 2: Convert the data to Parquet format
    // Convert to json string
    let json = serde_json::to_string(&vec![data])?;
    // read into a polars DataFrame
    let mut df = JsonReader::new(Cursor::new(json))
        .infer_schema_len(None)
        .finish()?;
    // Write the DataFrame to Parquet buffer
    let mut buffer = Vec::new();
    ParquetWriter::new(&mut buffer)
        .with_compression(ParquetCompression::Zstd(Some(ZstdLevel::try_new(1)?)))
        .finish(&mut df)?;



    // STEP 3: Upload the Parquet file to GCP Cloud Storage
    let media = Media::new(Cow::Owned(object_name));
    let upload_type = UploadType::Simple(media);

    gcs_client
        .upload_object(
            &UploadObjectRequest {
                bucket: bucket_name,
                ..Default::default()
            },
            buffer,
            &upload_type,
        )
        .await?;

    Ok(())
}

/// Send the ECG exam data to PubSub for further processing
async fn send_to_pubsub(
    data: serde_json::Value,
    pubsub_client: &Arc<PubSubClient>,
) -> Result<()> {
    // STEP 1: Extract the topic and message from the data
    let topic_name = data.get("topic")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("topic was not set"))?; // TODO -> deal error handling

    // STEP 2: Create the PubSub message as JSON string
    let payload = serde_json::to_string(&data)?;

    // STEP 3: Get the topic and create a publisher
    let topic = pubsub_client.topic(topic_name);
    let publisher = topic.new_publisher(None);

    // STEP 4: Create the PubSub message and publish it
    // let mut message = google_cloud_pubsub::publisher::PubsubMessage::default();
    let mut message = PubsubMessage {
        data: payload.clone().into_bytes(),
        attributes: Default::default(),
        message_id: "".to_string(),
        publish_time: None,
        ordering_key: "".to_string(),
    };
    message.data = payload.into_bytes();
    publisher.publish(message).await.get().await?; // TODO -> review...


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