use std::sync::{Arc};
use axum::{extract::{Path, State}, Json, http::StatusCode, response::IntoResponse, Router};
use serde_json::{to_string_pretty, Value};
use crate::AppState;
use crate::models::package::PackageRequest;

pub async fn put_package(
    Path(package_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PackageRequest>,
) -> impl IntoResponse {
    match to_string_pretty(&payload) {
        Ok(json) => println!("Received package creation request for package '{}': {}", package_name, json),
        Err(e) => {
            eprintln!("Failed to serialize payload: {}", e);
            return (StatusCode::BAD_REQUEST, format!("Invalid payload format: {}", e)).into_response();
        }
    }

    match state.save_package_to_file(&package_name, &payload) {
        Ok(_) => "Package stored successfully".into_response(),
        Err(e) => format!("Failed to store package: {}", e).into_response(),
    }
}