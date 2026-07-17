use axum::{
    Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

use zebra_browser_print::*;

use serde::{Deserialize, Serialize};
use zpl_toolchain_print_client::Printer;

struct HttpError(anyhow::Error);
type Result<T> = core::result::Result<T, HttpError>;

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        eprintln!("Req error: {}", self.0);
        (
            hyper::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<T: Into<anyhow::Error>> From<T> for HttpError {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

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

async fn available_devices() -> Result<Json<AvailableDevice>> {
    Ok(Json(AvailableDevice {
        other: vec![],
        printer: Device::list(),
    }))
}

async fn write(body: String) -> Result<()> {
    let write: WriteRequest = serde_json::from_str(&body)?;
    let mut printer = write.device.connect()?;
    println!("Print job started for {}", write.device.name);
    printer.send_zpl(&write.data)?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct WriteRequest {
    pub device: Device,
    pub data: String,
}
