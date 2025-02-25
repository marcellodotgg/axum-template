use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

pub async fn ping() -> impl IntoResponse {
    (StatusCode::OK, Json(json!(PingResponse::new()))).into_response()
}

#[derive(Serialize)]
pub struct PingResponse {
    message: String,
}

impl PingResponse {
    pub fn new() -> PingResponse {
        PingResponse {
            message: String::from("pong"),
        }
    }
}
