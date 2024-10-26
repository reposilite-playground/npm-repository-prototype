mod models;
mod api;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::{extract::{Path, State}, Json, http::StatusCode, response::IntoResponse, Router};
use axum::routing::{get, put};
use serde_json::Value;
use crate::api::package::put_package;

#[derive(Debug, Clone)]
pub struct AppState {
    pub packages: Arc<Mutex<HashMap<String, Value>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            packages: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/packages/:package_name", put(put_package))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
