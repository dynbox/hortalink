use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorMessage {
    message: String
}

pub fn error_message(status_code: StatusCode, message: &str) -> Response {
    (status_code, Json(ErrorMessage { message: message.to_string() })).into_response()
}