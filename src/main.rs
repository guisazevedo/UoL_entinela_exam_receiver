// TODO - create a proper redis -> persistent storage connection
// TODO - logging

// Imports *****************************************************************************************
// External Crates
use actix_web::{App, HttpServer, dev::ServiceRequest, web};

// Internal Modules
mod routes;


// Global variables ********************************************************************************
// Connection Constants
pub const PORT: u16 = 8080;
pub const HOST: &str = "0.0.0.0";


// Main ********************************************************************************************
#[actix_web::main]
async fn main() ->std::io::Result<()> {
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