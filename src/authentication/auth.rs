// Imports *****************************************************************************************
// External Crates
use actix_web::HttpRequest;
use anyhow::{anyhow, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};

// Internal Modules
use crate::utils::get_headers::get_headers;

// MAIN FUNCTION ***********************************************************************************
/// Authenticate hospital based on headers in the HTTP request
/// # Arguments
/// * `req` - The HTTP request containing headers for authentication
/// # Returns
/// * `Result<()>` - Ok(()) if authentication is successful, Err otherwise
pub async fn authenticate_hospital(req: HttpRequest) -> Result<()> {
    // STEP 1: Extract headers
    let (hospital_id, hospital_key) = get_headers(req)?;

    // STEP 2: Validate headers exist
    if hospital_id.is_empty() || hospital_key.is_empty() {
        return Err(anyhow!("Authentication failed: Missing valid headers"));
    }

    // STEP 3: Validate hospital credentials against database // TODO: check GCP connection
    let pool = connect_to_database().await?;
    validate_hospital_credentials(&hospital_id, &hospital_key, &pool).await?;

    // If all checks pass, return Ok
    Ok(())
}

// SUPPORTING FUNCTIONS ****************************************************************************authenticate_hospital

/// Function to connect to the database and return a connection object (pool)
/// # Returns
/// * `Result<Pool<Postgres>>` - A connection pool to the Postgres database
async fn connect_to_database() -> Result<Pool<Postgres>> {
    // STEP 1: Get database connection parameters from environment variables
    let db_user = std::env::var("DB_USER")?;
    let db_pass = std::env::var("DB_PASSWORD")?;
    let db_host = std::env::var("DB_HOST")?;
    let db_port = std::env::var("DB_PORT")?;
    let db_name = std::env::var("DB_NAME")?;

    // STEP 2: Create the database connection string
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_pass, db_host, db_port, db_name
    );

    // STEP 3: Create the connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

/// Function to validate hospital credentials against the database
/// # Arguments
/// * `hospital_id` - The ID of the hospital to validate
/// * `hospital_key` - The key of the hospital to validate
/// * `pool` - The database connection pool
/// # Returns
/// * `Result<()>` - Ok(()) if credentials are valid, Err otherwise
async fn validate_hospital_credentials(
    hospital_id: &str,
    hospital_key: &str,
    pool: &Pool<Postgres>,
) -> Result<()> {
    // STEP 1: Query the database for hospital credentials
    let row = sqlx::query(
        r#"
        SELECT COUNT(*) as checker 
        FROM hospital_credentials
        WHERE hospital_id = $1 AND hospital_key = $2
        "#,
    )
    .bind(hospital_id)
    .bind(hospital_key)
    .fetch_one(pool)
    .await?;

    // STEP 2: Check if credentials are valid
    let checker: i32 = row.try_get("checker")?;
    if checker == 0 {
        return Err(anyhow!("Authentication failed: Invalid credentials"));
    }

    Ok(())
}

// TESTS *******************************************************************************************
// TODO:
