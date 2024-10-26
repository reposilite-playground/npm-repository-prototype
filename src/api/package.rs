use std::sync::{Arc};
use axum::{extract::{Path, State}, Json, http::StatusCode, response::IntoResponse, Router};
use serde_json::Value;
use crate::AppState;

pub async fn put_package(
    Path(package_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(package_data): Json<Value>,
) -> impl IntoResponse {
    let mut packages = state.packages.lock().unwrap();

    println!("Received publish request for package '{}': {:?}", package_name, package_data);

    packages.insert(package_name.clone(), package_data);

    StatusCode::CREATED
}