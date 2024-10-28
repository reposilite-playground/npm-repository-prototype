use std::panic::resume_unwind;
use axum::extract::{Path};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use axum_macros::debug_handler;
use serde_json::Value;

#[debug_handler]
pub async fn add_user(
    Path(username): Path<String>,
    Json(payload): Json<Value>,
) -> Response {
    println!("Received user creation request for user '{}': {:?}", username, payload);

    let response = serde_json::json!({
        "ok": true,
        "token": uuid::Uuid::new_v4().to_string()
    });

    (StatusCode::CREATED, Json(response)).into_response()
}