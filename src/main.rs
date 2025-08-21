// TODO - create a proper redis -> persistent storage connection for rate limiting total and per user/IP
// TODO - github actions -> GCP
// TODO - virus scanner
// TODO - cxray data transformation

// Imports *****************************************************************************************
// External Crates
use actix_web::{mime, web, App, HttpServer};
use google_cloud_storage::client::{Client as GcsClient, ClientConfig as GcsClientConfig};
use google_cloud_pubsub::client::{Client as PubSubClient, ClientConfig as PubSubClientConfig};
use log::{info};
use dotenv::dotenv;
use std::sync::Arc;


// Internal Modules
mod models;
mod routes;
mod services;
mod utils;


// Global variables ********************************************************************************
// Connection Constants
pub const PORT: u16 = 8080;
pub const HOST: &str = "0.0.0.0";
pub const POST_SIZE_LIMIT: usize = 512_000; // 500 KB


// Main ********************************************************************************************
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Initialize environment variables
    dotenv().ok();

    // Initialize logger
    env_logger::init();
    info!("Starting the ActixWeb server: SENTINELA EXAM GATEWAY");

    // Initialize GCP clients once
    // GCS Client
    let gcs_client = init_gcs_client().await
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    // PubSub Client
    let pubsub_client = init_pubsub_client().await
        .map_err(|e| std::io::Error::other(e.to_string()))?;

    // ActixWeb server initialization
    HttpServer::new(move || {
        println!("Server is running on https://{HOST}:{PORT}");
    App::new()
        .app_data(web::Data::new(gcs_client.clone()))
        .app_data(web::Data::new(pubsub_client.clone()))
        .app_data(web::JsonConfig::default()
            .limit(POST_SIZE_LIMIT)
            .content_type(|mime| {mime == mime::APPLICATION_JSON}))
        .configure(routes::config)
    })
        .workers(num_cpus::get())
        .bind(format!("{HOST}:{PORT}"))?
        .run()
        .await
}

// Support Functions *******************************************************************************

// function to initialize the GCS client
async fn init_gcs_client() -> Result<Arc<GcsClient>, Box<dyn std::error::Error>> {
    let gcs_config = GcsClientConfig::default().with_auth().await?;
    Ok(Arc::new(GcsClient::new(gcs_config)))
}

// function to initialize the PubSub client
async fn init_pubsub_client() -> Result<Arc<PubSubClient>, Box<dyn std::error::Error>> {
    let pubsub_config = PubSubClientConfig::default().with_auth().await?;
    let pubsub_client = PubSubClient::new(pubsub_config).await?;
    Ok(Arc::new(pubsub_client))
}