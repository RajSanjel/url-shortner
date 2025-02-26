use axum::{
    extract::Path,
    // response::Redirect
};

pub async fn resolve_url(Path(url_id): Path<String>) -> &'static str {
    // Redirect::permanent("http://0.0.0.0:3000/")

    println!("{}", url_id);
    "Testing this.."
}
