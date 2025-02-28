use crate::db::model::ShortUrl;
use axum::{
    extract::{Path, State},
    response::Redirect,
};
use sqlx::PgPool;

pub async fn resolve_url(
    Path(url_id): Path<String>,
    State(pool): State<PgPool>,
) -> Result<Redirect, String> {
    let short_url = sqlx::query_as::<_, ShortUrl>(
        "SELECT id, url, custom_id, created_at FROM short_url WHERE custom_id = $1",
    )
    .bind(&url_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Redirect::permanent(&short_url.url))
}
