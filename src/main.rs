mod routes;

const URL_ADDRESS: &str = "0.0.0.0:3000";
#[tokio::main]
async fn main() {
    let app = routes::router();
    let listener = tokio::net::TcpListener::bind(URL_ADDRESS).await.unwrap();
    println!("App is runnin on http://{}", URL_ADDRESS);
    axum::serve(listener, app).await.unwrap();
}
