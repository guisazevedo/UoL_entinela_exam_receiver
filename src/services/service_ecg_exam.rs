// Imports *****************************************************************************************
// External Crates
use anyhow::Result;
use chrono;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::Client as PubSubClient;
use google_cloud_storage::client::Client as GcsClient;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use log::info;
use polars::io::json::JsonReader;
use polars::io::parquet::{ParquetWriter, ZstdLevel};
use polars::prelude::*;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;

// Internal Modules
use crate::models::models_exams::PayloadEcg;

// Services ****************************************************************************************
// Follow service protocol for handling ECG exam data
/// Handles the processing of an ECG exam data from processing to storage and PubSub
/// # Arguments
/// * `data` - A Payload struct containing the validated data of the ECG exam
/// * `gcs_client` - An Arc reference to the GCS client for storage operations
/// * `pubsub_client` - An Arc reference to the PubSub client for message publishing
/// # Returns
/// * A Result indicating success or failure of the operation
/// # Errors
/// * Returns an error if any step in the processing fails
pub async fn handler_ecg_exam(
    data: PayloadEcg,
    gcs_client: &Arc<GcsClient>,
    pubsub_client: &Arc<PubSubClient>,
) -> Result<()> {
    info!("Handling ECG payload - pre-processing the data");
    // STEP 1: Pre-process the data
    let prep_data = preprocess_ecg_data(data.clone())?;

    // STEP 2: Save ECG exam data to persistent storage
    let parquet = prep_data
        .get("parquet")
        .ok_or_else(|| anyhow::anyhow!("Missing 'parquet' entry in prep_data"))?
        .clone();
    save_ecg_exam_data(parquet, gcs_client).await?;

    info!("Handling ECG payload - pre-processing the data - done - parquet saved");

    // STEP 3: Send to PubSub for further processing
    let pubsub_data = prep_data
        .get("pubsub")
        .ok_or_else(|| anyhow::anyhow!("Missing 'pubsub' entry in prep_data"))?
        .clone();
    send_to_pubsub(pubsub_data.clone(), pubsub_client).await?;

    info!("Handling ECG payload - pre-processing the data - done - parquet saved - pubsub sent");

    // STEP 4: Return success response
    info!("ECG exam successfully processed");
    Ok(())
}

// Support Functions & Structs *********************************************************************
/// Struct to represent the ECG exam data in a format suitable for Parquet storage
/// # Arguments
/// * `timestamp` - A string representing the timestamp of the ECG exam
/// * data - A Payload struct containing the data of the ECG exam
#[derive(serde::Serialize, Debug)]
struct EcgExamParquet {
    exam_type: String,
    timestamp: String,
    #[serde(flatten)]
    data: PayloadEcg,
}

/// Struct to represent the ECG exam data in a format suitable for PubSub
/// # Arguments
/// * `topic` - A string representing the PubSub topic
/// * `exam_type` - A string representing the type of the ECG exam
/// * `timestamp` - A string representing the timestamp of the ECG exam
/// * `patient_id` - A string representing the patient id (SHA256 hash)
/// * `hospital_id` - A string representing the hospital id (SHA256 hash)
#[derive(Serialize, Debug)]
struct EcgExamPubSub {
    topic: String,
    exam_type: String,
    timestamp: String,
    patient_id: String,
    hospital_id: String,
}

/// Pre-process the ECG data for storage and PubSub
/// # Arguments
/// * `data` - A Payload struct containing the validated data of the ECG exam
/// # Returns
/// * A HashMap containing two entries: one for Parquet storage and one for PubSub
/// # Errors
/// * Returns an error if serialization fails
fn preprocess_ecg_data(data: PayloadEcg) -> Result<HashMap<String, serde_json::Value>> {
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
        topic: "topic-ecg-dev".to_string(), // TODO: after PoC: discuss name for dev/prod
        exam_type: "ECG Exam".to_string(),
        timestamp: utc_timestamp_string,
        patient_id: data.patient_id.clone(),
        hospital_id: data.hospital_id.clone(),
    };

    // STEP 4: Convert the structures to HashMap for further processing
    let mut map = HashMap::new();
    map.insert(
        "parquet".to_string(),
        serde_json::to_value(ecg_exam_parquet)?,
    );
    map.insert("pubsub".to_string(), serde_json::to_value(ecg_exam_pubsub)?);

    Ok(map)
}

