// Imports *****************************************************************************************
// External Crates
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

// Internal Modules

// Constants ***************************************************************************************
pub const ECG_LEAD_LENGTH: usize = 5000; // Length of each ECG lead

// MAIN FUNCTIONS AND STRUCTS **********************************************************************
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


// SUPPORTING FUNCTIONS ****************************************************************************
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
    // Check if the length of the leads is exactly ECG_LEAD_LENGTH samples
    if values.len() != ECG_LEAD_LENGTH {
        return Err(ValidationError::new("Leads must contain exactly ECG_LEAD_LENGTH samples"));
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

// TESTS *******************************************************************************************
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use validator::Validate;

    /// Helper function to create a hex string of a given length
    fn hex_of(len: usize, ch: char) -> String {
        std::iter::repeat(ch).take(len).collect()
    }

    /// Generates a valid SHA256 hash string of 64 characters
    fn valid_id() -> String {
        hex_of(64, 'a')
    }

    /// Generates a vector of f32 with the specified length, initializing the first element
    fn lead_with(len: usize, first: f32) -> Vec<f32> {
        let mut v = vec![0.0; len];
        if len > 0 { v[0] = first; }
        v
    }

    /// Generates a valid ECG lead with a length of ECG_LEAD_LENGTH and a non-flatline value
    fn valid_lead() -> Vec<f32> {
        lead_with(ECG_LEAD_LENGTH, 0.5) // in range, not flatline
    }

    /// Generates a Payload with valid IDs and a lead
    fn payload_with_lead(lead: Vec<f32>) -> Payload {
        Payload {
            patient_id: valid_id(),
            hospital_id: valid_id(),
            hospital_key: valid_id(),
            lead_i: lead.clone(),
            lead_ii: lead.clone(),
            lead_iii: lead.clone(),
            lead_avr: lead.clone(),
            lead_avl: lead.clone(),
            lead_avf: lead.clone(),
            lead_v1: lead.clone(),
            lead_v2: lead.clone(),
            lead_v3: lead.clone(),
            lead_v4: lead.clone(),
            lead_v5: lead.clone(),
            lead_v6: lead,
        }
    }

    // ---------- validate_sha256 ----------
    #[test]
    /// Tests the happy path for SHA256 validation
    fn sha256_happy_path() {
        assert!(validate_sha256(&valid_id()).is_ok());
    }

    #[test]
    /// Tests the out-of-range length for SHA256 validation
    fn sha256_out_of_range_length() {
        assert!(validate_sha256(&hex_of(63, 'a')).is_err());
    }

    #[test]
    /// Tests the borderline case for SHA256 validation with exact length and lower hex
    fn sha256_borderline_ok_exact_len_upper_hex() {
        assert!(validate_sha256(&hex_of(64, 'F')).is_ok());
    }

    #[test]
    /// Tests the borderline case for SHA256 validation with too long length
    fn sha256_error_non_hex() {
        let mut s = valid_id();
        s.replace_range(0..1, "g");
        assert!(validate_sha256(&s).is_err());
    }

    // ---------- validate_ecg_leads ----------
    #[test]
    /// Tests the happy path for ECG leads validation
    fn leads_happy_path() {
        assert!(validate_ecg_leads(&valid_lead()).is_ok());
    }

    #[test]
    /// Tests the out-of-range length for ECG leads validation
    fn leads_out_of_range_length() {
        let v = lead_with(ECG_LEAD_LENGTH - 1, 0.5);
        assert!(validate_ecg_leads(&v).is_err());
    }

    #[test]
    /// Tests the borderline case for ECG leads validation with exact length and lower amplitude
    fn leads_borderline_ok_edge_amplitude() {
        let v = lead_with(ECG_LEAD_LENGTH, 2.0); // exactly at limit
        assert!(validate_ecg_leads(&v).is_ok());
        let v2 = lead_with(ECG_LEAD_LENGTH, -2.0);
        assert!(validate_ecg_leads(&v2).is_ok());
    }

    #[test]
    /// Tests the error case for ECG leads validation with flatline
    fn leads_error_flatline() {
        let v = vec![0.0; ECG_LEAD_LENGTH];
        assert!(validate_ecg_leads(&v).is_err());
    }

    #[test]
    /// Tests the error case for ECG leads validation with amplitude too large
    fn leads_error_amplitude_too_large() {
        let v = lead_with(ECG_LEAD_LENGTH, 2.1);
        assert!(validate_ecg_leads(&v).is_err());
    }

    // ---------- Payload::validate ----------
    #[test]
    /// Tests the happy path for Payload validation
    fn payload_happy_path() {
        let p = payload_with_lead(valid_lead());
        assert!(p.validate().is_ok());
    }

    #[test]
    /// Tests the error case for Payload validation with invalid patient_id
    fn payload_out_of_range_one_lead_length() {
        let mut p = payload_with_lead(valid_lead());
        p.lead_v6 = lead_with(ECG_LEAD_LENGTH - 1, 0.5);
        assert!(p.validate().is_err());
    }

    #[test]
    /// Tests the borderline case for Payload validation with exact length and lower amplitude
    fn payload_borderline_ok_ids_and_edge_leads() {
        let lead = lead_with(ECG_LEAD_LENGTH, 2.0);
        let mut p = payload_with_lead(lead);
        p.patient_id = hex_of(64, 'A'); // uppercase hex, exact len
        p.hospital_id = hex_of(64, '0');
        p.hospital_key = hex_of(64, 'f');
        assert!(p.validate().is_ok());
    }

    #[test]
    /// Tests the error case for Payload validation with invalid patient_id
    fn payload_error_invalid_patient_id() {
        let mut p = payload_with_lead(valid_lead());
        p.patient_id = "not-a-sha256".into();
        assert!(p.validate().is_err());
    }

    #[test]
    /// Tests the error case for Payload validation with flatline in one lead
    fn payload_error_flatline_on_one_lead() {
        let mut p = payload_with_lead(valid_lead());
        p.lead_i = vec![0.0; ECG_LEAD_LENGTH];
        assert!(p.validate().is_err());
    }

    // ---------- serde deny_unknown_fields ----------
    #[test]
    /// Tests that the Payload serde rejects unknown fields
    fn payload_serde_rejects_unknown_fields() {
        let lead = valid_lead();
        let v = json!({
            "patient_id": valid_id(),
            "hospital_id": valid_id(),
            "hospital_key": valid_id(),
            "lead_i": lead, "lead_ii": valid_lead(), "lead_iii": valid_lead(),
            "lead_avr": valid_lead(), "lead_avl": valid_lead(), "lead_avf": valid_lead(),
            "lead_v1": valid_lead(), "lead_v2": valid_lead(), "lead_v3": valid_lead(),
            "lead_v4": valid_lead(), "lead_v5": valid_lead(), "lead_v6": valid_lead(),
            "extra": 123 // should be rejected by #[serde(deny_unknown_fields)]
        });
        let res: Result<Payload, _> = serde_json::from_value(v);
        assert!(res.is_err());
    }
}