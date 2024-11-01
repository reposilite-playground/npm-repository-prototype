
use std::fs::{create_dir_all, write, File};
use std::io::Read;
use std::path::PathBuf;
use crate::AppState;
use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::{to_string, to_string_pretty, to_vec, Value};
use std::sync::Arc;
use axum::body::Bytes;
use axum::http::{HeaderMap, HeaderValue};
use axum::routing::get;
use axum_macros::debug_handler;
use base64::{decode, encode, Engine};
use base64::alphabet::STANDARD;
use base64::engine::general_purpose;
use sha2::{Digest, Sha512};

pub async fn put_package(
    Path(package_name): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<Value>,
) -> impl IntoResponse {
    println!("payload: {:?}", payload);
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
        let encoded_compressed_data = match tarball_data.as_str() {
            Some(data) => data,
            None => {
                eprintln!("Tarball data is not a string");
                return (StatusCode::BAD_REQUEST, "Tarball data is not a string").into_response();
            }
        };
        
        // decode tarball data with base64
        let compressed_data = match general_purpose::STANDARD.decode(encoded_compressed_data) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to decode tarball data: {}", e);
                return (StatusCode::BAD_REQUEST, "Failed to decode tarball data").into_response();
            }
        };
        
        // save compressed data
        state.save_to_file(&PathBuf::from(format!("packages/{}", package_name)), &tarball_file_name, &*compressed_data).unwrap();
    }

    // remove _attachments field
    payload.as_object_mut().unwrap().remove("_attachments");
    
    // append existing versions
    append_existing_versions(state.clone(), &package_name, &mut payload);

    // save metadata
    state.save_json_to_file(&PathBuf::from(format!("packages/{}", package_name)), "metadata.json", &to_string(&payload).unwrap()).unwrap();

    "Package stored successfully".into_response()
}

fn append_existing_versions(state: Arc<AppState>, package_name: &str, metadata: &mut Value) -> () {
    let mut existing_metadata = state
        .load_json_from_file(&PathBuf::from(format!("packages/{}", package_name)), "metadata.json")
        .unwrap();
    
    // load versions to map from existing metadata (file)
    let versions = existing_metadata
        .get_mut("versions")
        .unwrap()
        .as_object_mut()
        .unwrap();
    
    // put or replace new version to map
    for (version, version_data) in metadata.get_mut("versions").unwrap().as_object_mut().unwrap() {
        versions.insert(version.clone(), version_data.clone());
    }
    
    // replace versions map in new metadata
    metadata.as_object_mut().unwrap().insert("versions".to_string(), Value::from(versions.clone()));
}

pub async fn get_package(
    Path(package_name): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("Received request for package metadata: {}", package_name);
    match state.load_json_from_file(&PathBuf::from(format!("packages/{}", package_name)), "metadata.json") {
        Ok(metadata) => {
            println!("Returning package metadata: {:?}", metadata);
            Json(metadata).into_response()
        },
        Err(e) => {
            eprintln!("Failed to load package metadata: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to load package metadata: {}", e)).into_response()
        }
    }
}

#[debug_handler]
pub async fn get_package_tarball(
    Path((package_name, tarball_name)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("Received request for package tarball: {} - {}", package_name, tarball_name);

    let file_path = PathBuf::from(format!("packages/{}/{}", package_name, tarball_name));

    let mut file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open tarball file: {}", e);
            return Err((StatusCode::NOT_FOUND, "Tarball not found".into()));
        },
    };

    let mut tarball_data = Vec::new();
    if let Err(e) = file.read_to_end(&mut tarball_data) {
        eprintln!("Failed to read tarball file: {}", e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to read tarball file".into()));
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/octet-stream"));

    let body = axum::body::Bytes::from(tarball_data);

    Ok((headers, body).into_response())
}
