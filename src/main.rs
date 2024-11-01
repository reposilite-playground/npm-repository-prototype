mod api;

use crate::api::package::{get_package, get_package_tarball, put_package};
use crate::api::user::add_user;
use axum::routing::{get, put};
use serde_json::{from_reader, Value};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use axum::Router;

#[derive(Debug, Clone, Default)]
pub struct AppState {
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save_to_file(&self, directory: &PathBuf, filename: &str, data: &[u8]) -> Result<(), String> {
        println!("Attempting to save file in directory: {:?}", directory);
        if let Err(e) = fs::create_dir_all(directory) {
            return Err(format!("Failed to create directory: {:?}", e));
        }

        let file_path = directory.join(filename);
        println!("Full file path resolved to: {:?}", file_path);

        let mut file = File::create(&file_path)
            .map_err(|err| format!("Failed to create file: {}", err))?;
        file.write_all(data)
            .map_err(|err| format!("Failed to write to file: {}", err))?;

        println!("Saved file: {:?}", file_path);

        Ok(())
    }
    
    pub fn load_from_file(&self, directory: &PathBuf, filename: &str) -> Result<Vec<u8>, String> {
        let file_path = directory.join(filename);

        let mut file = File::open(&file_path)
            .map_err(|err| format!("Failed to open file: {}", err))?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .map_err(|err| format!("Failed to read file: {}", err))?;

        Ok(data)
    }
    
    pub fn save_json_to_file(&self, directory: &PathBuf, filename: &str, raw_json: &str) -> Result<(), String> {
        if let Err(e) = fs::create_dir_all(directory) {
            return Err(format!("Failed to create directory: {:?}", e));
        }

        let file_path = directory.join(filename);

        let mut file = File::create(&file_path)
            .map_err(|err| format!("Failed to create file: {}", err))?;
        file.write_all(raw_json.as_bytes())
            .map_err(|err| format!("Failed to write to file: {}", err))?;
        
        println!("Saved file: {:?}", file_path);

        Ok(())
    }

    pub fn load_json_from_file(&self, directory: &PathBuf, filename: &str) -> Result<Value, String> {
        let file_path = directory.join(filename);

        let file = File::open(&file_path)
            .map_err(|err| format!("Failed to open file: {}", err))?;

        let data: Value = from_reader(file)
            .map_err(|err| format!("Failed to deserialize JSON: {}", err))?;

        Ok(data)
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/:package_name", put(put_package))
        .route("/:package_name", get(get_package))
        .route("/:package_name/-/:tarball_name", get(get_package_tarball))
        .route("/-/user/org.couchdb.user:{username}", put(add_user))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
