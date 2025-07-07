// Imports *****************************************************************************************
// External Crates
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

// Internal Modules


// Data Model for the ECG exam *********************************************************************
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
#[serde(deny_unknown_fields)]
/// Data Model for the ECG exam
/// # Arguments
/// * `patient_id` - A string representing the patient id (SHA256 hash)
/// * `hospital_id` - A string representing the hospital id (SHA256 hash)
/// * `hospital_key` - A string representing the hospital key (SHA256 hash)
/// * `lead_i` - A vector of f32 representing the Lead I of the ECG exam
/// * `lead_ii` - A vector of f32 representing the Lead II of the ECG exam
/// * `lead_iii` - A vector of f32 representing the Lead III of the ECG exam
/// * `lead_avr` - A vector of f32 representing the Lead aVR of the ECG exam
/// * `lead_avl` - A vector of f32 representing the Lead aVL of the ECG exam
/// * `lead_avf` - A vector of f32 representing the Lead aVF of the ECG exam
/// * `lead_v1` - A vector of f32 representing the Lead V1 of the ECG exam
/// * `lead_v2` - A vector of f32 representing the Lead V2 of the ECG exam
/// * `lead_v3` - A vector of f32 representing the Lead V3 of the ECG exam
/// * `lead_v4` - A vector of f32 representing the Lead V4 of the ECG exam
/// * `lead_v5` - A vector of f32 representing the Lead V5 of the ECG exam
/// * `lead_v6` - A vector of f32 representing the Lead V6 of the ECG exam
/// # Returns
/// * A Payload struct containing the data of the ECG exam
pub struct Payload {
    // Patient id as a string - SHA256 hash
    #[validate(custom(function = "validate_sha256"))]
    pub patient_id: String,

    // Hospital id as a string - SHA256 hash
    #[validate(custom(function = "validate_sha256"))]
    pub hospital_id: String,

    // Hospital key as a string - SHA256 hash
    #[validate(custom(function = "validate_sha256"))]
    pub hospital_key: String,

    // Lead I should be valid by custom validation function
    #[validate(custom(function = "validate_ecg_leads"))]
    pub lead_i: Vec<f32>,
    // Lead II should be valid by custom validation function
    #[validate(custom(function = "validate_ecg_leads"))]
    pub lead_ii: Vec<f32>,
    // Lead III should be valid by custom validation function
    #[validate(custom(function = "validate_ecg_leads"))]
    pub lead_iii: Vec<f32>,
    // Lead aVR should be valid by custom validation function
    #[validate(custom(function = "validate_ecg_leads"))]
    pub lead_avr: Vec<f32>,
    // Lead aVL should be valid by custom validation function
    #[validate(custom(function = "validate_ecg_leads"))]
    pub lead_avl: Vec<f32>,
    // Lead aVF should be valid by custom validation function
    #[validate(custom(function = "validate_ecg_leads"))]
    pub lead_avf: Vec<f32>,
    // Lead V1 should be valid by custom validation function
    #[validate(custom(function = "validate_ecg_leads"))]
    pub lead_v1: Vec<f32>,
    // Lead V2 should be valid by custom validation function
    #[validate(custom(function = "validate_ecg_leads"))]
    pub lead_v2: Vec<f32>,
    // Lead V3 should be valid by custom validation function
    #[validate(custom(function = "validate_ecg_leads"))]
    pub lead_v3: Vec<f32>,
    // Lead V4 should be valid by custom validation function
    #[validate(custom(function = "validate_ecg_leads"))]
    pub lead_v4: Vec<f32>,
    // Lead V5 should be valid by custom validation function
    #[validate(custom(function = "validate_ecg_leads"))]
    pub lead_v5: Vec<f32>,
    // Lead V6 should be valid by custom validation function
    #[validate(custom(function = "validate_ecg_leads"))]
    pub lead_v6: Vec<f32>,
}


// Supporting Functions ****************************************************************************
/// Custom validation function for SHA256 hash
/// # Arguments
/// * `patient_id` - A string representing the patient id
/// # Returns
/// * A Result containing a unit type or a ValidationError
fn validate_sha256(patient_id: &str) -> Result<(), ValidationError> {
    if patient_id.len() != 64 || !patient_id.chars().all(|c| c.is_ascii_hexdigit()) {
        Err(ValidationError::new("Patient_id must be a valid SHA256 hash"))
    } else {
        Ok(())
    }
}

/// Custom validation function for ECG leads
/// # Arguments
/// * `leads` - A vector of f32 representing the ECG leads
/// # Returns
/// * A Result containing a unit type or a ValidationError
fn validate_ecg_leads(values: &Vec<f32>) -> Result<(), ValidationError> {
    // Check if the length of the leads is exactly 5000 samples
    if values.len() != 5000 {
        return Err(ValidationError::new("Leads must contain exactly 5000 samples"));
    }
    // Check if the values are within the valid range
    let max_amplitude = 2.0;
    if values.iter().any(|&v| v.abs() > max_amplitude) {
        return Err(ValidationError::new("Leads values must be between -2.0 and 2.0"));
    }
    // Check if the patient is not flat-line
    if values.iter().all(|&v| v == 0.0) {
        return Err(ValidationError::new("Leads cannot be flat-line (all values are zero)"));
    }
    Ok(())
    }