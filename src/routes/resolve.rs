use crate::db::model::ShortUrl;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
};
use sqlx::PgPool;

pub async fn resolve_url(
    Path(url_id): Path<String>,
    State(pool): State<PgPool>,
) -> Result<Redirect, (StatusCode, String)> {
    let redirect_url = sqlx::query_as::<_, ShortUrl>(
        "SELECT id, url, custom_id, created_at FROM short_url WHERE custom_id = $1",
    )
    .bind(&url_id)
    .fetch_optional(&pool) // Changed to fetch_optional
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Check if a row was found
    let redirect_url = match redirect_url {
        Some(url) => url,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                "URL not found for the given ID".to_string(),
            ))
        }
    };

    if redirect_url.url.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "URL is empty, recheck if it's correct".to_string(),
        ));
    }

    Ok(Redirect::permanent(&redirect_url.url))
}
