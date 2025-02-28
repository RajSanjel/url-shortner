use crate::db::model::ShortUrl;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    Json as JsonResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Debug)]
pub struct ShortenPayload {
    url: String,
    custom_id: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    short_url: String,
}

pub async fn shorten_url(
    State(pool): State<PgPool>,
    Json(payload): Json<ShortenPayload>,
) -> Result<(StatusCode, JsonResponse<ShortenResponse>), (StatusCode, String)> {
    let short_url_row = sqlx::query_as::<_, ShortUrl>(
        "INSERT INTO short_url (url, custom_id) VALUES ($1, $2) RETURNING *",
    )
    .bind(&payload.url)
    .bind(&payload.custom_id)
    .fetch_one(&pool) // Changed to .fetch_one()
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.constraint() == Some("short_url_custom_id_key") {
                return (
                    StatusCode::BAD_REQUEST,
                    "Custom ID already exists".to_string(),
                );
            }
        }
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    let short_url = format!("http://localhost:3000/{}", &short_url_row.custom_id);
    Ok((
        StatusCode::CREATED,
        JsonResponse(ShortenResponse { short_url }),
    ))
}
