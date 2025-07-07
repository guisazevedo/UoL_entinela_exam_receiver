// TODO - create a proper redis -> persistent storage connection for rate limiting
// TODO - github actions -> GCP
// TODO - virus scanner
// TODO - cxray main routes -> POST
// TODO - cxray models
// TODO - cxray full data validation
// TODO - cxray data transformation
// TODO - set up "cloud storage" saving & "PubSub" for the results

// Imports *****************************************************************************************
// External Crates
use actix_web::{web, App, HttpServer};
use log::{info};

// Internal Modules
mod models;
mod routes;
mod services;
mod utils;


// Global variables ********************************************************************************
// Connection Constants
pub const PORT: u16 = 8080;
pub const HOST: &str = "0.0.0.0";
pub const SIZE_LIMIT: usize = 512_000; // 500 KB


// Main ********************************************************************************************
#[actix_web::main]
async fn main() ->std::io::Result<()> {
    // Initialize logger
    env_logger::init();
    info!("Starting the ActixWeb server");

    // ActixWeb server initialization
    HttpServer::new(move || {
        println!("Server is running on https://{HOST}:{PORT}");
    App::new()
        .app_data(web::JsonConfig::default().limit(SIZE_LIMIT))
        .configure(routes::config)
    })
        .workers(num_cpus::get())
        .bind(format!("{HOST}:{PORT}"))?
        .run()
        .await
}