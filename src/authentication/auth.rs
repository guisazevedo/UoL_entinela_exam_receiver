// Imports *****************************************************************************************
// External Crates
use actix_web::HttpRequest;
use anyhow::{anyhow, Result};

// Internal Modules
use crate::utils::get_headers::get_headers;

// Services ****************************************************************************************
// TODO
pub fn authenticate_hospital(req: HttpRequest) -> Result<()> {
    // STEP 1: Extract headers
    let (hospital_id, hospital_key) = get_headers(req)?;

    // STEP 2: Validate headers exist
    if hospital_id.is_empty() || hospital_key.is_empty() {
        return Err(anyhow!("Authentication failed: Missing valid headers"));
    }

    // STEP 3: TODO - Validate hospital credentials against database
    Ok(())
}
