use axum::extract::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ShortenPayload {
    url: String,
    custom_id: String,
}

pub async fn shorten_url(Json(payload): Json<ShortenPayload>) -> &'static str {
    println! {"Url:{}\nShort Url:{}", payload.url, payload.custom_id};
    "Shortned your url!"
}
