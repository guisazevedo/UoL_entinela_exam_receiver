// Imports *****************************************************************************************
// External Crates
use anyhow::{anyhow, Result};

// Internal Modules

use actix_web::HttpRequest;

// MAIN FUNCTION ***********************************************************************************
/// Extracts 'hospital_id' and 'hospital_key' from HTTP request headers.
/// # Arguments
/// * `req` - An HttpRequest object containing the headers
/// # Returns
/// * A Result containing a tuple with hospital_id and hospital_key as Strings, or an error
pub fn get_headers(req: HttpRequest) -> Result<(String, String)> {
    let hospital_id = req
        .headers()
        .get("hospital_id")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| anyhow!("Missing values"))?;

    let hospital_key = req
        .headers()
        .get("hospital_key")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| anyhow!("Missing values"))?;

    Ok((hospital_id.to_string(), hospital_key.to_string()))
}

// TESTS *******************************************************************************************
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    // --- Header extraction
    #[tokio::test]
    async fn test_get_headers_valid() {
        let req = TestRequest::default() // TODO:
            .insert_header(("hospital_id", "H123"))
            .insert_header(("hospital_key", "K456"))
            .to_http_request();

        let (id, key) = get_headers(req).unwrap();
        assert_eq!(id, "H123");
        assert_eq!(key, "K456");
    }
}
