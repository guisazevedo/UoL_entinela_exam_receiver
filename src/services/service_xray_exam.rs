// Imports *****************************************************************************************
// External Crates
use google_cloud_storage::client::{Client as GcsClient};
use google_cloud_pubsub::client::{Client as PubSubClient};
use std::sync::Arc;
use anyhow::{Result};

// Internal Modules
use crate::models::models_exams::PayloadXray;

// MAIN FUNCTIONS **********************************************************************************
// TODO: Implement the handler for XRay exam processing
pub async fn handler_xray_exam(
    _data: PayloadXray,
    _gcs_client: &Arc<GcsClient>,
    _pubsub_client: &Arc<PubSubClient>,
) -> Result<()> {
    Ok(())
}