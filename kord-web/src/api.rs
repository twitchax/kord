use axum::{extract::Path, http::StatusCode, response::IntoResponse};

pub async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    match super::shared::hello(name).await {
        Ok(greeting) => (StatusCode::OK, greeting),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()),
    }
}