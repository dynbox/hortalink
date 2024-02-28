use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Serialize, Serializer};
use crate::app::auth::AuthGate;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum ApiError {
    #[error("Database")]
    Database(String),
    #[error("Unauthorized")]
    Unauthorized(String),
    #[error("Custom")]
    Custom(
        #[serde(serialize_with = "serialize_status_code")]
        StatusCode,
        String
    )
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Custom(code, _) => code
        };

        (status, Json(self)).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        ApiError::Database(format!("{}", error))
    }
}

impl From<axum_login::Error<AuthGate>> for ApiError {
    fn from(value: axum_login::Error<AuthGate>) -> Self {
        ApiError::Database(format!("{}", value))
    }
}

pub fn serialize_status_code<S>(status_code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    serializer.serialize_u16(status_code.as_u16())
}