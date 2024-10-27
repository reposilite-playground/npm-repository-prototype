use axum::extract::{Path};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use axum_macros::debug_handler;
use crate::models::login::{LoginRequest, LoginResponse};

#[debug_handler]
pub async fn add_user(
    Path(username): Path<String>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    println!("Received user creation request for user '{}': {:?}", username, payload);

    let response = LoginResponse {
        token: uuid::Uuid::new_v4().to_string(),
        ok: true
    };

    (StatusCode::CREATED, Json(response)).into_response()
}