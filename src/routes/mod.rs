use axum::{routing::get, routing::post, Router};
mod resolve;
mod shortner;
pub fn router(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/shorten_url/", post(shortner::shorten_url))
        .route("/{url_id}", get(resolve::resolve_url))
        .with_state(pool)
}
