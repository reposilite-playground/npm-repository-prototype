mod models;
mod api;

use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc};
use axum::{Router};
use axum::routing::{get, put};
use serde_json::Value;
use sha2::{Digest, Sha512};
use crate::api::package::put_package;
use crate::api::user::add_user;
use crate::models::package::PackageRequest;

#[derive(Debug, Clone)]
pub struct AppState {
    pub package_dir: Arc<PathBuf>
}

impl AppState {
    pub fn new(directory: PathBuf) -> Self {
        if let Err(e) = fs::create_dir_all(&directory) {
            eprintln!("Failed to create directory: {:?}", e);
        }
        Self {
            package_dir: Arc::new(directory),
        }
    }

    pub fn save_package_to_file(&self, package_name: &str, payload: &PackageRequest) -> Result<(), String> {
        let file_path = self.package_dir.join(format!("{}.json", package_name));

        let json_data = serde_json::to_string_pretty(payload)
            .map_err(|err| format!("Failed to serialize package: {}", err))?;

        let mut file = File::create(&file_path)
            .map_err(|err| format!("Failed to create file: {}", err))?;
        file.write_all(json_data.as_bytes())
            .map_err(|err| format!("Failed to write to file: {}", err))?;

        let mut hasher = Sha512::new();
        hasher.update(&json_data);
        let checksum = hasher.finalize();

        println!("Checksum for package '{}': {:x}", package_name, checksum);

        Ok(())
    }

    fn package_file_path(&self, package_name: &str) -> PathBuf {
        self.package_dir.join(format!("{}.json", package_name))
    }
}

#[tokio::main]
async fn main() {
    let package_dir = PathBuf::from("packages");
    let state = Arc::new(AppState::new(package_dir));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/:package_name", put(put_package))
        .route("/-/user/org.couchdb.user:{username}", put(add_user))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
