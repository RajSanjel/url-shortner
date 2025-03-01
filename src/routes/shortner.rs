use crate::db::model::ShortUrl;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    Json as JsonResponse,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate; // Only need Validate

#[derive(Deserialize, Debug, Validate)]
pub struct ShortenPayload {
    #[validate(url(message = "Not a valid URL"))]
    url: Option<String>,
    custom_id: Option<String>,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    short_url: String,
}

fn random_custom_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

pub async fn shorten_url(
    State(pool): State<PgPool>,
    Json(payload): Json<ShortenPayload>,
) -> Result<(StatusCode, JsonResponse<ShortenResponse>), (StatusCode, String)> {
    // Validate the entire payload
    payload
        .validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Safe to unwrap url because validation ensures it's Some and a valid URL
    let url = payload.url.unwrap();
    let custom_id = payload.custom_id.unwrap_or_else(random_custom_id);

    let short_url_row = sqlx::query_as::<_, ShortUrl>(
        "INSERT INTO short_url (url, custom_id) VALUES ($1, $2) RETURNING *",
    )
    .bind(&url)
    .bind(&custom_id)
    .fetch_one(&pool)
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
