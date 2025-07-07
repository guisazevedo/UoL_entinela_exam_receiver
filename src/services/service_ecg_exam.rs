// Imports *****************************************************************************************
// External Crates
use anyhow::{Result};
use log::{info};
// Internal Modules
use crate::models::model_ecg_exam::Payload;


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
    save_ecg_exam_data(&prep_data)?;

    // STEP 3: Send to PubSub for further processing
    send_to_pubsub(&prep_data)?;

    // STEP 4: Return success response
    info!("ECG exam successfully processed");
    Ok(())
}

/// Pre-process the ECG data
fn preprocess_ecg_data(data: Payload) -> Result<Payload> {
    Ok(data)
}

/// Save the ECG exam data to persistent storage
fn save_ecg_exam_data(_data: &Payload) -> Result<()> {
    // Save ECG exam data to persistent storage as parquet -> GCP Cloud Storage
    Ok(())
}

/// Send the ECG exam data to PubSub for further processing
fn send_to_pubsub(_data: &Payload) -> Result<()> {
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