use axum::{routing::get, routing::post, Router};
mod resolve;
mod shortner;
pub fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/shorten_url/", post(shortner::shorten_url))
        .route("/resolve_url/{url_id}", get(resolve::resolve_url))
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
