use std::io::Error;

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

use zebra_browser_print::*;

use serde::{Deserialize, Serialize};
#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/default", get(default_device))
        .route("/available", get(available_devices))
        .route("/write", post(write))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9100").await.unwrap();

    println!("Driver started on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn default_device() -> Json<String> {
    Json(String::new())
}

#[derive(Serialize, Debug)]
pub struct AvailableDevice {
    pub printer: Vec<Device>,
    pub other: Vec<Device>,
}

async fn available_devices() -> Result<Json<AvailableDevice>, String> {
    Ok(Json(AvailableDevice {
        other: vec![],
        printer: find_available_devices().map_err(|e| e.to_string())?,
    }))
}

async fn write(body: String) -> Result<(), String> {
    let write: WriteRequest = serde_json::from_str(&body).map_err(|e| e.to_string())?;
    print_label(&write.device.name, &write.data).map_err(|e| e.to_string())?;
    println!("Print job started for {}", write.device.name);
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct WriteRequest {
    pub device: WriteDevice,
    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct WriteDevice {
    pub name: String,
    pub uid: String,
    pub connection: String,
    pub version: u8,
    pub provider: String,
    pub manufacturer: String,
}
