use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn db_connection() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").map_err(|e| {
        sqlx::Error::Configuration(format!("DATABASE_URL must be set: {}", e).into())
    })?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");
    println!("Connected to the database!");
    Ok(pool)
}
