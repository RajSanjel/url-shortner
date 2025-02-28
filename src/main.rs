mod db;
mod routes;
const URL_ADDRESS: &str = "0.0.0.0:3000";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = db::connect::db_connection()
        .await
        .expect("Failed to connect to database");
    let app = routes::router(pool);
    let listener = tokio::net::TcpListener::bind(URL_ADDRESS).await.unwrap();
    println!("App is runnin on http://{}", URL_ADDRESS);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
