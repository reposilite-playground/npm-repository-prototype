use std::fmt::format;
use std::path::PathBuf;
use crate::AppState;
use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::{to_string, to_string_pretty, Value};
use std::sync::Arc;
use axum::routing::get;

pub async fn put_package(
    Path(package_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<Value>,
) -> impl IntoResponse {
    let attachments = match payload.get_mut("_attachments") {
        Some(attachments_value) => {
            attachments_value.as_object_mut().ok_or("Expected _attachments to be an object").unwrap()
        },
        None => {
            eprintln!("Missing '_attachments' field in payload");
            return (StatusCode::BAD_REQUEST, "Missing '_attachments' field in payload").into_response();
        }
    };

    // iterate over all tarballs attached
    for (name, value) in attachments {
        let tarball_file_name = name.as_str();

        // extract tarball object
        let tarball_data = match value.get("data") {
            Some(data) => data,
            None => {
                eprintln!("Tarball data is missing");
                return (StatusCode::BAD_REQUEST, "Tarball data is missing").into_response();
            }
        };

        // extract tarball compressed data
        let compressed_data = match tarball_data.as_str() {
            Some(data) => data,
            None => {
                eprintln!("Tarball data is not a string");
                return (StatusCode::BAD_REQUEST, "Tarball data is not a string").into_response();
            }
        };
        
        // save tarball
        state.save_to_file(&PathBuf::from(format!("packages/{}", package_name)), &tarball_file_name, compressed_data).unwrap();
        if let Err(e) = std::fs::write(tarball_file_name, compressed_data) {
            eprintln!("Failed to write tarball file '{}': {}", tarball_file_name, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to write tarball file: {}", e)).into_response();
        }
    }
    
    // remove _attachments field
    payload.as_object_mut().unwrap().remove("_attachments");
    
    // save metadata
    state.save_to_file(&PathBuf::from(format!("packages/{}", package_name)), "metadata.json", &to_string(&payload).unwrap()).unwrap();

    "Package stored successfully".into_response()
}

pub async fn get_package(
    Path(package_name): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match state.load_from_file(&PathBuf::from(format!("packages/{}", package_name)), "metadata.json") {
        Ok(metadata) => {
            Json(metadata).into_response()
        },
        Err(e) => {
            eprintln!("Failed to load package metadata: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to load package metadata: {}", e)).into_response() // This also returns Response<Body>
        }
    }
}

