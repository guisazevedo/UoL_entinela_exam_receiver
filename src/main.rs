// TODO - create a proper redis -> persistent storage connection for rate limiting
// TODO - logging
// TODO - github actions -> GCP
// TODO - main routes -> POST -> exam/cxray
// TODO - models for xray
// TODO - virus scanner
// TODO - data validation for xray
// TODO - data transformation
// TODO - set up "cloud storage" saving & "PubSub" for the results
// TODO - OK 200 response to the client
// TODO - error handling for all steps

// Imports *****************************************************************************************
// External Crates
use actix_web::{App, HttpServer};
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
        .configure(routes::config)
    })
        .workers(num_cpus::get())
        .bind(format!("{HOST}:{PORT}"))?
        .run()
        .await
}