/// Save the ECG exam data to persistent storage as Parquet in GCP Cloud Storage
/// # Arguments
/// * `data` - A serde_json::Value containing the ECG exam data for Parquet storage
/// * `gcs_client` - An Arc reference to the GCS client for storage operations
/// # Returns
/// * A Result indicating success or failure of the operation
/// # Errors
/// * Returns an error if any step in the saving process fails
async fn save_ecg_exam_data(data: serde_json::Value, gcs_client: &Arc<GcsClient>) -> Result<()> {
    // Save ECG exam data to persistent storage as parquet -> GCP Cloud Storage
    // STEP 1: create the unique file name
    let bucket_name = std::env::var("BUCKET_NAME")?;
    let exam_type = "ecg_exam";
    let hospital_id = data
        .get("hospital_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("hospital_id was not set"))?;
    let patient_id = data
        .get("patient_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("patient_id was not set"))?;
    let timestamp = data
        .get("timestamp")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("timestamp was not set"))?;
    let object_name = format!("{exam_type}/{hospital_id}/{patient_id}/{timestamp}.parquet");

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
    let media = Media::new(Cow::Owned(object_name.clone()));
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
/// # Arguments
/// * `data` - A serde_json::Value containing the ECG exam data for PubSub
/// * `pubsub_client` - An Arc reference to the PubSub client for message publishing
/// # Returns
/// * A Result indicating success or failure of the operation
/// # Errors
/// * Returns an error if any step in the sending process fails
async fn send_to_pubsub(data: serde_json::Value, pubsub_client: &Arc<PubSubClient>) -> Result<()> {
    // STEP 1: Extract the topic and message from the data
    let topic_name = data
        .get("topic")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("topic was not set"))?;

    // STEP 2: Create the PubSub message as JSON string
    let payload = serde_json::to_string(&data)?;

    // STEP 3: Get the topic and create a publisher
    let topic = pubsub_client.topic(topic_name);
    let publisher = topic.new_publisher(None);

    // STEP 4: Create the PubSub message and publish it
    // let mut message = google_cloud_pubsub::publisher::PubsubMessage::default();
    let message = PubsubMessage {
        data: payload.clone().into_bytes(),
        attributes: Default::default(),
        message_id: "".to_string(),
        publish_time: None,
        ordering_key: "".to_string(),
    };

    // DEBUG
    println!("PubSub message payload: {}", payload);

    // STEP 5: Publish the message
    publisher.publish(message).await;

    // DEBUG
    println!("Published to PubSub topic: {}", topic_name);

    Ok(())
}

// TESTS *******************************************************************************************
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::models_exams::{PayloadEcg, ECG_LEAD_LENGTH};
    use validator::Validate;

    fn hex64(c: char) -> String {
        std::iter::repeat(c).take(64).collect()
    }
    fn lead_ok() -> Vec<f32> {
        let mut v = vec![0.0; ECG_LEAD_LENGTH];
        v[0] = 0.5;
        v
    }
    fn valid_payload() -> PayloadEcg {
        PayloadEcg {
            patient_id: hex64('a'),
            hospital_id: hex64('b'),
            hospital_key: hex64('c'),
            lead_i: lead_ok(),
            lead_ii: lead_ok(),
            lead_iii: lead_ok(),
            lead_avr: lead_ok(),
            lead_avl: lead_ok(),
            lead_avf: lead_ok(),
            lead_v1: lead_ok(),
            lead_v2: lead_ok(),
            lead_v3: lead_ok(),
            lead_v4: lead_ok(),
            lead_v5: lead_ok(),
            lead_v6: lead_ok(),
        }
    }

    // Happy path: returns both entries with expected fields
    #[test]
    fn preprocess_happy_path() {
        let p = valid_payload();
        assert!(p.validate().is_ok());

        let map = preprocess_ecg_data(p.clone()).expect("preprocess ok");
        assert!(map.contains_key("parquet"));
        assert!(map.contains_key("pubsub"));

        let parquet = map.get("parquet").unwrap();
        assert_eq!(
            parquet.get("exam_type").unwrap().as_str().unwrap(),
            "ECG Exam"
        );
        let ts = parquet.get("timestamp").unwrap().as_str().unwrap();
        assert!(ts.ends_with('Z'));

        // payload flattened under parquet
        assert_eq!(
            parquet.get("patient_id").unwrap().as_str().unwrap(),
            p.patient_id
        );
        assert_eq!(
            parquet.get("hospital_id").unwrap().as_str().unwrap(),
            p.hospital_id
        );

        let pubsub = map.get("pubsub").unwrap();
        assert_eq!(
            pubsub.get("topic").unwrap().as_str().unwrap(),
            "topic-ecg-dev"
        );
        assert_eq!(
            pubsub.get("exam_type").unwrap().as_str().unwrap(),
            "ECG Exam"
        );
        assert_eq!(
            pubsub.get("patient_id").unwrap().as_str().unwrap(),
            p.patient_id
        );
        assert_eq!(
            pubsub.get("hospital_id").unwrap().as_str().unwrap(),
            p.hospital_id
        );
    }

    // Borderline‑ok: timestamp format parses with your custom fmt
    #[test]
    fn preprocess_timestamp_format() {
        let map = preprocess_ecg_data(valid_payload()).unwrap();
        let ts = map
            .get("parquet")
            .unwrap()
            .get("timestamp")
            .unwrap()
            .as_str()
            .unwrap();
        assert!(ts.ends_with('Z'));
        let ts_no_z = &ts[..ts.len() - 1];
        let fmt = "%Y-%m-%dT%H%M%S%.f";
        chrono::NaiveDateTime::parse_from_str(ts_no_z, fmt).expect("timestamp matches custom fmt");
    }
}
